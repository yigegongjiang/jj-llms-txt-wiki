# 阶段 1：CLI 基础与模块边界

## 目标

用 `clap` 替换 `src/main.rs` 中的手写参数分派，建立后续业务命令可直接接入的入口，同时保持现有安装生命周期能力不回退。

## 前置状态

- 根 [`README.md`](../../README.md) 已确认仍是当前事实源。
- 现有 `help`、`version`、`update`/`upgrade`、`uninstall` 的行为和测试已记录。
- 工作区和 staging area 已分别检查，实施过程不修改 staging area。

## 行为契约

- 二进制名保持 `llms-wiki`。
- `--help`/`help` 和 `--version`/`version` 由统一 CLI 入口提供。
- `upgrade` 继续作为 `update` 的别名。
- 无参数时展示顶层帮助，不再输出 `Hello, world!`。
- 未知命令、非法参数和运行时错误写入 stderr，并返回非零。
- `update` 和 `uninstall` 的已安装二进制校验、checksum 和替换/删除逻辑不改变。

## 实现任务

- 在 `Cargo.toml` 添加 `clap` 的 derive 支持；依赖版本写入 `Cargo.lock`。
- 将参数模型放入 `src/cli.rs`：
  - 顶层 `Cli`。
  - `Command::Update`、`Command::Uninstall`。
  - 为后续 `site` 和 `sync` 预留 Rust 模块边界，但不暴露不可用的占位命令。
- 将自更新、卸载、checksum 等逻辑从 `main.rs` 移入 `src/lifecycle.rs`。
- 将 `main.rs` 缩减为解析、分派、错误输出和退出码映射。
- 保留 `asset_name`、`repository_slug`、`expected_checksum` 等现有单元测试，并补充 CLI 解析测试。
- 删除 Hello World 文案及对应分支。

建议阶段末结构：

```text
src/
├── cli.rs
├── lifecycle.rs
└── main.rs
```

## 测试

- `Cli::try_parse_from` 覆盖 `help`、`version`、`update`、`upgrade`、`uninstall` 和未知命令。
- 原有 checksum 精确资产匹配测试继续通过。
- 使用临时复制的二进制验证生命周期命令仍拒绝错误文件名，避免触碰真实安装文件。
- 运行全局质量门槛。

## 验收

```bash
cargo run -- --help
cargo run -- --version
cargo run -- help
cargo run -- version
cargo run -- unknown
```

- 帮助和版本输出中的名称、版本来自 Cargo package metadata。
- `unknown` 返回非零且不 panic。
- `cargo test --locked` 覆盖 CLI 分派和原生命周期逻辑。
- `src/main.rs` 不再包含手写字符串匹配式命令解析。

## 本阶段不做

- 不实现配置、`site`、`sync`、网络请求或进度显示。
- 不改 Release 资产名、安装目录和仓库地址。
