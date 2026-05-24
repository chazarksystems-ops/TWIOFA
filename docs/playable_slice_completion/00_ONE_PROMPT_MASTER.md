# One-Prompt Master Build Prompt

Copy/paste this entire prompt to the coding agent from the workspace root.

```text
You are inside:

C:\Users\cheez\Downloads\twiofa_chunk0_spec_workspace_v0_3_2_corrected

Goal:
Complete the TWIOFA Chunk 0 playable/testable slice in one controlled attempt, using the existing frozen Slice 2 baseline. If the whole sequence succeeds, great. If a phase needs refinement, stop after the last green phase and report exactly what remains. If anything goes badly, stop immediately; Chaz has a backup.

This prompt authorizes the bounded completion path only when Chaz explicitly pastes/approves it. Do not expand beyond Chunk 0.

READ FIRST, IN THIS ORDER:
1. BUILD_HANDOFF_V0_3_3.md
2. CHUNK_0_IMPLEMENTATION_REPORT.md
3. docs/elevated_reasoning/01_SLICE_2_FREEZE_REPORT.md
4. docs/elevated_reasoning/05_OPTIMIZATION_PASS_0_DESIGN.md
5. docs/elevated_reasoning/06_SLICE_3_READINESS_GATE.md
6. docs/playable_slice_completion/README.md
7. docs/playable_slice_completion/01_CURRENT_STATE_AND_GAPS.md
8. docs/playable_slice_completion/02_PHASED_COMPLETION_SEQUENCE.md
9. docs/playable_slice_completion/03_OPTIMIZATION_PASS_0_IMPLEMENTATION_SPEC.md
10. docs/playable_slice_completion/04_SLICE3_COLLAPSE_IMPLEMENTATION_SPEC.md
11. docs/playable_slice_completion/05_SLICE4_MOISTURE_WATER_IMPLEMENTATION_SPEC.md
12. docs/playable_slice_completion/06_SLICE5_SCENT_HARVEST_RETURN_IMPLEMENTATION_SPEC.md
13. docs/playable_slice_completion/07_CLI_REPL_VISUALIZER_IMPLEMENTATION_SPEC.md
14. docs/playable_slice_completion/08_ACCEPTANCE_AND_VALIDATION_MATRIX.md
15. Relevant spec_pack files: 03, 04, 05, 06, 07, 12.
16. Current Rust code under chunk0_rust_scaffold/src and tests.

DO NOT USE AS NORMATIVE INPUT:
- source_review/
- old card-dashboard framing
- rejected agent assumptions
- target/ build artifacts
- twiofa_claude_code_docs_patch/ except as already-applied documentation provenance

ABSOLUTE HARD LIMITS:
- No card-dashboard gameplay.
- No full-game canon lock.
- No procedural yard.
- No Tauri polish before CLI/text play works.
- No A* or general pathfinding.
- No GPU/Margolus/block CA.
- No MCTS/MCMC.
- No doctrine/economy/combat.
- No random/wall-time/floating-point core simulation.
- No HashMap iteration in canonical state, hashes, or receipts.
- No hidden truth leak in default ColonyView/render.
- Do not change frozen Slice 2 semantics unless fixing a regression and preserving all 27 tests.

IMPLEMENT IN THIS ORDER:

PHASE 0 — Optimization Pass 0 harness
Use docs/playable_slice_completion/03_OPTIMIZATION_PASS_0_IMPLEMENTATION_SPEC.md.
Add:
- src/bin/chunk0_harness.rs
- src/harness.rs if useful
- CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md
Implement:
- run-corpus
- stress-local-fixtures --seed 42 --cases 100
- compare-baseline if small, otherwise guarded stub
Validate with fmt/check/test + harness commands.

PHASE 1 — Slice 3 collapse
Use docs/playable_slice_completion/04_SLICE3_COLLAPSE_IMPLEMENTATION_SPEC.md.
Implement only deterministic bottom-up LooseSoil collapse + AntGroup landing consequence.
No moisture/water/scent/harvest/UI.
Validate with required tests and harness corpus additions.

PHASE 2 — Slice 4 water/moisture
Use docs/playable_slice_completion/05_SLICE4_MOISTURE_WATER_IMPLEMENTATION_SPEC.md.
Implement deterministic water flow, double-buffer moisture, wet LooseSoil support decay.
No scent/harvest/UI.
Validate with required tests and harness additions.

PHASE 3 — Slice 5 scent/harvest/return loop
Use docs/playable_slice_completion/06_SLICE5_SCENT_HARVEST_RETURN_IMPLEMENTATION_SPEC.md.
Implement scent decay/effects/reinforcement, carcass harvest, and return-home deposit counter.
Validate with required tests and harness additions.

PHASE 4 — CLI/REPL/text visualizer
Use docs/playable_slice_completion/07_CLI_REPL_VISUALIZER_IMPLEMENTATION_SPEC.md.
Add:
- src/bin/chunk0_cli.rs
- optional src/play.rs
Implement one-shot commands, REPL, render colony, render devtruth, run-script smoke.
Default render must not leak hidden truth.

PHASE 5 — final acceptance
Use docs/playable_slice_completion/08_ACCEPTANCE_AND_VALIDATION_MATRIX.md.
Run:
- cargo fmt --check
- cargo check
- cargo test
- cargo run --bin chunk0_harness -- run-corpus
- cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
- cargo run --bin chunk0_cli -- run-script smoke
Create/update:
- CHUNK_0_IMPLEMENTATION_REPORT.md
- CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md
- CHUNK_0_PLAYABLE_SLICE_REPORT.md

UNIVERSAL VALIDATION AFTER EACH PHASE:
Run from chunk0_rust_scaffold/:
- cargo fmt --check
- cargo check
- cargo test
If fmt fails due to formatting only, run cargo fmt, then rerun all three.
Do not proceed to the next phase unless the current phase validates.

OUTPUT REQUIRED:
1. PASS / PARTIAL / FAIL
2. Highest completed green phase
3. Files changed
4. Commands/binaries added
5. Tests added
6. Validation results for each phase
7. Harness run results
8. CLI smoke result
9. Any deviations from the packet
10. Remaining work, if any
11. Confirmation no source_review/ normative use
12. Confirmation no card-dashboard behavior
13. Confirmation default ColonyView/render hides hidden truth

STOP CONDITIONS:
- Any phase fails and cannot be fixed locally without inventing rules.
- A spec ambiguity blocks deterministic implementation.
- Any hidden truth leak appears in default view.
- Corpus becomes nondeterministic.
- Cargo validation fails after reasonable bounded repair.
- The required work expands beyond Chunk 0.

Do not ask for back-and-forth unless a stop condition is hit. Make the best bounded implementation attempt, then report results.
```
