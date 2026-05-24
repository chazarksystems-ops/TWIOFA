# 05 — Implementation Surface Map

Future implementation surfaces mapped from the spec. No code is written; this maps purpose, data flow, ownership, and test obligations.

---

## 1. Cell Model

| Attribute | Value |
|---|---|
| **Purpose** | Define the `Cell` struct and all enums (Material, Residue, Flags) |
| **Inputs** | Spec `01`: field table, material enum, residue enum, flags, forbidden types |
| **Outputs** | `Cell` struct with Copy/Clone, Material enum, Residue enum, flag constants |
| **Owns/Reads** | Owns: cell schema definition. Read by: everything |
| **Likely module** | `cell.rs` |
| **Tests** | C0-TEST-CELL-1 (Copy assert), C0-TEST-CELL-2 (field check), C0-TEST-ENUM-1 (material variants), C0-TEST-ENUM-2 (residue variants), C0-TEST-FLAGS-1 (non-overlapping bits) |
| **Must not leak** | Nothing — Cell is a data type, not a UI surface |

---

## 2. Chunk Model

| Attribute | Value |
|---|---|
| **Purpose** | Flat `Vec<Cell>` of 16,384 cells; coordinate helpers; index formula |
| **Inputs** | Cell definition; spec `02` coordinate convention |
| **Outputs** | `Chunk` struct wrapping `Vec<Cell>`; `get(x,y)` / `set(x,y)` accessors; boundary checks |
| **Owns/Reads** | Owns: `Vec<Cell>` (the substrate). Read by: sim, ant, events, render_frame |
| **Likely module** | `chunk.rs` |
| **Tests** | C0-TEST-INIT-1 (array length), C0-TEST-COORD-1 (index formula), C0-TEST-BOUND-1 (boundary behavior) |
| **Must not leak** | Raw `Vec<Cell>` reference must not be exposed directly to visualizer; goes through RenderFrame |

---

## 3. Layout Initialization

| Attribute | Value |
|---|---|
| **Purpose** | Hardcoded deterministic initialization of the 128×128 chunk per spec `02` coordinate bands |
| **Inputs** | Spec `02` layout table; seed (stored but unused for v0.3.2) |
| **Outputs** | Initialized `Chunk` with all bands applied in spec order |
| **Owns/Reads** | Owns: initialization logic. Writes: Chunk cells |
| **Likely module** | `chunk.rs` (init function) |
| **Tests** | C0-TEST-LAYOUT-1 (all bands match spec), C0-TEST-LAYOUT-2 (spot-check), C0-TEST-INIT-2 (ant position) |
| **Must not leak** | Nothing — initialization is internal |

---

## 4. Simulation Tick Loop

| Attribute | Value |
|---|---|
| **Purpose** | Execute the normative 10-step tick order each simulation step |
| **Inputs** | Current Chunk state; queued command intent; ant group state |
| **Outputs** | Mutated Chunk state; emitted events list; updated ant group |
| **Owns/Reads** | Reads/writes: WorldTruth (Chunk + ant state). Owns: tick ordering logic |
| **Likely module** | `sim.rs` |
| **Tests** | C0-TEST-DET-1 (determinism), C0-TEST-DET-2 (hash excludes UI), C0-TEST-ORDER-1 (order sensitivity), C0-TEST-PERF-2 (performance) |
| **Must not leak** | Internal tick state must not be visible to UI; only RenderFrame is exposed |

---

## 5. Simulation Rules (Sub-systems of Tick Loop)

### 5a. Digging

| Attribute | Value |
|---|---|
| **Purpose** | Convert adjacent Soil/LooseSoil to Tunnel; reduce neighbor support |
| **Inputs** | Ant group position/task; target cell |
| **Outputs** | Cell mutation; CellDug event; neighbor support changes |
| **Likely module** | `sim.rs` (dig sub-function) |
| **Tests** | C0-TEST-MAT-1 |

### 5b. Collapse

| Attribute | Value |
|---|---|
| **Purpose** | Bottom-up loop: LooseSoil with support < 100 above Air/Tunnel swaps down |
| **Inputs** | All cells in chunk |
| **Outputs** | Cell swaps; RECENTLY_COLLAPSED flags; CollapseOccurred events |
| **Likely module** | `sim.rs` (collapse sub-function) |
| **Tests** | C0-TEST-MAT-2 |

### 5c. Water Flow

| Attribute | Value |
|---|---|
| **Purpose** | Deterministic priority movement of water cells |
| **Inputs** | All water cells (bottom-up) |
| **Outputs** | Cell swaps; moisture carried |
| **Likely module** | `sim.rs` (water sub-function) |
| **Tests** | C0-TEST-MAT-3 |

### 5d. Moisture Diffusion

| Attribute | Value |
|---|---|
| **Purpose** | Spread moisture between cells based on threshold and material acceptance |
| **Inputs** | All cells in stable order |
| **Outputs** | Moisture value changes; wet LooseSoil support reduction |
| **Likely module** | `sim.rs` (moisture sub-function) |
| **Tests** | C0-TEST-MAT-3, C0-TEST-MAT-4, C0-TEST-WET-1 |

### 5e. Scent Decay and Effects

| Attribute | Value |
|---|---|
| **Purpose** | Reduce scent values per tick; water zeroes scent; wet cells extra decay |
| **Inputs** | All cells |
| **Outputs** | Scent value changes |
| **Likely module** | `sim.rs` (scent sub-function) |
| **Tests** | C0-TEST-MAT-4, C0-TEST-MAT-5 |

### 5f. Scent Reinforcement

| Attribute | Value |
|---|---|
| **Purpose** | Increase scent from ant movement and harvest |
| **Inputs** | Ant group position, task, harvest events |
| **Outputs** | Scent value increases |
| **Likely module** | `sim.rs` (scent sub-function) |
| **Tests** | C0-TEST-MAT-6 |

---

## 6. Commands

| Attribute | Value |
|---|---|
| **Purpose** | Command enum, precondition checks, dispatch to simulation |
| **Inputs** | Player/test command payloads |
| **Outputs** | Command accepted → intent queued for tick; or CommandFailed event |
| **Owns/Reads** | Reads: ant state, chunk state (for preconditions). Writes: intent queue |
| **Likely module** | `orders.rs` |
| **Tests** | C0-TEST-CMD-1, C0-TEST-CMD-ALL, C0-TEST-CMD-FAIL |
| **Must not leak** | Commands are internal; CommandReceipt is the external output |

---

## 7. Ant Group State

| Attribute | Value |
|---|---|
| **Purpose** | Ant group position, workers, task, food_carried, fatigue, confidence, memory |
| **Inputs** | Commands; simulation tick effects (residue, harvest) |
| **Outputs** | Movement; task execution; worker loss; confidence changes |
| **Owns/Reads** | Owns: ant group state fields. Reads: chunk cells for traversability/residue |
| **Likely module** | `ant.rs` |
| **Tests** | C0-TEST-ANT-1 (slowdown), C0-TEST-ANT-2 (worker loss), C0-TEST-MOVE-1 (movement), C0-TEST-INIT-2 (start pos) |
| **Must not leak** | Internal ant state is WorldTruth; ColonyView gets filtered perception |

---

## 8. Events / Perception Ledger

| Attribute | Value |
|---|---|
| **Purpose** | Emit structured events; maintain perception event ledger; translate WorldTruth events to ColonyPerception |
| **Inputs** | Simulation events from all subsystems |
| **Outputs** | Event structs with stable IDs; perception mapping; language-guardrailed descriptions |
| **Owns/Reads** | Owns: event ledger, perception state. Reads: WorldTruth events |
| **Likely module** | `events.rs`, `perception.rs` |
| **Tests** | C0-TEST-PERC-1, C0-TEST-PERC-2, C0-TEST-PERC-3, C0-TEST-EVENT-ALL |
| **Must not leak** | Hidden semantic meanings (e.g., "SourbackBitter") must not appear in ColonyPerception before earned observation |

---

## 9. RenderFrame

| Attribute | Value |
|---|---|
| **Purpose** | Generate a derived view of WorldTruth suitable for the visualizer; enforce truth boundary |
| **Inputs** | WorldTruth (chunk + ant state); overlay mode; inspection mode (ColonyView/DevTruth) |
| **Outputs** | `RenderFrame` struct with tick_index, chunk_hash, visible_cells, overlay, deltas, perception_markers, debug_stats |
| **Owns/Reads** | Reads: WorldTruth. Owns: RenderFrame/VisibleCell generation |
| **Likely module** | `render_frame.rs` |
| **Tests** | C0-TEST-PERC-1 (hidden truth absent), C0-TEST-UI-3 (DevTruth toggle), C0-TEST-VISIBLE-1 (forbidden fields absent), C0-TEST-HASH-1 (visual state excluded from hash) |
| **Must not leak** | Hidden residue semantics, hidden hazard identity, exact collapse probability, unearned Sourback labels, raw debug fields |

---

## 10. Inspection Modes

| Attribute | Value |
|---|---|
| **Purpose** | ColonyView (default) and DevTruth (debug toggle) cell inspection |
| **Inputs** | Cell coordinates; mode toggle |
| **Outputs** | ColonyView: observed material, signs, confidence, events. DevTruth: all raw cell fields |
| **Owns/Reads** | Reads: RenderFrame (ColonyView) or WorldTruth (DevTruth) |
| **Likely module** | `render_frame.rs` or `perception.rs` |
| **Tests** | C0-TEST-UI-3 |
| **Must not leak** | DevTruth data must never appear in ColonyView mode |

---

## 11. Hashing / Deterministic Replay

| Attribute | Value |
|---|---|
| **Purpose** | Canonical serialization of simulation state; chunk hash generation |
| **Inputs** | All hash-included state: cells (7 fields each), ant group state, tick index |
| **Outputs** | Stable chunk hash |
| **Owns/Reads** | Reads: WorldTruth |
| **Likely module** | Part of `chunk.rs` or dedicated hash utility |
| **Tests** | C0-TEST-DET-1 (same inputs → same hash), C0-TEST-DET-2 (UI state excluded) |
| **Must not leak** | Hash is debug/internal data; may appear in CommandReceipt and DevTruth but is not player-facing gameplay |

---

## 12. Visualizer

| Attribute | Value |
|---|---|
| **Purpose** | Render the 128×128 grid as the central visual surface; provide overlay toggles, inspection, debug panels |
| **Inputs** | `RenderFrame` from Rust simulation |
| **Outputs** | Pixel rendering of grid; UI controls; inspection panels |
| **Owns/Reads** | Reads: RenderFrame only. Must not read WorldTruth directly |
| **Likely module** | `visualizer/` (Tauri+canvas or minifb/pixels) |
| **Tests** | C0-TEST-UI-1 (grid is central), C0-TEST-UI-2 (grid-first receipt), C0-TEST-UI-3 (DevTruth toggle), C0-TEST-VIS-1 (canvas renders), C0-TEST-OVERLAY-1 (overlays work) |
| **Must not leak** | Visualizer must never bypass RenderFrame to access WorldTruth; no hidden truth in default mode |

---

## 13. Tests

| Attribute | Value |
|---|---|
| **Purpose** | Implement all C0-TEST-* validation tests; provide fixtures |
| **Inputs** | Fixtures (FIXTURE_INITIAL, FIXTURE_WEAK_ROOF, etc.); command sequences |
| **Outputs** | Pass/fail results; chunk hashes for determinism verification |
| **Owns/Reads** | Reads: all simulation modules. Owns: test fixtures and assertions |
| **Likely module** | `tests/` directory |
| **Tests** | Self-referential — the test suite validates itself |
| **Must not leak** | Tests run headlessly; visualizer not required for test pass |

---

## 14. Debug Dumps

| Attribute | Value |
|---|---|
| **Purpose** | Dump full WorldTruth state for debugging; support DevTruth inspection |
| **Inputs** | Full simulation state |
| **Outputs** | Serialized state dump; human-readable cell inspection |
| **Likely module** | Part of `chunk.rs` or `events.rs` |
| **Tests** | No dedicated test; used during development |
| **Must not leak** | Debug dumps are developer-only; never in default player UI |

---

## 15. Implementation Report

| Attribute | Value |
|---|---|
| **Purpose** | Document what was built, how to run, test results, compliance checks |
| **Inputs** | Build results; test outputs; visual review |
| **Outputs** | `CHUNK_0_IMPLEMENTATION_REPORT.md` with 11 required sections |
| **Likely module** | Markdown file (not code) |
| **Tests** | Reviewed by Chaz; not automated |
| **Must not leak** | Nothing — this is a report |

---

## Surface Count Summary

| Surface Category | Count |
|---|---|
| Data model | 2 (cell, chunk) |
| Initialization | 1 (layout) |
| Simulation | 7 (tick loop + 6 sub-systems) |
| Commands | 1 |
| Ant group | 1 |
| Events/perception | 1 |
| RenderFrame | 1 |
| Inspection | 1 |
| Hashing | 1 |
| Visualizer | 1 |
| Tests | 1 |
| Debug | 1 |
| Report | 1 |
| **Total** | **~15 distinct surfaces** |
