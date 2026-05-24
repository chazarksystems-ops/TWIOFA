# Chunk 0 — Playable/Testable Slice Report

## Final Status

```
Chunk 0 playable/testable slice is complete as a deterministic local scaffold.
This does not lock full-game canon and does not authorize expansion beyond Chunk 0
without Chaz approval.
```

---

## Validation Commands Run

From `chunk0_rust_scaffold/`:

```text
cargo fmt --check                                              PASS
cargo check                                                    PASS (0 warnings)
cargo test                                                     57 passed / 0 failed
cargo run --bin chunk0_harness -- run-corpus                   12/12 PASS
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100   100/100 PASS
cargo run --bin chunk0_cli -- run-script smoke                 SMOKE_DETERMINISM: PASS
```

---

## Files Changed in This Session

### New files
| File | Description |
|---|---|
| `src/harness.rs` | Invariant checker, 12-script corpus, stress-local-fixtures |
| `src/bin/chunk0_harness.rs` | Harness CLI binary |
| `src/bin/chunk0_cli.rs` | Interactive REPL + smoke script |
| `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | Optimization Pass 0 report |
| `CHUNK_0_PLAYABLE_SLICE_REPORT.md` | This file |

### Modified files
| File | Changes |
|---|---|
| `src/sim.rs` | Added food_returned, tick steps 3–8 (collapse, water, moisture, scent, harvest, deposit) |
| `src/events.rs` | Added FoodDeposited variant |
| `src/perception.rs` | Added FoodDeposited translation |
| `src/lib.rs` | Added harness module export |
| `Cargo.toml` | Added chunk0_harness and chunk0_cli binary targets |
| `tests/scaffold_tests.rs` | 30 new tests (57 total) |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Appended Slices 3–5 + Opt Pass 0 + CLI sections |

---

## Tests Added (30 new tests, 57 total)

**Slice 3 — Collapse (9 tests)**
- test_loose_soil_collapses_down_into_air
- test_loose_soil_collapses_down_into_tunnel
- test_loose_soil_does_not_collapse_with_support_100_or_more
- test_loose_soil_does_not_collapse_into_soil_or_stone
- test_collapse_scan_is_bottom_up_deterministic
- test_collapse_into_antgroup_applies_worker_loss
- test_boundary_ring_not_changed_by_collapse
- test_collapse_receipt_contains_delta_and_perception
- test_collapse_hash_determinism

**Slice 4 — Water/Moisture (9 tests)**
- test_water_flows_down_first
- test_water_priority_order
- test_water_does_not_enter_solid_materials
- test_water_carries_moisture_on_swap
- test_moisture_double_buffer_order_independent
- test_moisture_does_not_enter_non_accepting_materials
- test_wet_loose_soil_support_decays
- test_dry_loose_soil_support_does_not_decay
- test_moisture_hash_determinism

**Slice 5 — Scent/Harvest/Return (12 tests)**
- test_scent_base_decay
- test_water_zeroes_scent
- test_wet_cell_extra_scent_decay
- test_return_home_reinforces_home_scent
- test_forage_near_food_reinforces_food_scent
- test_carcass_harvest_converts_one_cell_to_air
- test_carcass_harvest_fixed_order
- test_harvest_increments_food_carried
- test_harvest_scent_current_cell_if_traversable
- test_harvest_scent_adjacent_fallback_if_current_not_traversable
- test_return_home_deposits_food
- test_full_forage_return_loop_deterministic

---

## Functional Acceptance Matrix

| Area | Covered by | Result |
|---|---|---|
| Grid 128x128 / 16384 cells | test_cell_count, check_invariants | PASS |
| Boundary Stone ring unchanged | test_boundary_ring_*, harness invariant | PASS |
| Dig | test_basic_dig_*, BASIC_DIG corpus | PASS |
| Movement cardinal/deterministic | test_cardinal_movement_*, test_greedy_* | PASS |
| Sourback slowdown + entry loss + guardrail | Slice 2 tests (10 tests) | PASS |
| Collapse bottom-up + ant impact | Slice 3 tests | PASS |
| Water priority flow | test_water_* | PASS |
| Moisture double-buffer diffusion | test_moisture_* | PASS |
| Wet LooseSoil support decay | test_wet_loose_soil_support_decays | PASS |
| Scent decay + water/wet effects | test_scent_*, test_water_zeroes_scent | PASS |
| Harvest one carcass deterministically | test_carcass_harvest_* | PASS |
| Return + deposit food | test_return_home_deposits_food | PASS |
| Receipt with hashes | test_replay_determinism + all receipt checks | PASS |
| Hash determinism | test_*_hash_determinism + smoke determinism | PASS |
| Colony view hides hidden truth | test_render_frame_colony_view_residue_boundary, harness RENDERFRAME_BOUNDARY | PASS |
| CLI / REPL | smoke script PASS, manual play instructions below | PASS |

---

## Required Final Tests — Coverage Map

| Required test | Where covered |
|---|---|
| full_run_seeded_yard_smoke_test | `run-script smoke` (SMOKE_DETERMINISM: PASS) |
| scout_residue_then_forage_then_return | smoke script steps 4–7; test_full_forage_return_loop_deterministic |
| dig_then_collapse_receipt | BASIC_DIG corpus + test_collapse_receipt_contains_delta_and_perception |
| water_moisture_support_interaction | test_wet_loose_soil_support_decays + test_moisture_hash_determinism |
| harvest_one_carcass_and_return_food | test_carcass_harvest_converts_one_cell_to_air + test_return_home_deposits_food |
| renderframe_no_truth_leak_after_full_run | test_render_frame_colony_view_residue_boundary + harness RENDERFRAME_BOUNDARY |
| repeated_full_run_hash_sequence_stable | test_full_forage_return_loop_deterministic + SMOKE_DETERMINISM: PASS |

---

## Manual Play Instructions

**Requirements:** Rust toolchain (cargo)

**Start REPL:**
```bash
cd chunk0_rust_scaffold
cargo run --bin chunk0_cli
```

**REPL commands:**
```
> reset                    # reset to initial state
> render colony            # 48x24 viewport around ant
> render devtruth          # same viewport with raw debug info
> step 5                   # advance 5 ticks
> scout 84 28              # send scouts toward residue band
> forage 95 22             # send foragers toward carcass region
> inspect 85 30 colony     # inspect cell (colony-safe view)
> inspect 85 30 devtruth   # inspect cell (full debug view)
> return-home              # start return journey
> hash                     # print canonical hash
> receipt                  # print full state summary
> quit                     # exit REPL
```

**Run smoke script (complete demo):**
```bash
cargo run --bin chunk0_cli -- run-script smoke
```

**Render full 128x128 map to file:**
```bash
cargo run --bin chunk0_cli -- render-text colony > map_colony.txt
cargo run --bin chunk0_cli -- render-text devtruth > map_dev.txt
```

**Map symbols:**
| Symbol | Meaning |
|---|---|
| `#` | Stone / boundary |
| `.` | Air |
| `s` | Soil |
| `l` | LooseSoil |
| `t` | Tunnel |
| `w` | Water |
| `c` | Carcass |
| `r` | Root |
| `n` | NestWall |
| `A` | AntGroup position |
| `?` | Perceived bitter/yellow residue (colony mode) |
| `!` | SourbackBitter residue (devtruth mode only) |

---

## Known Deferred Items

Authorized to remain deferred (spec_pack/08 + acceptance matrix):
- Full yard / procedural generation
- Tauri polished UI
- Multi-ant groups
- Doctrine / economy / combat
- A* pathfinding
- GPU / Margolus / MCTS / MCMC
- Full save/load (in-memory only for this pass)

---

## Scope Boundary Assurances

**`source_review/` not normative:** `source_review/` was not consulted as a normative input at any point. All decisions derive from `spec_pack/`, `BUILD_HANDOFF_V0_3_3.md`, and `docs/playable_slice_completion/`.

**Card-dashboard not introduced:** No card-based gameplay, no dashboard panels, no card-driven mechanics were added. The CLI is a text-only REPL.

**No new dependencies:** The only third-party dependency remains `sha2 = "0.10"` (unchanged from initial scaffold).

**No floating point in authoritative state:** All simulation state uses integer types (u8, u32, u64). No f32/f64 in WorldTruth.

**No HashMap in deterministic simulation paths:** Only Vec and array iteration is used in tick steps.

**No wall time in simulation logic:** tick_index is an explicit u32 counter; no std::time calls.

**No random/nondeterminism in simulation:** The LCG in stress_local_fixtures is test-harness-only and not part of the simulation engine.
