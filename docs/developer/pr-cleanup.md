# PR Cleanup Process (for Agents)

Follow these steps for every PR to ensure it’s merge-ready and consistent:

1) Prepare locally
- Fetch and checkout the PR branch: `gh pr checkout <number>`
- Update `main`: `git fetch origin`
- Rebase the PR onto latest main (no merges): `git rebase origin/main`
- Resolve conflicts; prefer upstream patterns and keep scope minimal

2) Run local CI checks
- Format: `cargo fmt --all -- --check`
- Clippy: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Tests: `cargo nextest run --workspace` and `cargo test --doc --workspace`
- Demos (if applicable): run CLI demos that the PR touches

3) Address review feedback
- Implement all Copilot/Cursor/Bugbot comments
- Remove dead code/duplication; refactor repeated logic into helpers
- Ensure error handling returns proper errors (don’t just print)

4) Commit hygiene
- Use allowed branch prefixes: feat/, fix/, docs/, refactor/, test/, chore/
- Create fixup commits targeting the original change: `git commit --fixup <sha>`
- Autosquash to linear atomic history: `GIT_EDITOR=true git rebase -i --autosquash origin/main`
- Ensure ≤50 char subject, format `(scope): message`; include issue references (Fixes #123) when appropriate
- No merge commits

5) Validation parity
- Run PR validation locally before pushing: `./scripts/validate-pr.sh --commit-range origin/main..HEAD --branch-name $(git branch --show-current)`
- Pre-push hook will run the same check automatically; fix failures locally

6) Push & update PR
- Push with lease: `git push --force-with-lease`
- Update PR title/body (no escaped \n), add labels, and ensure the body clearly states scope and closes issues

7) Verify CI
- Monitor GitHub checks; fix red jobs quickly
- Re-run local checks as needed and push minimal fixups, then autosquash

Tips
- Keep changes tightly scoped to the PR’s intent
- Prefer deterministic, stable behaviors; add tests where necessary
- When in doubt, rebase small and often; avoid merges
