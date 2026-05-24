# Playable Slice Completion Packet

## Status
- **Normative status:** implementation-control packet derived from the v0.3.3 spec, frozen Slice 2 report, current Rust scaffold, and elevated reasoning docs.
- **Purpose:** give a coding agent enough explicit direction to complete a playable, testable Chunk 0 slice with minimal interpretation.
- **Current baseline:** Slice 2 is frozen, 27 tests are reported passing, and Slice 3 remains gated unless Chaz explicitly approves this completion pass.

## What this packet does not authorize by itself
- Full-game canon.
- New mechanics beyond Chunk 0.
- Card-dashboard gameplay.
- Tauri/UI polish beyond the minimal visual/CLI surface defined here.
- GPU, Unreal, MCTS/MCMC, Margolus/block CA, procedural yard, economy, doctrine, or combat.
- Use of `source_review/` as current canon.

## Reading order
1. `00_ONE_PROMPT_MASTER.md` — pasteable one-shot build prompt.
2. `01_CURRENT_STATE_AND_GAPS.md` — what exists now and what is missing.
3. `02_PHASED_COMPLETION_SEQUENCE.md` — required order and stop gates.
4. `03_OPTIMIZATION_PASS_0_IMPLEMENTATION_SPEC.md` — deterministic harness before more material complexity.
5. `04_SLICE3_COLLAPSE_IMPLEMENTATION_SPEC.md` — bottom-up collapse only.
6. `05_SLICE4_MOISTURE_WATER_IMPLEMENTATION_SPEC.md` — water, double-buffer moisture, wet support.
7. `06_SLICE5_SCENT_HARVEST_RETURN_IMPLEMENTATION_SPEC.md` — scent, carcass harvest, food return loop.
8. `07_CLI_REPL_VISUALIZER_IMPLEMENTATION_SPEC.md` — minimal playable command surface and text/grid view.
9. `08_ACCEPTANCE_AND_VALIDATION_MATRIX.md` — final proof checklist.

## Source hierarchy for agents
| Source | Use |
|---|---|
| `BUILD_HANDOFF_V0_3_3.md` | High-level build spine and module ownership. |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Current implementation state and frozen Slice 2 semantics. |
| `docs/elevated_reasoning/` | Elevated guidance and gates, especially Slice 2 freeze and Optimization Pass 0 design. |
| `spec_pack/03`, `04`, `05`, `06`, `07`, `12` | Normative rules and validation expectations. |
| `agent_mapping_pass_01/05`, `06`, `07`, `08`, `10`, `12`, `14` | Reconciled support maps; use to clarify surfaces, not to override spec. |
| `chunk0_rust_scaffold/` | Current implementation baseline. |
| `source_review/` | Historical provenance only; do not use as canon. |

## Definition of “done”
Chunk 0 becomes a fully playable/testable slice when:
- `cargo fmt --check`, `cargo check`, and `cargo test` pass.
- The deterministic harness corpus and stress mode pass.
- Collapse, water/moisture, scent, carcass harvest, return-food deposit, and CLI/play commands all work through receipts.
- A user can run a command session without writing Rust tests.
- Default render/text view does not leak hidden truth.
- A scripted full-run smoke path is deterministic and repeatable.
