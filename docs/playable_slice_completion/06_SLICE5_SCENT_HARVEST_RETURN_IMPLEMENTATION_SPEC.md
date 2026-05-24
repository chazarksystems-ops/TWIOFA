# Slice 5 — Scent, Carcass Harvest, and Return-Food Loop Spec

## Status
- **Normative status:** implementation spec for Slice 5 after Slices 3 and 4 are green.
- **Allowed target:** close the first forage loop: move, harvest one carcass cell, carry food, return home, deposit.

## What this document does not authorize
- Economy or storage system beyond a simple returned-food counter if needed.
- Full AI/pathfinding.
- Multiple ant groups.
- UI/Tauri.
- Combat/doctrine/colony simulation.

## Scent decay
Every tick:

```text
scent_home = scent_home.saturating_sub(1)
scent_food = scent_food.saturating_sub(1)
```

If material is Water:

```text
scent_home = 0
scent_food = 0
```

If moisture > 100:

```text
scent_home = scent_home.saturating_sub(2)
scent_food = scent_food.saturating_sub(2)
```

Water zeroing should dominate the wet-cell extra effect.

## Scent reinforcement
When AntGroup moves through a traversable cell:
- If task is `ReturnHome`, `scent_home = min(255, scent_home + 12)`.
- If task is `Forage` and carrying/near food, `scent_food = min(255, scent_food + 12)`.

For Chunk 0, define “near food” conservatively:
- `food_carried > 0` OR adjacent to Carcass in up/right/down/left order.
- Document whichever exact condition is implemented.

## Carcass harvest
If task is `Forage` and ant is adjacent to a Carcass cell:
- Select one adjacent Carcass in fixed order up, right, down, left.
- Convert that Carcass cell to Air.
- Set or preserve support according to `Material::Air` default support 0.
- `food_carried += 1`.
- Emit `CarcassHarvested`.
- Set `Flags::HARVESTED` if useful for traceability; if the material becomes Air, ensure flag use does not imply remaining carcass.
- No harvesting through soil/solid material.

## Harvest scent target
When harvest succeeds:
- If AntGroup current cell is traversable, apply `scent_food = min(255, scent_food + 24)` to current cell.
- Otherwise select first adjacent traversable cell in order up, right, down, left and apply +24 there only.
- Clamp `[0,255]`.
- Scent update itself does not require perception.

## Return-home deposit
When task is `ReturnHome` and AntGroup reaches `HOME_COORD` or the home/nest tunnel cell:
- If `food_carried > 0`, deposit all carried food into a simple authoritative field.
- If no existing field exists, add `food_returned: u32` either to `AntGroup` or `Simulation` and include it in canonical hashing.
- Preferred: `Simulation.food_returned: u32` as WorldTruth colony state, serialized in `compute_chunk_hash` after ant state.
- Emit a simple event such as `FoodDeposited { amount }` only if adding event is clean; otherwise document in `dev_event_summary`.
- Set `food_carried = 0`.
- Do not implement economy/storage beyond this counter.

## Files likely touched
| File | Expected action |
|---|---|
| `src/sim.rs` | Tick steps 7 and 8; harvest in step 3; return deposit. |
| `src/events.rs` | Existing `CarcassHarvested`; optional `FoodDeposited`. |
| `src/perception.rs` | Harvest/deposit wording. |
| `src/ant.rs` or `src/sim.rs` | Returned food field if needed. |
| `src/cell.rs` | Use existing `Flags::HARVESTED` if helpful. |
| Tests | Scent/harvest/return tests. |
| Reports/harness | Add slice status/corpus scripts. |

## Required tests
| Test | Expected |
|---|---|
| `test_scent_base_decay` | seeded scent decays by 1. |
| `test_water_zeroes_scent` | Water cell scent becomes 0. |
| `test_wet_cell_extra_scent_decay` | moisture >100 applies extra decay. |
| `test_return_home_reinforces_home_scent` | ReturnHome movement adds +12 home scent. |
| `test_forage_near_food_reinforces_food_scent` | Forage near/carrying food adds +12 food scent. |
| `test_carcass_harvest_converts_one_cell_to_air` | exactly one adjacent carcass removed. |
| `test_carcass_harvest_fixed_order` | up/right/down/left selection. |
| `test_harvest_increments_food_carried` | `food_carried += 1`. |
| `test_harvest_scent_current_cell_if_traversable` | current cell +24. |
| `test_harvest_scent_adjacent_fallback_if_current_not_traversable` | first adjacent traversable +24. |
| `test_return_home_deposits_food` | carried food becomes returned/deposited count. |
| `test_full_forage_return_loop_deterministic` | repeat same command sequence, same hash. |

## Harness corpus additions
- `SCENT_DECAY`
- `SCENT_WATER_ERASURE`
- `CARCASS_HARVEST`
- `HARVEST_SCENT`
- `RETURN_HOME_DEPOSIT`
- `FULL_FORAGE_RETURN_LOOP`

## Acceptance criteria
- All prior tests pass.
- New Slice 5 tests pass.
- Full forage-return loop works through public commands.
- Receipts include harvest and deposit evidence.
- No hidden Sourback label leaks into default ColonyView.
- No economy/doctrine/combat invented.
