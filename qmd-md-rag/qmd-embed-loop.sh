#!/usr/bin/env bash
# qmd embed 循环器
#
# 背景: `qmd embed` 单次会话有硬编码的 30 分钟上限 (store.js: maxDuration: 30*60*1000)。
# md 很多时会在到点时中止, 打印 "Session expired — skipping N remaining chunks"。
#
# 关键前提(已核对源码): embed 是增量且可断点续跑的。
#   - 只嵌入还缺向量的 doc (getPendingEmbeddingDocs / chunk_count < expected_chunks)
#   - 每个 chunk 嵌入后立即写入 SQLite (insertEmbedding), 已完成的不会丢
# 因此反复运行 `qmd embed` 即可续跑, 本脚本负责循环 + 判停。
#
# 用法:
#   ./qmd-embed-loop.sh                 # 嵌入全部 collection
#   ./qmd-embed-loop.sh -c <collection> # 只嵌入指定 collection
#
# 注意: 禁止传 -f/--force —— 它每轮都会清空全部向量, 导致永远跑不完。脚本会直接拒绝。

set -uo pipefail

# 拒绝 -f/--force: 与增量续跑语义冲突
for arg in "$@"; do
  case "$arg" in
    -f|--force)
      echo "✗ 不能在循环里用 $arg —— 它每轮清空全部向量, 会导致死循环。" >&2
      echo "  要强制重嵌请单独运行一次: qmd embed -f" >&2
      exit 2
      ;;
  esac
done

strip_ansi() { sed $'s/\x1b\\[[0-9;]*m//g'; }

# 输出 "<vectors> <pending>"; 缺失字段按 0 计 (pending 行仅在有待嵌入时打印)
read_counts() {
  local s v p
  s=$(qmd status 2>/dev/null | strip_ansi)
  v=$(awk '/Vectors:/{print $2; exit}' <<<"$s"); v=${v:-0}
  p=$(awk '/Pending:/{print $2; exit}' <<<"$s"); p=${p:-0}
  echo "$v $p"
}

round=0
prev_v=-1
while :; do
  round=$((round + 1))
  read -r v p < <(read_counts)
  printf '── round %d | vectors=%s pending=%s ──\n' "$round" "$v" "$p"

  if [ "$p" -eq 0 ]; then
    echo "✓ 全部嵌入完成 (pending=0), 共 $round 轮。"
    exit 0
  fi
  # 无进展保护: 上一轮 embed 后向量数没涨 => 剩余是超大 doc(单轮嵌不完被回滚)
  # 或持续失败的 chunk, 再跑也是死循环, 停。
  if [ "$v" -eq "$prev_v" ]; then
    echo "⚠ 本轮无进展 (vectors 未增长), 剩余 $p 个 doc 可能单次跑不完或持续失败, 停止。"
    echo "  可尝试: qmd embed --max-docs-per-batch 4 --max-batch-mb 8  (减小批, 便于卡住的大文件推进)"
    exit 1
  fi
  prev_v=$v

  qmd embed "$@" || echo "· qmd embed 退出码 $? — 视作中途中止, 继续下一轮续跑"
done
