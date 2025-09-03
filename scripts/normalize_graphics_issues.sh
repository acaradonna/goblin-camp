#!/usr/bin/env bash
set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: GitHub CLI 'gh' is not installed." >&2
  exit 1
fi

EPIC_URL=${EPIC_URL:-"https://github.com/acaradonna/goblin-camp/issues/181"}

declare -A issue_labels
issue_labels[182]="ui,graphics,bevy"
issue_labels[193]="core,bevy,refactor"
issue_labels[183]="core,bevy"
issue_labels[184]="graphics,tilemap"
issue_labels[185]="assets,docs,licensing"
issue_labels[186]="graphics,sprites"
issue_labels[187]="graphics,animation"
issue_labels[188]="ui,input"
issue_labels[189]="graphics"
issue_labels[194]="input,ui,gameplay"
issue_labels[195]="cli,integration"
issue_labels[190]="docs"
issue_labels[191]="ci"
issue_labels[192]="graphics,polish"

issue_ids=(182 183 184 185 186 187 188 189 190 191 192 193 194 195)

for id in "${issue_ids[@]}"; do
  echo "Normalizing #$id"
  title=$(gh issue view "$id" --json title --jq .title)
  labels_expected_csv=${issue_labels[$id]:-}
  # Add missing labels only
  if [[ -n "$labels_expected_csv" ]]; then
    IFS=',' read -ra expected <<< "$labels_expected_csv"
    for lbl in "${expected[@]}"; do
      # If label is already present, skip
      if gh issue view "$id" --json labels --jq '.labels[].name' | grep -Fxq "$lbl"; then
        :
      else
        gh issue edit "$id" --add-label "$lbl" >/dev/null
      fi
    done
  fi

  # Ensure Parent epic line exists in body
  body=$(gh issue view "$id" --json body --jq .body)
  if ! printf "%s" "$body" | grep -Fq "Parent epic:"; then
    tmp=$(mktemp)
    printf "%s\n\nParent epic: %s\n" "$body" "$EPIC_URL" > "$tmp"
    gh issue edit "$id" --body-file "$tmp" >/dev/null
    rm -f "$tmp"
  fi
done

echo "Done."
