#!/usr/bin/env bash
set -euo pipefail

# Normalize nested unordered list indents to two spaces across markdown files.
# This is a conservative regex replacement that fixes lines that begin with
# three or more spaces followed by a dash, converting to exactly two spaces.

paths=("${@:-docs/**/*.md}")

for p in "${paths[@]}"; do
  # Use a temp file to avoid issues with sed -i differences across platforms
  tmp="$(mktemp)"
  # Replace start-of-line of 3+ spaces before '-' with exactly two spaces
  sed -E 's/^[[:space:]]{3,}- /  - /' "$p" > "$tmp" || true
  if ! cmp -s "$p" "$tmp"; then
    mv "$tmp" "$p"
    echo "Fixed nested list indents: $p"
  else
    rm -f "$tmp"
  fi
done
