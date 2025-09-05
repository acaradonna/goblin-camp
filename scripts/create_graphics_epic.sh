#!/usr/bin/env bash
set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: GitHub CLI 'gh' is not installed. Install from https://cli.github.com and retry." >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "Error: GitHub CLI is not authenticated. Run: gh auth login" >&2
  exit 1
fi

EPIC_TITLE="Epic: Graphics UI Alpha (gc_gfx)"
EPIC_BODY=$(cat <<'EOF'
Goals:
- Working, animated, pixel-art tile UI with camera/input.
- Reuse existing `bevy_ecs` systems deterministically.
- Keep TUI available.

Non-goals:
- Full menus, save/load UI, audio, advanced art polish.

Deliverables:
- `gc_gfx` crate; tilemap + sprites; visibility overlay; mouse designations.
- Docs: `docs/design/graphics.md`.

Risks:
- Version mismatches; asset licensing; WSL2 GUI. See epic description for mitigations.

Acceptance:
- Map + entities render; mining via mouse updates tiles; overlay toggles; 60 FPS on 80x50.
EOF
)

# Ensure labels exist
for L in ui graphics bevy epic core tilemap sprites animation input docs assets licensing ci polish refactor gameplay cli integration; do
  gh label create "$L" --color "ededed" --description "" >/dev/null 2>&1 || true
done

# Helper: find first issue URL exactly matching title
find_issue_url_by_title () {
  local TITLE="$1"
  gh issue list --state all --search "$TITLE in:title" --json title,url --jq \
    ".[] | select(.title==\"$TITLE\") | .url" | head -n 1
}

# Create epic tracking issue (idempotent)
EPIC_URL=$(find_issue_url_by_title "$EPIC_TITLE" || true)
if [[ -z "${EPIC_URL:-}" ]]; then
  CREATE_OUT=$(gh issue create --title "$EPIC_TITLE" --body "$EPIC_BODY" --label epic --label graphics --label ui --assignee "@me")
  EPIC_URL=$(echo "$CREATE_OUT" | grep -Eo 'https://github.com/[^ ]+/issues/[0-9]+' | head -n 1)
  # Fallback: re-query if URL not parsed
  if [[ -z "${EPIC_URL:-}" ]]; then
    EPIC_URL=$(find_issue_url_by_title "$EPIC_TITLE")
  fi
  echo "Created EPIC: $EPIC_URL"
else
  echo "Epic already exists: $EPIC_URL"
fi

create_issue () {
  local TITLE="$1"
  local BODY="$2"
  local LABELS="$3"
  local DEPENDS_ON="${4:-}"
  local URL
  local LABEL_ARGS=()
  # Expand comma-separated labels into multiple --label flags
  IFS=',' read -ra labels_arr <<< "$LABELS"
  for l in "${labels_arr[@]}"; do LABEL_ARGS+=("--label" "$l"); done
  if [[ -n "$DEPENDS_ON" ]]; then
    BODY="$BODY

Depends on: $DEPENDS_ON"
  fi
  BODY="$BODY

Parent epic: $EPIC_URL"
  # Idempotent: check by title first
  URL=$(find_issue_url_by_title "$TITLE" || true)
  if [[ -z "${URL:-}" ]]; then
    CREATE_OUT=$(gh issue create --title "$TITLE" --body "$BODY" "${LABEL_ARGS[@]}" --assignee "@me")
    URL=$(echo "$CREATE_OUT" | grep -Eo 'https://github.com/[^ ]+/issues/[0-9]+' | head -n 1)
    if [[ -z "${URL:-}" ]]; then
      URL=$(find_issue_url_by_title "$TITLE")
    fi
  fi
  echo "$URL"
}

issue1=$(create_issue "Create Bevy graphics crate gc_gfx and window" "Add new workspace crate; Bevy App; pixel camera; nearest sampling.

AC:
- cargo run -p gc_gfx opens a window with pixel-perfect camera
- CI compiles gc_gfx" "ui,graphics,bevy")
issue2=$(create_issue "Refactor bootstrap to support Bevy App world" "Expose plugin/setup that inserts core resources/entities into Bevy World.

AC:
- Core runs in Bevy World (no second World)
- Sim behavior unchanged for N ticks" "core,bevy,refactor" "$issue1")
issue3=$(create_issue "Integrate core simulation into Bevy schedule" "Register systems/time; maintain determinism.

AC:
- Pause/step parity with TUI for N steps" "core,bevy" "$issue2")
issue4=$(create_issue "Add tilemap rendering for GameMap" "Use bevy_ecs_tilemap; TileKind->atlas indices; apply diffs.

AC:
- Tiles render & update on mining" "graphics,tilemap" "$issue3")
issue5=$(create_issue "Load and credit a permissive tileset" "Add assets/tiles + attribution.

AC:
- Assets load; README credits present" "assets,docs,licensing" "$issue1")
issue6=$(create_issue "Entity sprites for goblins and items" "Sprites for Miner/Carrier/Stone; proper z-order.

AC:
- Sprites at Position; visible with overlay" "graphics,sprites" "$issue4,$issue5")
issue7=$(create_issue "Movement tweening and basic sprite animation" "Tween between tiles; walk cycle frames.

AC:
- Smooth movement; idle vs walk frames" "graphics,animation" "$issue6")
issue8=$(create_issue "Camera controls and UI overlay" "WASD/arrow pan, +/- zoom; HUD text for paused/steps.

AC:
- Input works; HUD reflects state" "ui,input" "$issue3")
issue9=$(create_issue "Visibility overlay layer" "Semi-transparent tile overlay from FOV.

AC:
- Toggleable overlay without perf regressions" "graphics" "$issue4,$issue8")
issue10=$(create_issue "Mouse mining designations" "Click-drag rectangle spawns MineDesignation.

AC:
- Mining changes tiles and drops items" "input,ui,gameplay" "$issue3,$issue4,$issue8")
issue11=$(create_issue "CLI integration: add gfx demo route" "Add 'gfx' subcommand to launch gc_gfx.

AC:
- goblin-camp gfx launches Bevy UI" "cli,integration" "$issue1")
issue12=$(create_issue "Docs: graphics design + runbook" "Design doc, assets license notes, WSL/GL troubleshooting.

AC:
- Clear run instructions and credits" "docs" "$issue5")
issue13=$(create_issue "CI build of gc_gfx" "Compile-only CI job for gc_gfx.

AC:
- CI passes on Linux" "ci" "$issue1")
issue14=$(create_issue "Polish: water/lava tile animation + particles (optional)" "Animate water/lava; simple mining particles.

AC:
- Visible animated tiles + effect" "graphics,polish" "$issue7,$issue9")

echo "Created issues:"
echo "$issue1"
echo "$issue2"
echo "$issue3"
echo "$issue4"
echo "$issue5"
echo "$issue6"
echo "$issue7"
echo "$issue8"
echo "$issue9"
echo "$issue10"
echo "$issue11"
echo "$issue12"
echo "$issue13"
echo "$issue14"
echo "All issues reference EPIC: $EPIC_URL"


