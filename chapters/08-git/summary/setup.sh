#!/usr/bin/env bash
# Initialise the project as a git repo with a couple of commits so that
# `git log`, `git blame`, and `git diff HEAD~1 HEAD` show meaningful output.

set -e
cd "$(dirname "$0")/project"

if [ -d .git ]; then
  echo "Already initialised. To reset: rm -rf .git && ./setup.sh"
  exit 0
fi

git init -q

# Commit 1: working version
cat > src/buggy.rs <<'EOF'
pub fn validate(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("empty input".to_string());
    }
    Ok(())
}
EOF
git add .
git commit -q -m "initial: add validate"

# Commit 2: introduce the bug + add space check
cat > src/buggy.rs <<'EOF'
pub fn validate(input: &str) -> Result<(), String> {
    // BUG: the empty-string check is inverted.
    if !input.is_empty() {
        return Err("empty input".to_string());
    }

    if input.contains(' ') {
        return Err("contains space".to_string());
    }

    Ok(())
}

pub fn parse(input: &str) -> Result<Vec<i32>, String> {
    validate(input)?;
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>().map_err(|e| e.to_string()))
        .collect()
}
EOF
git add src/buggy.rs
git commit -q -m "feat: add space check and parse helper"

echo "Repo initialised in $(pwd)"
