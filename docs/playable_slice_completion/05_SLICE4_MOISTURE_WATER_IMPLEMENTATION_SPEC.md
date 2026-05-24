# Slice 4 — Water Flow and Moisture Implementation Spec

## Status
- **Normative status:** implementation spec for Slice 4 after Slice 3 passes.
- **Allowed target:** deterministic water flow, double-buffer moisture diffusion, and wet LooseSoil support decay.

## What this document does not authorize
- Scent reinforcement/harvest.
- UI/Tauri.
- Active chunks.
- GPU/Margolus/block CA.
- Procedural yard.

## Water flow rule
Use bottom-up order for water cells. For each water cell, try destinations in this exact priority:
1. down
2. down-left
3. down-right
4. left
5. right

Move only into `Air` or `Tunnel`. The water cell swaps with the destination and carries moisture value with it.

## Moisture double-buffer rule
- Source buffer = moisture values at start of step 6.
- Destination buffer = separate array initialized to source values.
- Read only from source.
- Write only to destination.
- Stable order row-major is acceptable, but final result must not depend on update order.
- If `source[cell] > source[neighbor] + 16`, transfer 4 if neighbor accepts moisture.
- After full pass, copy/swap destination back to cell moisture fields.

## Important implementation note
Naive pairwise writes can double-apply transfers if each neighbor pair is processed twice. To avoid ambiguity, choose and document one deterministic method:
- Preferred: process directed neighbors in fixed order up/right/down/left and apply all directed transfers to a delta buffer, clamped at `[0,255]`, with source conservation bounded by available moisture.
- Alternative: process each unordered neighbor pair once using right/down only, simpler but must be documented as the chosen Chunk 0 interpretation.

Use the method that is easiest to test and explain. Do not use floating point.

## Materials accepting moisture
Accepts moisture:
- Air
- Tunnel
- Soil
- LooseSoil
- Water

Does not accept moisture:
- Carcass
- Root
- Stone
- NestWall

## Wet LooseSoil support decay
If a `LooseSoil` cell has `moisture > 120`, then each tick:

```text
support = support.saturating_sub(1)
```

This happens during moisture step after moisture values are updated.

## Files likely touched
| File | Expected action |
|---|---|
| `src/sim.rs` | Implement tick step 5 water, step 6 moisture, wet support decay. |
| `src/materials.rs` | Use or refine `accepts_moisture`. |
| `tests/scaffold_tests.rs` or new file | Add water/moisture tests. |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Append Slice 4 results. |
| Harness | Add corpus scripts if pass 0 exists. |

## Required tests
| Test | Setup | Expected |
|---|---|---|
| `test_water_flows_down_first` | Water above Air/Tunnel | Water moves down. |
| `test_water_priority_order` | Down blocked; down-left available | Moves down-left before other directions. |
| `test_water_does_not_enter_solid_materials` | Water beside Stone/Root/Soil/etc. | No illegal move. |
| `test_water_carries_moisture_on_swap` | Water moisture 255 | Destination Water has 255 after swap. |
| `test_moisture_double_buffer_order_independent` | symmetric fixture | repeated runs identical; no in-place drift. |
| `test_moisture_does_not_enter_non_accepting_materials` | Carcass/Root/Stone/NestWall neighbor | unchanged. |
| `test_wet_loose_soil_support_decays` | LooseSoil moisture 121 | support decreases by 1. |
| `test_dry_loose_soil_support_does_not_decay` | LooseSoil moisture 120 | no decay. |
| `test_moisture_hash_determinism` | same setup x3 | identical hash. |

## Harness corpus additions
- `WATER_FLOW_DOWN`
- `WATER_FLOW_PRIORITY`
- `MOISTURE_DIFFUSION_BASIC`
- `WET_LOOSESOIL_DECAY`

## Acceptance criteria
- All prior tests pass.
- New water/moisture tests pass.
- Harness corpus still deterministic.
- No scent/harvest/UI code added in this pass.
- Report says Slice 4 completed but Slice 5 still gated.
