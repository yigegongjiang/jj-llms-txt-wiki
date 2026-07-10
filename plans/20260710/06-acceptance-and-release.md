# 阶段 6：端到端验收与发布

## 目标

用可重复的本地测试和真实 GitHub Release 产物证明 README 全部行为已实现，并将文档状态从“业务 CLI 尚未实现”更新为当前事实。

## 前置状态

- 阶段 1–5 全部通过验收。
- 业务命令、快照语义和发布骨架已在当前分支完整存在。

## 实现任务

- 建立端到端 fixture：
  - 两个独立同源站点。
  - 多层相对/绝对 Markdown 链接、重复和环。
  - 同源/不同源重定向。
  - `404`、`410`、`429`、`5xx` 和超时。
  - 两次快照之间的新增、修改、删除。
- 通过临时 HOME、临时输出目录和本地 HTTP server 运行真实 CLI，不访问用户配置和公共网络。
- 校验 stdout、stderr、退出码、配置文件、目录树和文件内容。
- 检查依赖：
  - `reqwest` 使用 `rustls`，不引入 OpenSSL。
  - `Cargo.lock` 已更新并可用 `--locked` 构建。
  - Release 二进制不依赖开发期 fixture。
- 更新根 `README.md`：
  - 删除 Hello World/“业务 CLI 尚未实现”的过期描述。
  - 保持已定义的 CLI、配置、输出和同步行为不变。
  - 补充与真实 `--help` 一致的必要安装后验证命令，不增加未实现功能。
- 检查 `.github/workflows/release.yml` 与 `install.sh` 仍匹配最终资产名和 Cargo package version。
- 选择下一个未发布的 SemVer，确保 tag 为 `v<Cargo.toml version>`；不得覆盖已有 tag。

## 端到端验收

在隔离环境按顺序验证：

1. 添加两个站点并确认排序。
2. `sync <site>` 只生成目标站点的 URL 层级目录。
3. `sync` 生成两个站点快照。
4. 修改 fixture 后再次同步，确认新增/修改生效，删除和 `404`/`410` 文件消失。
5. 注入 `429`、`5xx`、超时，确认命令非零且旧快照内容和目录树不变。
6. 验证不同源 link 和 redirect 均未写入，也未继续抓取目标。
7. 验证 `--concurrency` 峰值和 `--interval` 启动间隔。
8. 验证正常成功/失败后无 `.cache` 和临时目录残留。

所有用例必须自动化；人工观察只能作为补充证据。

## 本地质量门槛

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
git diff --check
```

额外检查：

- `cargo tree` 中 TLS backend 符合 `rustls` 约束。
- `target/release/llms-wiki --help` 与 README 命令一致。
- 从空临时 HOME 运行不会读取或写入真实 `~/.config/llms-wiki` 和 `~/llms-wiki`。

## Release 验收

- 推送与 Cargo version 一致的新 `v*` tag。
- GitHub Actions 的 quality、macOS arm64、macOS x64、release jobs 全部成功。
- Release 同时包含：
  - `llms-wiki-darwin-arm64`
  - `llms-wiki-darwin-x64`
  - `checksums.txt`
- 在当前 macOS 架构执行 `install.sh`，核对下载资产 checksum、安装路径和版本。
- 在隔离安装目录验证 `update`/`upgrade` 和 `uninstall`，不得删除仓库内构建产物或其他同名文件。
- 安装后的 Release 二进制完成一次本地 fixture 的 `site add`、`site list` 和 `sync` 冒烟测试。

## 最终完成判定

- README 的设计原则、CLI、配置、输出目录和五步同步行为都有端到端证据。
- 所有单元、集成、端到端、Clippy、格式和 Release 构建检查通过。
- 失败语义已证明会保留旧完整快照；成功语义已证明会移除旧文件。
- Release 产物可安装并实际运行完整业务流程。
- 根 README 不再包含与实现冲突的“尚未实现”或 Hello World 描述。
- 不存在未处理的占位实现、跳过测试或计划内遗留项。

只有以上证据全部成立，整个 `plans/20260710` 才可标记完成。
