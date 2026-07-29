```When Editing
本文档作用: 工程工作流程 (可用工具 / 调试 / 发布); MUST NOT 写工程说明 (→ README.md) / LLM 约束 (→ AGENTS.md)
遵循 AGENTS.md 文档编写规范
- 段落按工程实际保留或删除; 存在即为明确流程, MUST NOT 附加强度标记
- 发布内按顺序编号; 顶部 TL;DR ≤ 5 行; 删子段后重编号保持连续
- 风险 / 不可逆操作用 `>` 引用块, 标禁用条件
```

# 可用工具

- `gh` 已登录（GitHub Release / PR）

# 发布

代码变更完成后立即执行（= 需求交付的最后环节）。交付 = 预部署 + push。推送 `v*` tag → `.github/workflows/release.yml` 自动构建并发布 GitHub Release。

## TL;DR

依序执行：

1. 验证：`cargo fmt --all -- --check` + `clippy` + `test`
2. 写版本：`Cargo.toml` + `Cargo.lock` + `CHANGELOG.md` + `CHANGELOG.dev.md` 同步（与 tag 一致）
3. 预部署：`./scripts/install-local.sh`
4. 发布：commit + annotated tag（`-a -m`）+ push `main` + push tag → push 成功即结束，不验证 CI

## 1. 验证

与 CI quality job 一致，全绿才能发布：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

## 2. 写版本

- 版本号：默认递增 PATCH（第三位）；大功能更新 / 破坏性改动 → MINOR；MAJOR 仅人类主动要求。
- 同步编辑，全部与 tag 一致：
  - `Cargo.toml` 的 `version`
  - `CHANGELOG.md` + `CHANGELOG.dev.md` 追加对应版本条目
- 改完 `cargo check` 重新生成 `Cargo.lock`。

> CI 全程 `--locked`：`Cargo.lock` 未随 `Cargo.toml` 版本同步 → 构建直接失败。

## 3. 预部署

本机完成实际交付：release 构建 + 装入 `~/.local/bin` + `--version` 自检。

```bash
./scripts/install-local.sh
```

## 4. 发布

> 先 `git fetch origin --tags`，确认 `main` 无落后、目标 tag 未被占用（多会话并行时可能已被抢先发布）。
> CI 校验 `v<Cargo version> == tag`，不一致直接失败。

```bash
git commit -m "chore(release): prepare vX.Y.Z" -- Cargo.toml Cargo.lock CHANGELOG.md CHANGELOG.dev.md <其他改动文件>
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin main
git push origin vX.Y.Z
```

push 成功即交付结束，直接收尾。后续 CI（fmt/clippy/test → 构建 macOS arm64/x64 → `checksums.txt` → 创建 Release）由 GitHub 自动完成，不查结果、不等待、不轮询（不用 `gh run watch` / `gh run list` / Release 页面确认）。
