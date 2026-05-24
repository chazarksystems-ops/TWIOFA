# Phased Completion Sequence

## Status
- **Normative status:** required implementation order for the one-shot completion attempt.
- **Goal:** complete the playable/testable slice by moving through small gated phases.

## What this document does not authorize
- It does not permit skipping validation between phases.
- It does not permit adding systems outside the named phase.
- It does not permit broad rewrite of the scaffold.

## Phase order
| Phase | Name | Required output | May proceed when |
|---|---|---|---|
| 0 | Harness / Optimization Pass 0 | `src/bin/chunk0_harness.rs`, optional `src/harness.rs`, `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | corpus and stress mode deterministic |
| 1 | Slice 3 collapse | collapse logic + tests + report update | collapse tests pass and corpus remains green |
| 2 | Slice 4 water/moisture | water flow, double-buffer moisture, wet LooseSoil support + tests | moisture/order tests pass and corpus remains green |
| 3 | Slice 5 scent/harvest/return | scent decay/reinforcement, carcass harvest, deposit loop + tests | harvest full-loop tests pass |
| 4 | Play surface | CLI/REPL/text render + receipts view | user can run smoke path manually |
| 5 | Acceptance freeze | full-run scripts + docs/report | final validation matrix passes |

## Universal rules for every phase
- Run `cargo fmt --check`, `cargo check`, and `cargo test` after each phase.
- If formatting fails only due to formatting, run `cargo fmt`, then rerun checks.
- Do not use random/wall time/floating point in simulation state.
- Do not use `HashMap` iteration for receipts, hashes, or deterministic output.
- Do not include UI/render state in canonical hash.
- Do not change frozen Slice 2 semantics unless a regression test proves a bug.
- Update `CHUNK_0_IMPLEMENTATION_REPORT.md` after each completed phase.

## Required final reports
| File | Purpose |
|---|---|
| `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | Harness/corpus/stress mode report. |
| `CHUNK_0_PLAYABLE_SLICE_REPORT.md` | Final playable slice summary, commands, validation, known limits. |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Running implementation history. |

## Stop conditions
Stop immediately and report if:
- A phase requires inventing a rule not present in the docs.
- Collapse/AntGroup impact becomes ambiguous.
- Moisture double-buffer would require nondeterministic ordering.
- A command surface leaks hidden Sourback semantics in default view.
- The corpus becomes nondeterministic.
- Any code attempts card-dashboard gameplay.
- Any code uses `source_review/` as normative input.

## If the one-shot attempt gets too large
The agent should stop after the last fully green phase, report PASS/PARTIAL, and list the next exact phase prompt. It should not continue by guessing.
