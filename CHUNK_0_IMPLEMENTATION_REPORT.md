# Chunk 0 — Rust Substrate Scaffold Implementation Report

This report summarizes the creation of the deterministic Chunk 0 Rust substrate scaffold matching the `BUILD_HANDOFF_V0_3_3.md` specifications.

---

## 1. Scaffold Location
The scaffold is implemented as a clean, standalone library crate located at:
`chunk0_rust_scaffold/`

---

## 2. Files Created/Changed
All files are located within the `chunk0_rust_scaffold/` project directory:
- `Cargo.toml`: Package configuration and minimal dependencies
- `src/lib.rs`: Declarations and public exports for all modules
- `src/cell.rs`: `Cell` schema (exactly 7 fields, 0-heap dependencies) and associated enums (`Material`, `Residue`, `Flags`)
- `src/materials.rs`: Pure helper functions for querying material properties
- `src/chunk.rs`: Flat `Vec<Cell>` container (128x128 = 16,384 cells), stable 1D index mapping (`y * 128 + x`), and seeded/default hardcoded coordinate band initialization overrides
- `src/ant.rs`: `AntGroup` state representation and task memory
- `src/orders.rs`: `Command` payload variants and `CommandReceipt` structure
- `src/sim.rs`: 10-step tick simulation loop, deterministic state hashing, and the first safe behavior slice (digging + immutable boundary stone block)
- `src/events.rs`: Core structured event enum and perception event ledger
- `src/perception.rs`: Objective-to-subjective perception description translators with strict language guardrails (yellow/bitter residue instead of unearned Sourback labels)
- `src/render_frame.rs`: Derived visualizer projections (`RenderFrame` and `VisibleCell`) enforcing epistemic boundaries
- `tests/scaffold_tests.rs`: Direct verification test suite containing all required tests

---

## 3. Implemented Features
- **Deterministic Substrate Spine**: Setup of 128x128 half-open spatial coordinates mapping onto flat `Vec<Cell>`.
- **Stone Boundary Ring**: Immutable, indestructible Stone boundaries at `x=0, x=127, y=0, y=127` that block digging attempts and emit blocked receipts.
- **First Safe Behavioral Slice**:
  - `Reset` restores initial hardcoded layout bands.
  - Adjacent digging of `Soil` or `LooseSoil` is restricted strictly to **cardinal adjacency** (4-way orthogonal neighbors). This is required as there is no diagonal movement or diagonal neighbor interaction (e.g. `spec_pack/03_SIMULATION_RULES_V0_3_2.md` Carcass harvest rules and movement directions are cardinally ordered: `up, right, down, left`).
  - Digging converts target to `Tunnel`, sets `flags |= RECENTLY_DUG`, defaults support to `0`, and propagates a `-50` support reduction to adjacent cardinal cells. This degradation is explicitly required by `spec_pack/03_SIMULATION_RULES_V0_3_2.md` (lines 85-86: `for each cardinal neighbor: neighbor.support = saturating_sub(neighbor.support, 50)`).
  - Proper receipts containing `chunk_deltas`, `perception_updates`, `dev_event_summary`, and `debug_stats` are produced.
- **Canonical Replay Hashing**: Custom canonical byte serialization mapping only game state fields (excluding UI/render-only elements) through SHA-256 for a completely deterministic signature.
- **Epistemic boundary**: A default `ColonyView` that guardrails residue labels and hides internal parameters (`support`, `moisture`, exact enum variant names) from the visualizer, while allowing full inspection in dedicated `DevTruth` debug mode.

---

## 4. Intentionally Deferred / Non-Goals
The following systems are intentionally deferred for subsequent development slices to ensure zero scope creep:
- Moisture diffusion & wet LooseSoil decay (deferred/out-of-scope)
- Structural collapse loops (deferred/out-of-scope)
- Scent decay and water flow (deferred/out-of-scope)
- Carcass harvest (deferred/out-of-scope)
- Tauri integrations & HTML Canvas UI panels
- Procedural yard generation, A* pathfinding, economy, doctrine, combat, GPU, Unreal integration, and card-driven mechanics

Note: AntGroup movement, greedy stepping, SourbackBitter slowdown, and worker loss rules were implemented in Slice 2 (see Section 9).

---

## 5. Dependency Justification
Per constraints, dependencies are restricted to the bare minimum:
- **`sha2`**: Used for deterministic canonical state hashing using SHA-256 (neutral, standard, architecture-independent).
- **No `serde`**: Avoided to limit compilation size/overhead; state serialization is implemented manually via byte buffer writing (`write_canonical_bytes`).
- **No `hex`**: State hashes are formatted into lowercase hexadecimal manually without introducing third-party packages.

---

## 6. Deviations from `BUILD_HANDOFF_V0_3_3.md`
No deviations have been introduced. The implementation perfectly conforms to the planning state:
- Tunnel support defaults to `0` as locked.
- Flat `Vec<Cell>` of size 16,384 is used.
- Explicit Stone boundaries are enforced.
- Replay is completely deterministic.

---

## 7. Verification Test Results
Tests are located in `tests/scaffold_tests.rs`. This section preserves the original Slice 1 scaffold result; the current full suite result after Slice 2 is documented in Section 9.4 as **27 passed / 0 failed**. The original scaffold-era 7-test run passed cleanly:
```text
running 7 tests
test test_stable_index_mapping ... ok
test test_boundary_ring_cells_are_stone ... ok
test test_cell_count ... ok
test test_boundary_dig_blocked_and_receipted ... ok
test test_basic_dig_to_tunnel_with_support_changes ... ok
test test_render_frame_colony_view_residue_boundary ... ok
test test_replay_determinism ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

---

## 8. Remaining Next Slices
- **Slice 2**: ✅ Implemented (see Section 9 below).
- **Slice 3**: Implement deterministic bottom-up gravity/collapse rules and AntGroup landing interaction.
- **Slice 4**: Implement double-buffer moisture diffusion and wet LooseSoil support decay.
- **Slice 5**: Implement Scent reinforcement, carcass harvesting, and decay.

---

## 9. Slice 2 — AntGroup Movement, Greedy Stepping, Residue Slowdown, Worker Loss

### 9.1 Implemented Features
- **Tick step 2 — AntGroup movement (`Simulation::execute_movement_step`)** runs at the start of each tick for `Scout`, `Forage`, and `ReturnHome` tasks. `Dig` is resolved in step 3 unchanged; `Avoid` and `Idle` do not move (spec_pack/04 — Avoid must not alter WorldTruth).
- **Cardinal-only greedy stepping** per `spec_pack/04_ANT_GROUP_AND_ORDERS_V0_3_2.md` "Movement Rule":
  1. Reduce x distance if possible (preferred direction).
  2. Then reduce y distance if possible.
  3. Then try the fixed fallback order `up, right, down, left`.
  4. If no traversable cardinal neighbor exists, emit `CommandFailed(Blocked)` exactly once per command.
  Movement is permitted only into `Air`, `Tunnel`, or `Water` (via `materials::is_traversable`). No diagonal motion is possible by construction. No A*, no pathfinding expansion, no random choice, no floating point.
- **SourbackBitter movement slowdown** per `spec_pack/03_SIMULATION_RULES_V0_3_2.md` "Residue Effects": when standing on `Residue::SourbackBitter` with task `Scout` or `Forage`, the ant may move only when `tick_index` is even. On the first odd-tick skip during a command, exactly one `AntGroupSlowed` event is emitted (`slowed_emitted` per-command flag prevents duplicates).
- **Worker loss rule — transition-based entry** per `spec_pack/03` and D12 of `BUILD_HANDOFF_V0_3_3.md`:
  WorkerLoss triggers only on a **true non-SourbackBitter → SourbackBitter transition**. Moving between two SourbackBitter cells does NOT count as entry. Specifically:
  - `was_on_sourback = current_cell.residue == SourbackBitter` (captured before movement, using the already-computed `on_sourback` variable).
  - `dest_on_sourback = destination_cell.residue == SourbackBitter`.
  - `entered_sourback = !was_on_sourback && dest_on_sourback`.
  - Fire only when: `is_forage && entered_sourback && !cmd_flags.sourback_entered`.
  When fired: `lost = min(3, workers)`; `workers -= lost`; `confidence = confidence.saturating_sub(16)` (clamped to `[0, 255]` by Rust saturating arithmetic, never wraps); `WorkerLoss` emitted; `sourback_entered = true` for the rest of the command.
  Starting a command already on SourbackBitter means `was_on_sourback = true` from the outset; any subsequent sourback→sourback move has `entered_sourback = false` and cannot fire.
- **Per-command transient flags (`CommandFlags`)** on `Simulation`: `sourback_entered`, `slowed_emitted`, `move_blocked_emitted`. Reset at the start of every command (including `Reset`). **Intentionally excluded from `compute_chunk_hash`**: the hash covers stable post-command WorldTruth; these flags are command-local scaffolding. If future mid-command save/resume is required, CommandFlags would need to become serializable state.
- **`HOME_COORD = (55, 118)`** in `src/ant.rs` — deterministic ReturnHome target matching the AntGroup's default spawn inside the nest band.

### 9.2 Files Changed (Slice 2 + Semantic Correction)
- `src/sim.rs` — added `CommandFlags` struct and `Simulation.cmd_flags`; reset flags at command start; added `Simulation::execute_movement_step` as tick step 2; corrected worker-loss check from destination-only to transition-based (`!on_sourback && dest_on_sourback`).
- `src/ant.rs` — added `HOME_COORD` constant.
- `tests/scaffold_tests.rs` — 20 Slice 2 tests (12 original + 8 semantic-correction tests).

### 9.3 Tests (Slice 2 — 20 tests total)
All tests in `tests/scaffold_tests.rs`. Total suite: **27 tests** (7 scaffold + 20 Slice 2).

**Original 12 (movement / events / determinism):**
1. `test_cardinal_movement_along_row`
2. `test_no_diagonal_movement_x_first`
3. `test_movement_into_non_traversable_blocked_and_receipted`
4. `test_greedy_deterministic_tie_x_first`
5. `test_sourback_bitter_movement_slowdown`
6. `test_worker_loss_and_confidence_penalty`
7. `test_confidence_penalty_saturates_at_zero`
8. `test_worker_loss_min3_with_small_workforce`
9. `test_worker_loss_only_once_per_command`
10. `test_receipt_includes_worker_loss_perception_update`
11. `test_slice2_repeated_command_sequence_hash_stable`
12. `test_return_home_uses_greedy_cardinal_stepping`

**8 semantic-correction tests (transition-based entry proof):**
- `test_non_residue_to_sourback_triggers_worker_loss` (REQ-1): non-residue→SourbackBitter transition fires loss.
- `test_sourback_to_sourback_does_not_trigger_worker_loss` (REQ-2): SourbackBitter→SourbackBitter move does NOT fire loss. ← primary correctness test.
- `test_starting_on_sourback_odd_tick_slows_without_worker_loss` (REQ-3): starting on sourback, odd tick → slowed, no loss.
- `test_leaves_sourback_then_reenters_triggers_worker_loss_once` (REQ-4): custom fixture (10,10)→(10,11)→(10,12), exit then re-entry within one command fires exactly one loss.
- `test_worker_loss_only_once_per_command_after_true_entry` (REQ-5): single `StepSimulation` crossing many sourback cells → exactly 1 loss, Δworkers ≤ 3.
- `test_step_simulation_n_is_one_command_for_flags` (REQ-8): `StepSimulation(10)` → loss=1, slowed≤1, blocked=0.
- `test_blocked_movement_emits_blocked_exactly_once` (REQ-9): unit-isolation fixture, `StepSimulation(3)` while fully blocked → exactly 1 Blocked event.
- `test_greedy_tie_test_uses_traversable_fixture` (REQ-10): explicit traversability assertions before the tie test.

### 9.4 Validation Results
```text
cargo fmt --check : OK (no diff)
cargo check       : OK (0 warnings, 0 errors)
cargo test        : 27 passed; 0 failed; 0 ignored
  - 7 pre-existing scaffold tests still green
  - 20 Slice 2 tests green (12 original + 8 semantic-correction)
```

### 9.5 Dependency Changes
None. `sha2` remains the only third-party dependency.

### 9.6 Deviations from v0.3.3 Handoff / Spec
None.
- `HOME_COORD = (55, 118)` — spec_pack/04 names "nest/tunnel/home scent" without an exact cell. (55, 118) is the AntGroup default spawn inside the nest band; keeps Chunk 0 fully deterministic without scent-driven routing (deferred).
- `CommandFlags` excluded from `compute_chunk_hash` — spec_pack/03 hashes "ant group state" (WorldTruth). Per-command transient flags are not WorldTruth. Excluding them prevents hash drift across command boundaries. If mid-command resume is ever required, they would need serialization.
- Transition-based `entered_sourback = !was_on_sourback && dest_on_sourback` — the spec says "ant enters SourbackBitter for the first time during that command." Entry requires a crossing from non-residue to residue; sourback→sourback movement is not entry. The original code only checked the destination. This pass corrects that.

### 9.7 Slice 2 Frozen
**Frozen.** All 27 tests pass. No out-of-scope features were introduced.

---

## 10. Slices 3–5 + Optimization Pass 0 + CLI

### 10.1 New Files
- `src/harness.rs` — invariant checker, corpus runner, stress-local-fixtures runner
- `src/bin/chunk0_harness.rs` — CLI binary for harness corpus and stress modes
- `src/bin/chunk0_cli.rs` — interactive REPL and one-shot CLI for manual play

### 10.2 Files Modified (Slices 3–5 + Opt Pass 0)
- `src/sim.rs` — added `food_returned: u32` (WorldTruth), `CommandFlags`, tick steps 3–8 (collapse, water flow, moisture, scent decay, scent reinforcement, carcass harvest, return deposit), `compute_chunk_hash` now includes `food_returned`
- `src/events.rs` — added `FoodDeposited { amount: u32 }` variant
- `src/perception.rs` — added `FoodDeposited` translation
- `src/lib.rs` — added `pub mod harness`
- `Cargo.toml` — added two `[[bin]]` targets
- `tests/scaffold_tests.rs` — 30 new tests (57 total)

### 10.3 Slice 3 — Deterministic Bottom-Up Collapse
`execute_collapse_step` scans y=126..0 bottom-up, x=0..127. LooseSoil with support < 100 above Air or Tunnel swaps down. Moves are collected first, then re-verified before executing. AntGroup impact: if the ant is at the destination cell, applies `min(3, workers)` worker loss and `confidence.saturating_sub(16)`. Sets `RECENTLY_COLLAPSED` flag.

Tests: 9 collapse tests covering basic fall (Air/Tunnel), support threshold, scan order, ant impact, boundary immutability, delta/perception in receipt, and hash determinism.

### 10.4 Slice 4 — Water Flow + Moisture Diffusion
`execute_water_flow_step`: bottom-up scan; priority `[(0,1),(−1,1),(1,1),(−1,0),(1,0)]` (down, down-left, down-right, left, right). Water swaps with Air or Tunnel only. Re-checks cell is still Water before moving (handles earlier displacements).

`execute_moisture_step`: double-buffer with source snapshot. Processes directed pairs (right, then down) for each cell — each unordered pair visited exactly once. Transfer: 4 units if source[a] > source[b]+16 and b accepts moisture. Wet LooseSoil decay: moisture > 120 → support.saturating_sub(1), applied after copying dest back.

Tests: 9 moisture/water tests covering flow direction, priority, solid blocking, moisture carry, double-buffer determinism, non-accepting materials, wet/dry support decay, hash determinism.

### 10.5 Slice 5 — Scent, Carcass Harvest, Return Deposit
`execute_scent_decay_step`: Water cells zero both scent fields. All others: base −1 both, then −2 more if moisture > 100.

`execute_scent_reinforcement_step`: ReturnHome → scent_home += 12 at current cell. Forage (food_carried > 0 OR adjacent Carcass) → scent_food += 12 at current cell.

`execute_harvest_step`: fixed order up/right/down/left. First adjacent Carcass → material=Air, support=0, HARVESTED flag, food_carried += 1, emit CarcassHarvested. Scent +24 at current cell if traversable, else first adjacent traversable.

Return deposit: after movement in each tick, if task=ReturnHome and pos==(55,118) and food_carried > 0, deposits food_carried into food_returned, zeroes food_carried, emits FoodDeposited.

Tests: 12 scent/harvest/return tests.

### 10.6 Optimization Pass 0 Harness
`src/harness.rs` provides `check_invariants`, `run_corpus` (12 scripts), and `stress_local_fixtures` (deterministic LCG, 4 fixture variants).

`src/bin/chunk0_harness.rs` writes CSV/text results to `optimization_runs/latest/`.

### 10.7 CLI / REPL
`src/bin/chunk0_cli.rs` provides:
- REPL: reset, step, dig, scout, forage, return-home, inspect, render, hash, receipt, quit
- One-shot: `reset`, `run-script smoke`, `render-text [colony|devtruth]`, `repl`
- `render colony`: 48×24 viewport around ant, `?` for bitter/yellow residue, `A` for ant
- `render devtruth`: same viewport, `!` for SourbackBitter (opt-in)
- Smoke script: 8-step deterministic demo covering dig, scout, forage, harvest, return, deposit

### 10.8 Validation Results (Final)
```text
cargo fmt --check      : PASS (clean)
cargo check            : PASS (0 warnings, 0 errors)
cargo test             : 57 passed / 0 failed
cargo run --bin chunk0_harness -- run-corpus
  : ALL PASSED (12/12)
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
  : ALL PASSED (100/100)
cargo run --bin chunk0_cli -- run-script smoke
  : SMOKE_DETERMINISM: PASS
```

### 10.9 Known Deferred Items
All non-Chunk-0 features remain deferred: Tauri UI, procedural yard generation, A* pathfinding, multi-ant groups, doctrine/economy/combat, GPU/Margolus/MCTS/MCMC, full save/load.

### 10.10 Source Review Non-Normative Assurance
`source_review/` was not used as normative input at any point. All implementation decisions derive from `spec_pack/`, `BUILD_HANDOFF_V0_3_3.md`, and `docs/playable_slice_completion/`.

### 10.11 Card-Dashboard Non-Introduction Assurance
No card-dashboard gameplay mechanics, no dashboard UI, no card-driven mechanics were introduced. The CLI is a text-only REPL with no UI panels.
