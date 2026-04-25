#!/usr/bin/env bash
# Run once before starting the mission to initialise the git repo.

set -e
cd "$(dirname "$0")/project"

if [ -d .git ]; then
  echo "Already initialised. To reset: rm -rf .git && ./setup.sh"
  exit 0
fi

git init -q
git add .
git commit -q -m "initial commit"
echo "Repo initialised in $(pwd)"
