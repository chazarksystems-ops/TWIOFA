# TWIOFA Chunk 0 — Build Handoff (v0.3.3 Spec)

**Status (as of this document):**  
This is a **planning handoff document only**. It does **not** authorize implementation.  
No code may be written until Chaz explicitly approves the full D1–D12 gate and build-readiness checklist (see below).

**Normative Sources Used (only):**  
- `spec_pack/` (v0.3.2 + v0.3.3 corrections)  
- `agent_mapping_pass_01/` (reconciled mapping and surface analysis)  

`source_review/` was **not** used as normative input.

---

## 1. D1-D12 Approval Checklist

Chaz must explicitly approve or revise each item before any implementation begins.

| ID | Decision | Locked Candidate | Status (Current) |
|----|----------|------------------|------------------|
| D1 | Repo target | Fresh substrate repo (not the old card-dashboard micro-yard) | ⬜ Requires explicit Chaz approval |
| D2 | Visualizer | Tauri + HTML Canvas as default; minifb or pixels as fallback | ⬜ Requires explicit Chaz approval |
| D3 | Chunk size | 128×128 cells | ⬜ Requires explicit Chaz approval |
| D4 | Cell storage | Flat `Vec<Cell>` (small, Copy, no heap-owned fields) | ⬜ Requires explicit Chaz approval |
| D5 | Ant group | Single coordinate ant group | ⬜ Requires explicit Chaz approval |
| D6 | Collapse | Deterministic threshold, no random roll | ⬜ Requires explicit Chaz approval |
| D7 | Perception scope | Perception Event Ledger only (full Field Notes UI deferred) | ⬜ Requires explicit Chaz approval |
| D8 | Tauri policy | Tauri allowed only for canvas/grid + controls; no card-dashboard gameplay | ⬜ Requires explicit Chaz approval |
| D9 | Receipt naming | `CommandReceipt`, `chunk_deltas`, `dev_event_summary` (snake_case in serialized forms) | ⬜ Requires explicit Chaz approval |
| D10 | Render boundary | `RenderFrame` / `VisibleCell`; default mode excludes hidden semantic truth | ⬜ Requires explicit Chaz approval |
| D11 | Carcass harvest | One carcass cell = one harvestable unit in Chunk 0 (no per-cell mass) | ⬜ Requires explicit Chaz approval |
| D12 | Worker loss | Forage entering SourbackBitter loses `min(3, workers)` once per command + fixed confidence penalty (-16, clamped [0,255]) | ⬜ Requires explicit Chaz approval |

**Build-Readiness Checklist**

Before any coding starts, the following must be approved by Chaz:

- D1–D12 decisions (see table above)
- The full build-readiness checklist (see spec_pack/12_OPEN_DECISIONS_FOR_REVIEW_V0_3_2.md and agent_mapping_pass_01/12_BUILD_READINESS_GATE.md)

Additional items that must also be confirmed:

- [ ] Dependency policy (allowed/banned crates) approved
- [ ] Anti-card-dashboard UI rule approved
- [ ] Implementation report template approved
- [ ] All v0.3.3 CA items (moisture double-buffer, confidence penalty, harvest target, collapse+AntGroup, boundary Stone, Tunnel support note) reflected in the implementation plan

**No implementation is authorized until Chaz explicitly approves both the D1–D12 decisions and the build-readiness checklist.**

---

## 2. Rust Module / File Plan

Recommended first-pass layout (synthesized from `spec_pack/09` + `agent_mapping_pass_01/05`):

```
src/
├── cell.rs              # Cell struct, Material/Residue/Flags enums
├── chunk.rs             # Vec<Cell> + coordinate helpers + init
├── materials.rs         # Material semantics & interaction helpers (pure)
├── sim.rs               # Tick loop + all material rules (dig, collapse, water, moisture double-buffer, scent)
├── orders.rs            # Command enum + preconditions + dispatch
├── ant.rs               # AntGroup state + movement + task logic
├── events.rs            # Event types + Perception Event Ledger
├── perception.rs        # ColonyView / DevTruth projection + language guardrails
├── render_frame.rs      # RenderFrame + VisibleCell generation (truth boundary)
├── visualizer/          # Tauri + HTML canvas (or minifb/pixels fallback)
│   └── ...
└── tests/               # Fixture-based C0-TESTs + determinism harness
```

**Ownership Rules (from agent 05):**
- `cell.rs` owns the Cell schema.
- `chunk.rs` owns the flat substrate + initialization.
- `sim.rs` owns the 10-step tick order and all WorldTruth mutations.
- `render_frame.rs` owns the truth boundary (never expose raw WorldTruth to default visualizer).
- No module may leak hidden semantic truth (e.g., exact `SourbackBitter` meaning) into default ColonyView paths.

---

## 3. Data Model Plan

**Cell (spec_pack/01 + agent 05):**
- 7 fields, `Copy + Clone`
- `material: Material` (u8-compatible enum)
- `moisture: u8`
- `scent_home: u8`
- `scent_food: u8`
- `residue: Residue` (u8-compatible)
- `support: u8`
- `flags: u8` (bitmask: RECENTLY_DUG, RECENTLY_COLLAPSED, etc.)

**Materials (minimum 9):**
Air, Soil, LooseSoil, Tunnel, Water, Carcass, Root, Stone, NestWall

**Residues (Chunk 0):** None, SourbackBitter

**Chunk:**
- `Vec<Cell>` (exactly 16,384 entries)
- `index = y * 128 + x`
- Half-open coordinates (0..128)

**AntGroup (minimum):**
- `pos: (u8, u8)`
- `workers: u32`
- `task`
- `food_carried: u32`
- `fatigue: u8`
- `confidence: u8`
- per-command flags (e.g., sourback_entered)

**WorldTruth vs ColonyPerception split** must be enforced at the RenderFrame boundary.

---

## 4. Deterministic Tick Plan

**Normative 10-step order** (spec_pack/03, locked with v0.3.3 double-buffer):

1. Apply queued command intent
2. Execute ant group movement and task
3. Execute digging / harvesting effects
4. Execute gravity and collapse (bottom-up loop)
5. Execute water flow (deterministic priority: down, down-left, down-right, left, right)
6. **Execute moisture diffusion using double-buffer**
   - Source buffer = moisture values at start of step
   - Destination buffer receives all writes
   - Process in stable order (row-major)
   - After full pass: copy/swap dest → source
   - Order of cell updates **cannot** affect results
7. Execute scent decay + moisture scent effects
8. Execute scent reinforcement from ant path / harvest (+12 movement, +24 harvest with deterministic target rule)
9. Generate perception events (WorldTruth → ColonyPerception mapping + language guardrails)
10. Produce `CommandReceipt` + `RenderFrame` data

**Determinism requirements:**
- No `HashMap` iteration in hot paths
- Canonical serialization for hashing (not raw memory)
- Fixed neighbor orders everywhere (up, right, down, left)
- No floating point, no wall time, no random

---

## 5. Fixture / Test Matrix

**Core Fixtures** (from expanded v0.3.3 `spec_pack/07`):

- `FIXTURE_INITIAL` — full 128×128 layout per spec_pack/02
- `FIXTURE_WEAK_ROOF`, `FIXTURE_COLLAPSE_RISK`
- `FIXTURE_MOISTURE_SCENT`
- `FIXTURE_RESIDUE_ROUTE`
- `FIXTURE_CARCASS_HARVEST`, `FIXTURE_HARVEST_SCENT`
- `FIXTURE_BLOCKED_DIG`, `FIXTURE_BASIC_DIG`, `FIXTURE_BOUNDARY_DIG_BLOCKED`

**Required Test Categories (minimum from spec_pack/07 + agent 05):**
- Determinism (C0-TEST-DET-*)
- Material behavior (MAT-1 through MAT-7)
- Ant behavior & worker loss (ANT-1, ANT-2)
- Perception / truth boundary (PERC-*)
- UI anti-card compliance (UI-*)
- Command failure & receipt generation

All tests must be runnable from `Reset(seed)` + command sequence and must produce stable chunk hashes on repeated runs.

---

## 6. Receipt / Hash Validation Plan

**CommandReceipt** (spec_pack/05 + 09):
- `command_id`
- `tick_start`, `tick_end`
- `chunk_hash_before`, `chunk_hash_after`
- `chunk_deltas`
- `perception_updates`
- `dev_event_summary`
- `debug_stats`

**Hashing Rule (spec_pack/03):**
- Canonical serialization of WorldTruth (material, moisture, scents, residue, support, flags, ant state, tick index)
- Must exclude all visual-only / UI-only state

**Validation:**
- Every command produces a receipt.
- Repeated identical command sequences on fresh `Reset()` must produce identical final hashes.
- `RenderFrame` changes must never affect `chunk_hash`.

---

## 7. Visualizer Boundary Plan

**Core Rule (spec_pack/06 + 05):**
- Rust owns **WorldTruth**.
- Default visualizer receives only **RenderFrame** / **VisibleCell**.
- `ColonyView` (default) must never contain hidden semantic truth.

**VisibleCell (ColonyView) may contain:**
- coord, visible_material_category, visible_overlay, known_perception_marker, recent_delta_marker

**Must never contain (unless DevTruth mode):**
- raw `residue` enum meaning
- hidden hazard identity
- unearned Sourback labels
- raw support / exact moisture (unless toggled)

**DevTruth mode** is a development-only toggle. It is not the default player experience.

**Default visualizer stack after approval:**
- Tauri + HTML Canvas (central 128×128 grid)
- Fallback: minifb or pixels

---

## 8. First Implementation Task Sequence (Smallest Safe Slice)

1. **Cell + basic Chunk** (`cell.rs`, `chunk.rs`) + deterministic initialization from spec_pack/02 bands. Unit tests for Copy, indexing, layout fidelity.
2. **Minimal tick skeleton** in `sim.rs` (empty steps 1-10) + `CommandReceipt` production.
3. **Digging rule** (step 3) + `DigTunnel` command + receipt with deltas.
4. **Basic RenderFrame + VisibleCell** generation + truth boundary enforcement (no hidden residue semantics in default mode).
5. **Determinism harness** (`Reset` + repeated command sequences + hash comparison).
6. **Worker loss + residue slowdown** (step 2) + confidence penalty (-16 clamped).
7. **Collapse rule** (step 4) + AntGroup interaction (worker-loss consequence on landing cell).
8. **Double-buffer moisture** (step 6) + acceptance table.
9. **Harvest scent target rule** + carcass harvest.
10. **Perception Event Ledger** + language guardrails.
11. **Full fixture test matrix** execution + hash stability proof.
12. **Anti-card UI compliance check** (canvas is the central surface).

Only proceed to the next slice after the previous slice passes all relevant C0-TESTs and determinism checks.

---

## 9. Explicit Non-Goals (for Chunk 0)

From `spec_pack/08` and `spec_pack/00` + REPORT:

**Hard non-goals for this implementation pass:**
- Full yard generation or procedural content
- Any card-dashboard or zone-only gameplay surface
- Unreal integration or GPU-owned simulation
- A* pathfinding or full ant AI
- Multiplayer
- Complete biology / food economy / doctrine systems
- Shader-heavy presentation
- Anything that hides the 128×128 material grid behind panels as the primary experience

Any feature that does not directly prove material cell behavior, digging, collapse, moisture, scent, residue, carcass harvest, ant-group interaction, perception split, or deterministic replay must be deferred.

---

## 10. Stop Conditions

**Implementation must stop / be paused if any of the following occur:**

- Chaz revokes or withholds approval of any D1–D12 item
- Any attempt to turn the central surface into a card dashboard or abstract zone manager
- Hidden WorldTruth (e.g., exact `SourbackBitter` semantics) leaks into default `ColonyView` / `RenderFrame`
- Determinism is lost (different hashes on identical command sequences)
- Scope creep beyond the 10-step tick + 9 materials + single ant group + perception ledger
- Any file begins treating `source_review/` as current normative input
- The implementation report (CHUNK_0_IMPLEMENTATION_REPORT.md) is not produced with all required sections

**Primary success signal:**  
A small, inspectable, deterministic 128×128 material-yard substrate that can be driven by commands, observed through a clean `RenderFrame`, and proven repeatable via hash.

---

**End of Build Handoff**

This document was synthesized strictly from `spec_pack/` and the reconciled `agent_mapping_pass_01/` files.  
It is a planning artifact only. Implementation is not authorized until Chaz explicitly approves the gate.