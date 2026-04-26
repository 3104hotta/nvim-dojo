#!/usr/bin/env bash
# Reset exercise-1/, exercise-2/, exercise-3/ to their committed (initial) state.
# Use this between attempts so you can start any sub-problem from scratch.

set -e
cd "$(dirname "$0")"

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "error: not inside a git repository — cannot restore initial state"
  exit 1
fi

targets=()
for dir in exercise-1 exercise-2 exercise-3; do
  if git ls-files --error-unmatch "$dir" >/dev/null 2>&1; then
    targets+=("$dir")
  fi
done

if [ ${#targets[@]} -eq 0 ]; then
  echo "error: no tracked exercise-* directories found."
  echo "       commit the initial state first: git add final-exam/intermediate && git commit"
  exit 1
fi

git checkout HEAD -- "${targets[@]}"
git clean -fd "${targets[@]}" >/dev/null

echo "reset: restored ${targets[*]} to committed state"
