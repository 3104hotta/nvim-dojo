#!/usr/bin/env bash

PROGRESS_FILE="$(dirname "$0")/progress.md"

declare -a CHAPTER_NAMES=(
  "第1章  移動の高速化        "
  "第2章  モードとインサート   "
  "第3章  動詞＋名詞          "
  "第4章  繰り返しの3種       "
  "第5章  検索・置換          "
  "第6章  ファイル・バッファ  "
  "第7章  LSP                "
  "第8章  git 操作           "
  "第9章  init.lua           "
  "最終総合演習               "
)

declare -a SECTION_PATTERNS=(
  "第1章"
  "第2章"
  "第3章"
  "第4章"
  "第5章"
  "第6章"
  "第7章"
  "第8章"
  "第9章"
  "最終総合演習"
)

bar() {
  local done=$1
  local total=$2
  local width=8
  local filled=$(( done * width / total ))
  local empty=$(( width - filled ))
  local b=""
  for ((i=0; i<filled; i++)); do b+="█"; done
  for ((i=0; i<empty; i++)); do b+="░"; done
  echo "$b"
}

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Neovim 実践学習 — 進捗サマリー"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

total_done=0
total_all=0

for i in "${!SECTION_PATTERNS[@]}"; do
  pattern="${SECTION_PATTERNS[$i]}"
  name="${CHAPTER_NAMES[$i]}"

  in_section=0
  done_count=0
  all_count=0

  while IFS= read -r line; do
    if [[ "$line" =~ ^##.*"$pattern" ]]; then
      in_section=1
      continue
    fi
    if [[ $in_section -eq 1 ]]; then
      if [[ "$line" =~ ^## ]]; then
        break
      fi
      if [[ "$line" =~ ^-\ \[\ \] ]]; then
        ((all_count++))
      elif [[ "$line" =~ ^-\ \[x\] ]]; then
        ((done_count++))
        ((all_count++))
      fi
    fi
  done < "$PROGRESS_FILE"

  b=$(bar "$done_count" "$all_count")
  printf "  %s %s  %d/%d\n" "$name" "$b" "$done_count" "$all_count"

  total_done=$((total_done + done_count))
  total_all=$((total_all + all_count))
done

pct=0
if [[ $total_all -gt 0 ]]; then
  pct=$(( total_done * 100 / total_all ))
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "  合計  %d / %d  (%d%%)\n" "$total_done" "$total_all" "$pct"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
