#!/usr/bin/env bash
set -euo pipefail

# Simple wrapper for markdownlint-cli2 if available; otherwise, print guidance.

usage() {
  echo "Usage: $0 [check|fix] [path...]" >&2
}

MODE=${1:-check}
shift || true
TARGETS=("${@:-docs/**/*.md}")

if command -v markdownlint-cli2 >/dev/null 2>&1; then
  if [[ "$MODE" == "fix" ]]; then
    markdownlint-cli2 --fix "${TARGETS[@]}"
  else
    markdownlint-cli2 "${TARGETS[@]}"
  fi
elif command -v markdownlint >/dev/null 2>&1; then
  if [[ "$MODE" == "fix" ]]; then
    markdownlint --fix "${TARGETS[@]}"
  else
    markdownlint "${TARGETS[@]}"
  fi
else
  echo "markdownlint not found. Install one of:" >&2
  echo "  npm i -g markdownlint-cli2" >&2
  echo "  or npm i -g markdownlint-cli" >&2
  exit 127
fi
