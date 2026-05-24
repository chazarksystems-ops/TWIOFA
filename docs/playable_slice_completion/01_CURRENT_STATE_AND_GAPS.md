# Current State and Remaining Gaps

## Status
- **Normative status:** factual planning snapshot for the completion pass.
- **Baseline:** current `chunk0_rust_scaffold/` implements Slice 1 and Slice 2.
- **Reported validation baseline:** 27 tests passed / 0 failed after Slice 2 semantic correction.

## What this document does not authorize
- It does not authorize new rules beyond the named completion phases.
- It does not authorize Slice 3 by itself; approval must be explicit in the agent prompt.
- It does not permit broad redesign.

## Implemented now
| Area | Current state |
|---|---|
| World grid | `Vec<Cell>` with 16,384 cells, 128x128. |
| Boundary | Explicit immutable Stone ring at x=0, x=127, y=0, y=127. |
| Materials | Air, Soil, LooseSoil, Tunnel, Water, Carcass, Root, Stone, NestWall. |
| Residue | SourbackBitter exists as hidden WorldTruth with guarded ColonyView wording. |
| Digging | Cardinal-adjacent Soil/LooseSoil -> Tunnel; support=0; neighbor support -=50 saturating. |
| Movement | Cardinal-only greedy x-reduce, then y-reduce, then fallback up/right/down/left. |
| Sourback slowdown | Scout/Forage on SourbackBitter moves only on even tick; `AntGroupSlowed` once per command. |
| Worker loss | Forage true non-Sourback -> Sourback entry only; `min(3, workers)`; confidence `saturating_sub(16)`. |
| Hashing | Canonical bytes for cells + AntGroup + tick index; no UI or `CommandFlags`. |
| Receipts | CommandReceipt exists with hashes, deltas, perception updates, summary, debug stats. |
| Render boundary | RenderFrame / ColonyView exists; default hides hidden semantics. |
| Tests | 27 reported passing after Slice 2 correction. |

## Missing before playable slice
| Missing area | Why it matters | Required pass |
|---|---|---|
| Optimization Pass 0 harness | Needed before adding dynamic systems to keep replay/hash proof cheap. | Pass 0 |
| Collapse | Makes material substrate behave and validates support/dig consequences. | Slice 3 |
| Water flow | Water currently exists as cells but does not move. | Slice 4 |
| Moisture diffusion | Needed for wet support/scent interaction and order-independent material behavior. | Slice 4 |
| Scent decay/reinforcement | Needed for visible route/carry/return feedback. | Slice 5 |
| Carcass harvest | Needed for the first actual forage/reward loop. | Slice 5 |
| Return-home deposit | Needed to close the harvest loop. | Slice 5 |
| CLI/REPL/text render | Needed to play without writing Rust tests. | Play surface pass |
| Full-run smoke tests | Needed to prove the playable loop is deterministic. | Acceptance pass |

## Non-goals that remain forbidden
- Full yard/procedural generation.
- Tauri polish as the first playable surface.
- Card-dashboard gameplay.
- A* pathfinding or route graphs.
- Combat, doctrine, economy, colony AI, full biology.
- GPU/Margolus/block CA, MCTS/MCMC, Unreal.

## Current risk notes
| Risk | Control |
|---|---|
| Agent starts Slice 3 without harness | Use `03_OPTIMIZATION_PASS_0_IMPLEMENTATION_SPEC.md` first unless Chaz explicitly defers it. |
| Agent reopens Sourback semantics | Frozen in Slice 2; do not edit except fixing regressions. |
| Agent implements UI before commands | CLI/REPL first, grid/text view second, Tauri deferred. |
| Agent treats `source_review/` as canon | Explicitly forbidden. |
| Agent adds crates casually | Avoid dependencies; justify any addition in report. |
