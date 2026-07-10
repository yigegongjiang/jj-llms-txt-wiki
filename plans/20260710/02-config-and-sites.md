# 阶段 2：配置与站点管理

## 目标

实现 README 规定的 TOML 配置和 `site add/list`，使站点与全局抓取参数可持久化、校验并稳定读取。

## 前置状态

- 阶段 1 已通过验收。
- CLI 入口可增加嵌套子命令，生命周期命令保持可用。

## 行为契约

- 默认配置路径固定为 `~/.config/llms-wiki/config.toml`。
- 配置文件不存在时使用：
  - `output_dir = "~/llms-wiki"`
  - `concurrency = 4`
  - `interval_ms = 500`
  - 空 `sites`。
- `output_dir` 仅展开开头的 `~` 或 `~/`；`~user` 不支持并报错。
- `concurrency` 必须大于 `0`；`interval_ms` 使用非负整数。
- 站点名必须可安全作为单个目录名：只允许 ASCII 字母、数字、`.`、`_`、`-`，首字符为字母或数字，且不得为 `.` 或 `..`。
- 入口必须是绝对 HTTP/HTTPS URL，并具有 origin。
- `site add` 遇到重复名称时失败，不静默覆盖。
- `site list` 按名称排序输出 `name` 和 URL；无站点时成功输出空列表。
- 配置写入采用同目录临时文件加 rename；失败不得破坏旧配置。
- 本阶段不增加配置路径环境变量或额外 CLI 参数。

## 实现任务

- 在 `Cargo.toml` 添加 `serde` derive、`toml` 和 `url`。
- 新增 `src/config.rs`：
  - `Config`、`SiteConfig` 及默认值。
  - 默认配置路径和 `output_dir` 展开。
  - `load`、`validate`、`save`。
  - 同目录原子文件替换和失败清理。
- 扩展 `src/cli.rs`：
  - `site add <name> <url>`。
  - `site list`。
- 新增 `src/site.rs`，负责站点名/URL 校验和配置变更；CLI 层只做输入输出。
- 配置目录不存在时由 `site add` 创建；只读的 `site list` 不产生无意义文件。
- 将错误附带配置路径、字段或站点名上下文，但不得输出用户配置以外的敏感环境内容。

建议阶段末结构：

```text
src/
├── cli.rs
├── config.rs
├── lifecycle.rs
├── main.rs
└── site.rs
```

## 测试

- 默认值、完整 TOML、缺失字段和非法 TOML。
- `~` 展开、绝对路径、缺失 `HOME`、不支持的 `~user`。
- 合法/非法站点名和 HTTP/HTTPS/相对/非 HTTP URL。
- 首次添加、重复添加、多个站点排序。
- 模拟写入失败后旧配置字节不变，且无临时文件残留。
- CLI 测试通过注入临时配置路径或配置存储对象隔离真实用户目录；不引入产品级隐藏参数。

## 验收

在隔离的临时 HOME 下执行：

```bash
llms-wiki site add anthropic https://platform.claude.com/llms.txt
llms-wiki site add another-site https://example.com/llms.txt
llms-wiki site list
```

- 生成的 `~/.config/llms-wiki/config.toml` 可由程序再次读取。
- 列表按 `another-site`、`anthropic` 排序。
- 重复添加返回非零，配置文件内容不变。
- 默认配置与 README 示例一致。
- 全局质量门槛通过。

## 本阶段不做

- 不下载入口或 Markdown。
- 不创建输出根目录、站点目录或 `.cache`。
- 不实现站点删除、修改或交互式编辑。
