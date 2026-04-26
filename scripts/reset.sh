#!/usr/bin/env bash
# 演習で編集した exercise.* を HEAD に戻すスクリプト。
#
# 使い方:
#   scripts/reset.sh                          # 全ミッションをリセット
#   scripts/reset.sh 01-movement              # 章単位でリセット
#   scripts/reset.sh 01-movement/mission-2    # 個別ミッションをリセット
#
# 引数は chapters/ 配下の相対パス（複数指定可）。

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

if [[ $# -eq 0 ]]; then
  targets=(chapters)
else
  targets=()
  for arg in "$@"; do
    targets+=("chapters/$arg")
  done
fi

# 対象配下の exercise.* を git HEAD に戻す。
files=()
while IFS= read -r f; do
  files+=("$f")
done < <(git ls-files -- "${targets[@]}" | grep -E '/exercise\.[^/]+$' || true)

if [[ ${#files[@]} -eq 0 ]]; then
  echo "no exercise files found under: ${targets[*]}" >&2
  exit 1
fi

git checkout HEAD -- "${files[@]}"
echo "reset ${#files[@]} file(s):"
printf '  %s\n' "${files[@]}"
