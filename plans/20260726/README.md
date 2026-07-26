# 并发与限速语义重构

日期：2026-07-26

## 问题

`RequestGate`（`src/crawler.rs`，已移除）用一把全局 `Mutex` 串行化所有请求发起：

```rust
let mut next = self.next.lock().await;
if *next > now { sleep_until(*next).await; }
*next = Instant::now() + self.interval;
```

- `interval` = 全局发起间隔（相邻两个请求的启动间隔），MUST NOT 与并发数正交
- 实际在途请求数 ≈ `min(concurrency, latency / interval)`，`concurrency` 基本失效
- 实测口径：`concurrency = 3000, interval_ms = 300` -> 吞吐 3.3 req/s、在途 ≈ 1

## 决策

- `concurrency` = 下载槽位数；队列有货时始终打满，在途请求数 = 槽位数
- `interval` = 单槽位完成一次请求后的休息时长；只影响该槽位，MUST NOT 影响其他槽位
- 休息在「槽位取到下一个 URL 之后、发起请求之前」执行 -> 休息中不占用网络、`Started` 事件仍等于真实在途
- 上限 `MAX_CONCURRENCY = 64`；配置/参数超限收敛到 64 + stderr warning（旧默认 1000 与自用 3000 在新语义下 = socket 耗尽 + 远端 429）
- 默认值 `concurrency = 8, interval_ms = 100`（8 = 通用单域名并发惯例，对齐 Scrapy `CONCURRENT_REQUESTS_PER_DOMAIN`）
- resume 命中本地文件不占槽位、不触发休息 —— 无网络请求

## 实现

`crawl()` 主循环持有 `VecDeque<Instant>` 槽位就绪时刻，无共享锁：

```text
task 完成(finished_at) -> 主循环 push finished_at + interval
填充槽位 -> pop_front() 得到 start_at -> spawn { sleep_until(start_at); Started; fetch }
interval == 0 -> 不入队, 零 await 开销
```

- `finished_at` 由 task 自身返回 -> 休息从「请求真正结束」起算，MUST NOT 把主循环处理耗时算进休息
- 单一 spawn 点在主循环（单 task），槽位队列无需 `Mutex`
- 吞吐 = `concurrency / (latency + interval)`

## 进度显示修正

旧 `inflight` = `Started` 减去各分类完成事件，分类由主循环发出 -> 主循环处理积压时虚高，实测出现 `inflight=9/8`。

- 新增 `CrawlEvent::Finished`，与 `Started` 在 task 内夹住 `fetch` -> `inflight = started - finished` 为精确在途数
- 分类事件退化为纯计数，MUST NOT 参与在途推导
- 进度行改为 `inflight=<在途>/<槽位数>`；聚合链路只有 1 个请求 -> 分母固定 1

## 验收

- `saturates_every_download_slot`：4 槽位 + 8 个延迟文档 -> 服务端 `max_active == 4`
- `interval_rests_only_the_slot_that_finished`：3 槽位 + `interval = 400ms` + `latency = 150ms` -> 相邻两个发起间隔 < 200ms 且 `max_active >= 2`（旧实现每个发起相隔 400ms）
- `interval_delays_the_next_request_of_a_slot`：1 槽位 + `interval = 120ms` -> 相邻发起间隔 >= 120ms
- `zero_interval_keeps_slots_running_without_rest`：3 槽位 + `interval = 0` -> `max_active == 3`
- `inflight_ignores_the_outcome_classification_backlog`：8 发 8 回 6 分类 -> 在途 0
- e2e：`--concurrency 500` -> `concurrency 500 exceeds the maximum, using 64` + 同步照常完成

## 实测（`claude-code-en`，175 个请求）

<!-- prettier-ignore -->
| 参数 | 新实现耗时 | 旧实现耗时（全局闸门） |
| --- | --- | --- |
| `--concurrency 8 --interval 1s` | 37.9s | >= 175s |
| `--concurrency 8 --interval 0ms` | 23.9s | 23.9s |

`interval = 0` 时在途分布：`8/8` 占 296 帧、`7/8` 占 148 帧 -> 槽位真实打满。
