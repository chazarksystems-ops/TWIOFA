# Slice 2 Freeze Report — AntGroup Movement and Sourback Semantics

## Status
- **Normative status:** Elevated guidance that records the corrected Slice 2 behavior. It supports, but does not replace, `spec_pack/`, `BUILD_HANDOFF_V0_3_3.md`, and `CHUNK_0_IMPLEMENTATION_REPORT.md`.
- **Slice 2 semantic correction:** Completed.
- **Reported validation:** `cargo fmt --check: OK`; `cargo check: OK`; `cargo test: OK`; `27 passed / 0 failed`.
- **Freeze-ready condition:** Slice 2 is freeze-ready only under the semantics documented here.
- **Slice 3:** Still gated pending explicit Chaz approval.

## What this document does not authorize
- Does not authorize Slice 3.
- Does not authorize Optimization Pass 0 implementation.
- Does not authorize collapse, moisture, scent, harvest, Tauri/UI, procedural yard, pathfinding, economy, doctrine, combat, GPU/Unreal, MCTS/MCMC, Margolus/block CA, or card-dashboard behavior.

## Files changed by semantic correction
- `chunk0_rust_scaffold/src/sim.rs`
- `chunk0_rust_scaffold/tests/scaffold_tests.rs`
- `CHUNK_0_IMPLEMENTATION_REPORT.md`

## Locked Slice 2 behavior
| Behavior | Status | Rule |
|---|---|---|
| Cardinal-only movement | LOCKED FOR CHUNK 0 | No diagonal movement. Movement vectors are up, right, down, left only. |
| Greedy stepping | LOCKED FOR CHUNK 0 | 1. reduce x if possible; 2. reduce y if possible; 3. fallback up, right, down, left. |
| Traversability | LOCKED FOR CHUNK 0 | Movement only into traversable materials: Air, Tunnel, Water. |
| Blocked movement | LOCKED FOR CHUNK 0 | If no traversable cardinal neighbor is available, emit `CommandFailed(Blocked)` once per command. |
| Sourback slowdown | LOCKED FOR CHUNK 0 | Scout/Forage standing on SourbackBitter skips movement on odd ticks; `AntGroupSlowed` emits once per command. |
| WorkerLoss | LOCKED FOR CHUNK 0 | Forage only; true transition entry only; once per command. |

## Exact WorkerLoss rule
WorkerLoss triggers only when all conditions are true:

- previous/current cell residue != `SourbackBitter`
- destination cell residue == `SourbackBitter`
- task == `Forage`
- command scratch flag `sourback_entered == false`

Arithmetic:

- `lost = min(3, workers)`
- `workers -= lost`
- `confidence = confidence.saturating_sub(16)`

Rejected/corrected assumptions:

- **CORRECTED AGENT ASSUMPTION:** Destination-only Sourback detection is wrong.
- **LOCKED FOR CHUNK 0:** non-Sourback -> Sourback triggers WorkerLoss.
- **LOCKED FOR CHUNK 0:** Sourback -> Sourback does not trigger WorkerLoss.
- **LOCKED FOR CHUNK 0:** Starting already on Sourback does not trigger WorkerLoss by itself.
- **LOCKED FOR CHUNK 0:** Leaving and later re-entering Sourback can trigger WorkerLoss once per command.

## Worked examples
| Example | Result |
|---|---|
| non-Sourback -> Sourback | WorkerLoss fires once if task is Forage and `sourback_entered == false`. |
| Sourback -> Sourback | No WorkerLoss. This is remaining inside the residue zone, not entry. |
| starting on Sourback, odd tick | `AntGroupSlowed` fires once for the command; no WorkerLoss. |
| non-Sourback -> Sourback -> Sourback in one `StepSimulation` command | One WorkerLoss total. |
| workers=2 entering Sourback | `lost=2`, `workers=0`. |
| confidence=10 entering Sourback | `confidence=0` via saturating subtraction. |

## CommandScratch / CommandFlags
- **PROMOTED PATTERN:** Command scratch state should be explicit and named.
- Current implementation name: `CommandFlags`.
- Command-local flags include `sourback_entered`, `slowed_emitted`, and `move_blocked_emitted`.
- They reset exactly once at command start.
- They are excluded from canonical hash.
- This is acceptable because current save/replay semantics are command-boundary/post-command.
- If mid-command save/resume is required later, these fields must become serializable authoritative state or the command must be replayed from its boundary.

## Tests proving semantics
New semantic-correction tests:

- `test_non_residue_to_sourback_triggers_worker_loss`
- `test_sourback_to_sourback_does_not_trigger_worker_loss`
- `test_starting_on_sourback_odd_tick_slows_without_worker_loss`
- `test_leaves_sourback_then_reenters_triggers_worker_loss_once`
- `test_worker_loss_only_once_per_command_after_true_entry`
- `test_step_simulation_n_is_one_command_for_flags`
- `test_blocked_movement_emits_blocked_exactly_once`
- `test_greedy_tie_test_uses_traversable_fixture`

Prior scaffold and Slice 2 tests are still reported as passing. Current reported total: `27 passed / 0 failed`.

## Deferred after Slice 2
- collapse
- moisture
- scent
- harvest
- Tauri/UI
- procedural yard
- pathfinding
- economy/doctrine/combat
- GPU/Unreal
- MCTS/MCMC
- Margolus/block CA
- card-dashboard behavior

## Freeze statement
Slice 2 may be treated as frozen only for the behavior documented here. This does not authorize Slice 3.
