# Final Acceptance and Validation Matrix

## Status
- **Normative status:** final checklist for calling the result a playable/testable Chunk 0 slice.

## What this document does not authorize
- It does not approve further scope after Chunk 0.
- It does not approve full-game canon.

## Required commands before final freeze
From `chunk0_rust_scaffold/`:

```text
cargo fmt --check
cargo check
cargo test
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_cli -- run-script smoke
```

If a binary is intentionally deferred, the report must say so and Chaz must approve the defer.

## Functional acceptance
| Area | Required proof |
|---|---|
| Grid | 128x128, 16,384 cells, stable index. |
| Boundary | Stone ring unchanged after all scripts. |
| Dig | Valid adjacent dig changes target to Tunnel and emits receipt. |
| Movement | Cardinal-only, deterministic greedy stepping. |
| Sourback | Slowdown, transition WorkerLoss, and perception guardrails. |
| Collapse | Deterministic bottom-up LooseSoil collapse with AntGroup impact. |
| Water | Deterministic priority flow. |
| Moisture | Double-buffer diffusion, order-independent enough for Chunk 0. |
| Wet support | LooseSoil support decays when moisture > 120. |
| Scent | Decay, water/wet effects, movement/harvest reinforcement. |
| Harvest | One adjacent carcass harvested deterministically. |
| Return | Food carried can be returned/deposited. |
| Receipts | Every command has receipt with hashes and event/perception summary. |
| Hash | Same script produces same hash sequence. |
| Render | Colony view hides hidden truth; DevTruth is opt-in. |
| CLI | User can play a smoke path without writing tests. |

## Required final tests or scripts
- `full_run_seeded_yard_smoke_test`
- `scout_residue_then_forage_then_return`
- `dig_then_collapse_receipt`
- `water_moisture_support_interaction`
- `harvest_one_carcass_and_return_food`
- `renderframe_no_truth_leak_after_full_run`
- `repeated_full_run_hash_sequence_stable`

These may be Rust tests, harness corpus scripts, or CLI smoke scripts, but final report must say where each is covered.

## Documentation acceptance
Create/update:
- `CHUNK_0_IMPLEMENTATION_REPORT.md`
- `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md`
- `CHUNK_0_PLAYABLE_SLICE_REPORT.md`

Final report must include:
- files changed,
- commands added,
- tests added,
- validation results,
- known deferred items,
- exact manual play instructions,
- assurance that `source_review/` was not normative,
- assurance that card-dashboard behavior was not introduced.

## Known deferred after playable slice
Allowed to remain deferred:
- Full yard/procedural generation.
- Tauri polished UI.
- Multi-ant groups.
- Doctrine/economy/combat.
- A* pathfinding.
- GPU/Margolus/MCTS/MCMC.
- Full save/load unless implemented safely.

## Final freeze language
Use this phrase only if all acceptance items pass:

```text
Chunk 0 playable/testable slice is complete as a deterministic local scaffold. This does not lock full-game canon and does not authorize expansion beyond Chunk 0 without Chaz approval.
```
