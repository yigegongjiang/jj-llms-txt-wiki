```When Editing
本文档作用: 工程工作流程 (可用工具 / 调试 / 发布); MUST NOT 写工程说明 (→ README.md) / LLM 约束 (→ AGENTS.md)
遵循 AGENTS.md 文档编写规范
- 段落按工程实际保留或删除; 存在即为明确流程, MUST NOT 附加强度标记
- 发布内按顺序编号; 顶部 TL;DR ≤ 5 行; 删子段后重编号保持连续
- 风险 / 不可逆操作用 `>` 引用块, 标禁用条件
```

# 可用工具

- `gh` 已登录（GitHub Release / PR）

# 调试

`cargo run -- <command>` 运行骨架；`cargo test --locked` 跑单测。

验证：

```bash
cargo run -- version   # 打印 llms-wiki <version>
cargo run -- help      # 打印用法
cargo test --locked    # 单测通过
```

# 发布

代码变更完成后立即执行（= 需求交付的最后环节）。推送 `v*` tag → `.github/workflows/release.yml` 自动构建并发布 GitHub Release。

## TL;DR

依序执行：

1. 验证：`cargo fmt --all -- --check` + `clippy` + `test`
2. 写版本：`Cargo.toml` + `Cargo.lock` + `CHANGELOG.md` + `CHANGELOG.dev.md` 同步（与 tag 一致）
3. 发布：commit + annotated tag（`-a -m`）+ push `main` + push tag
4. 修上版 bug：amend + 删 Release + 删 tag + 重打 + force push

## 1. 验证

与 CI quality job 一致，全绿才能发布：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

## 2. 写版本

- 版本号：默认递增 PATCH（第三位）；新功能 → MINOR；不兼容改动 → MAJOR。
- 同步编辑，全部与 tag 一致：
  - `Cargo.toml` 的 `version`
  - `CHANGELOG.md` + `CHANGELOG.dev.md` 追加对应版本条目
- 改完 `cargo check` 重新生成 `Cargo.lock`。

> CI 全程 `--locked`：`Cargo.lock` 未随 `Cargo.toml` 版本同步 → 构建直接失败。

## 3. 发布

> CI 校验 `v<Cargo version> == tag`，不一致直接失败。

```bash
git commit -am "chore(release): prepare vX.Y.Z"
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin main
git push origin vX.Y.Z
```

tag 推送后 CI：fmt/clippy/test → 构建 macOS arm64/x64 → 生成 `checksums.txt` → 创建 Release。

## 4. 修上版 bug

上版刚发布即发现明显 bug 时，amend 修复后按同版本号重发：

```bash
git commit -a --amend --no-edit
gh release delete vX.Y.Z -y            # 先删已发布 Release
git tag -d vX.Y.Z
git push origin :refs/tags/vX.Y.Z
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin main --force-with-lease
git push origin vX.Y.Z
```

> `--verify-tag` 遇已存在 Release 会失败，故必先 `gh release delete`。
> 仅限该版本尚未被用户安装；已扩散则改为递增新 PATCH 重发，MUST NOT force push。
