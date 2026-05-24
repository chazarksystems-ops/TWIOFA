# Slice 3 — Deterministic Collapse Implementation Spec

## Status
- **Normative status:** implementation spec for Slice 3 if and only if Chaz approves this phase.
- **Allowed target:** deterministic bottom-up gravity/collapse and AntGroup landing interaction.

## What this document does not authorize
- Moisture diffusion.
- Water flow.
- Scent.
- Carcass harvest.
- UI/Tauri.
- Pathfinding/procedural yard.
- Health/combat systems.

## Source rule
From `spec_pack/03`: scan `y` from 126 down to 0; scan `x` from 0 to 127. If a cell is `LooseSoil` and `support < 100`, and the cell below is `Air` or `Tunnel`, swap cell with below, set moved cell `RECENTLY_COLLAPSED`, emit `CollapseOccurred`. No random roll.

## Implementation constraints
- Use staged deltas or an explicit single-pass deterministic procedure.
- Preserve bottom-up order exactly.
- Do not let a cell collapse twice in the same tick unless the spec is amended. Preferred implementation: one source cell evaluated once per tick in the bottom-up scan.
- Do not collapse boundary Stone.
- Collapse only `LooseSoil` in this pass.
- Destination is only `Air` or `Tunnel`.
- Material swap must carry the falling cell’s moisture/scent/residue/support/flags unless a test proves otherwise; the emptied source receives the destination cell’s previous state from the swap.

## AntGroup interaction
If the destination cell of the collapse swap is the AntGroup’s current cell:
- Apply existing worker-loss consequence: `lost = min(3, workers)`.
- `workers -= lost`.
- `confidence = confidence.saturating_sub(16)`.
- Emit `WorkerLoss`.
- Emit `CollapseOccurred`.
- Do not invent health, damage types, combat, animation, or displacement.
- Material swap still occurs.

## Files likely touched
| File | Expected action |
|---|---|
| `src/sim.rs` | Implement tick step 4 collapse after digging and before water. |
| `src/events.rs` | Existing `CollapseOccurred` is likely sufficient; avoid extra event unless necessary. |
| `src/perception.rs` | Ensure CollapseOccurred and WorkerLoss translate safely. |
| `tests/scaffold_tests.rs` or new collapse test file | Add deterministic collapse tests. |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Append Slice 3 results. |
| `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | If harness exists, update corpus status. |

## Required tests
| Test | Setup | Expected |
|---|---|---|
| `test_loose_soil_collapses_down_into_tunnel` | LooseSoil support 40 above Tunnel | Swap; falling cell flagged RECENTLY_COLLAPSED; event emitted. |
| `test_loose_soil_does_not_collapse_with_support_100_or_more` | LooseSoil support 100 above Tunnel | No move. |
| `test_loose_soil_does_not_collapse_into_soil_or_stone` | LooseSoil support 40 above Soil/Stone | No move. |
| `test_collapse_scan_is_bottom_up_deterministic` | Two possible collapses in one column | Same result across repeated runs. |
| `test_collapse_into_antgroup_applies_worker_loss` | Fixture from spec_pack/07 COLLAPSE_RISK | lost=min(3,workers), confidence -=16, WorkerLoss + CollapseOccurred. |
| `test_boundary_ring_not_changed_by_collapse` | collapse near boundary | Boundary Stone remains unchanged. |
| `test_collapse_receipt_contains_delta_and_perception` | collapse command/tick | receipt summary includes collapse. |
| `test_collapse_hash_determinism` | same setup/commands x3 | identical hash sequence. |

## Harness corpus additions
If Optimization Pass 0 exists, add:
- `COLLAPSE_WEAK_ROOF`
- `COLLAPSE_NO_SUPPORT_CHANGE_ABOVE_THRESHOLD`
- `COLLAPSE_ANTGROUP_IMPACT`

## Acceptance criteria
- All old 27 tests still pass.
- New collapse tests pass.
- Harness corpus still deterministic if implemented.
- No moisture, scent, harvest, or UI code added.
- Report says Slice 3 completed but Slice 4 still gated.
