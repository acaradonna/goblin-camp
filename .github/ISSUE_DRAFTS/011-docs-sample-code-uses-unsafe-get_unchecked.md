Title: Docs include unsafe get_unchecked in examples; clarify context or provide safe variant

Summary
- Developer docs (`docs/developer/performance.md`) demonstrate `unsafe { ... get_unchecked(...) }` for tile access. While fine for performance docs, it risks copy-paste into production without guardrails.

Details
- Location: `docs/developer/performance.md` around the `tiles.get_unchecked` example.

Proposal
- Add a warning callout and a safe alternative example.
- Clarify preconditions required for safe usage.

Acceptance Criteria
- Docs updated with safety notes and safe alternative.
