# 04 — Traceability Matrix

Requirement-to-test coverage for all C0-REQ requirements.

---

## Full Traceability Table

| Requirement ID | Requirement Summary | Source File | Test ID (if present) | Needed Test (if missing) | Implementation Surface | Risk If Untested |
|---|---|---|---|---|---|---|
| C0-REQ-001 | Whole-chunk updates each tick | `03` | — | C0-TEST-PERF-1: verify all 16,384 cells touched per tick | `sim.rs` tick loop | Silent partial updates could cause nondeterminism |
| C0-REQ-002 | Normative tick order (10 steps) | `03` | C0-TEST-DET-1 (indirect) | C0-TEST-ORDER-1: change tick order, verify hash changes | `sim.rs` | Wrong order = different simulation = wrong hash |
| C0-REQ-003 | Deterministic simulation | `03` | C0-TEST-DET-1, C0-TEST-DET-2 | — | `sim.rs`, hash module | Nondeterminism = replay broken = Chunk 0 fails |
| C0-REQ-004 | Canonical serialization hash | `03` | C0-TEST-DET-1, C0-TEST-DET-2 | — | hash module | Brittle hash = false test failures |
| C0-REQ-005 | Single CPU thread performance | `01` | — | C0-TEST-PERF-2: benchmark 1000 ticks under threshold | `sim.rs` | Slow tick = unresponsive visualizer |
| C0-REQ-006 | No per-tick heap churn | `01` | — | C0-TEST-PERF-3: allocator tracking across 100 ticks | `sim.rs`, `chunk.rs` | Memory spikes on long runs |
| C0-REQ-010 | Flat 1D array 16,384 cells | `01` | C0-TEST-DET-1 (indirect) | C0-TEST-INIT-1: verify array length and index formula | `chunk.rs` | Wrong index = wrong cell access |
| C0-REQ-011 | Cell is Copy, no heap | `01` | — | C0-TEST-CELL-1: compile-time assert Copy; `size_of` test | `cell.rs` | Heap-owned Cell = performance disaster |
| C0-REQ-012 | Cell has 7 specified fields | `01` | — | C0-TEST-CELL-2: field existence and type assertion | `cell.rs` | Missing field = missing behavior |
| C0-REQ-013 | No unverified byte-size claims | `01` | — | C0-TEST-CELL-3: `size_of::<Cell>()` recorded but not asserted unless spec requires | `cell.rs` | False alignment assumptions |
| C0-REQ-014 | 9 minimum materials | `01` | C0-TEST-MAT-1 (Soil→Tunnel) | C0-TEST-ENUM-1: all 9 variants exist and are distinct | `cell.rs` | Missing material = missing behavior |
| C0-REQ-015 | Residue enum (4 variants) | `01` | C0-TEST-ANT-1 (SourbackBitter) | C0-TEST-ENUM-2: all variants exist | `cell.rs` | Missing residue variant |
| C0-REQ-016 | Flags (4 minimum bits) | `01` | — | C0-TEST-FLAGS-1: flag bits non-overlapping and functional | `cell.rs` | Flag collision = wrong state |
| C0-REQ-017 | Visual-only flags outside Cell | `01` | C0-TEST-DET-2 (indirect) | C0-TEST-HASH-1: visual flag change doesn't alter hash | `cell.rs`, `render_frame.rs` | Visual state polluting hash |
| C0-REQ-018 | Material semantics table | `01` | C0-TEST-MAT-1 through MAT-7 (partial) | C0-TEST-SEM-1: for each material, verify traversability/diggability/support/moisture/scent/harvest | `materials.rs` | Material misbehavior |
| C0-REQ-020 | Half-open coordinate convention | `02` | — | C0-TEST-COORD-1: verify coordinate ranges and index formula | `chunk.rs` | Off-by-one at boundaries |
| C0-REQ-021 | Hardcoded deterministic layout | `02` | C0-TEST-DET-1 (uses FIXTURE_INITIAL) | C0-TEST-LAYOUT-1: verify all bands against spec table | `chunk.rs` init | Wrong layout = wrong simulation |
| C0-REQ-022 | Boundary behavior | `02` | — | C0-TEST-BOUND-1: water at edge stays; ant at edge stays | `chunk.rs`, `sim.rs` | Off-screen leaks |
| C0-REQ-023 | 9-row layout band accuracy | `02` | C0-TEST-DET-1 (indirect) | C0-TEST-LAYOUT-2: spot-check each band's material/support | `chunk.rs` init | Wrong initial state |
| C0-REQ-024 | Ant group start position | `02` | — | C0-TEST-INIT-2: ant at (55,118), task=Idle | `ant.rs` | Wrong start = wrong simulation |
| C0-REQ-030 | 8 commands supported | `04` | C0-TEST-MAT-1, CMD-1 (partial) | C0-TEST-CMD-ALL: each command accepted and returns receipt | `orders.rs` | Missing command = missing capability |
| C0-REQ-031 | Command preconditions and failures | `04` | C0-TEST-CMD-1 | C0-TEST-CMD-FAIL: each failure case produces correct event | `orders.rs`, `events.rs` | Silent failure or crash |
| C0-REQ-032 | Avoid doesn't alter WorldTruth | `04` | — | C0-TEST-AVOID-1: Avoid command, verify chunk hash unchanged | `orders.rs` | Avoid corrupting simulation state |
| C0-REQ-033 | CommandReceipt fields | `03`, `05` | C0-TEST-DET-1 (produces receipt) | C0-TEST-RECEIPT-1: verify all 8 required fields present | `events.rs` | Incomplete receipt breaks replay |
| C0-REQ-040 | WorldTruth/ColonyPerception separation | `05` | C0-TEST-PERC-1, UI-3 | — | `perception.rs`, `render_frame.rs` | Hidden truth leak |
| C0-REQ-041 | WorldTruth contents | `05` | — | C0-TEST-TRUTH-1: WorldTruth contains all listed data | sim state | Incomplete truth state |
| C0-REQ-042 | ColonyPerception contents | `05` | C0-TEST-PERC-1 (partial) | C0-TEST-PERC-FULL: perception ledger contains all listed categories | `perception.rs` | Incomplete perception |
| C0-REQ-043 | Language guardrail | `05` | C0-TEST-PERC-2 | — | `perception.rs`, `events.rs` | Premature Sourback reveal |
| C0-REQ-044 | 10 event types | `05` | C0-TEST-PERC-2, PERC-3 (partial) | C0-TEST-EVENT-ALL: each event type emittable | `events.rs` | Missing event type |
| C0-REQ-045 | Event-to-perception mapping | `05` | C0-TEST-PERC-2, PERC-3 | — | `events.rs`, `perception.rs` | Wrong perception from correct events |
| C0-REQ-046 | Receipt terminology | `05` | — | C0-TEST-TERM-1: no `board_deltas` or `result_card` in API | All output structs | Terminology drift |
| C0-REQ-050 | Grid is main surface | `06` | C0-TEST-UI-1 | — | `visualizer/` | Card-dashboard regression |
| C0-REQ-051 | Tauri + Canvas default | `06` | C0-TEST-UI-1 (partial) | C0-TEST-VIS-1: canvas renders 128×128 grid | `visualizer/` | Wrong visualizer |
| C0-REQ-052 | minifb/pixels fallback | `06` | — | C0-TEST-VIS-2: fallback renders same grid | `visualizer/` | No fallback if Tauri fails |
| C0-REQ-053 | RenderFrame excludes hidden truth | `06` | C0-TEST-UI-3 | — | `render_frame.rs` | Hidden truth leak |
| C0-REQ-054 | VisibleCell forbidden fields | `06` | C0-TEST-PERC-1 (partial) | C0-TEST-VISIBLE-1: VisibleCell struct has no forbidden fields | `render_frame.rs` | Structural truth leak |
| C0-REQ-055 | ColonyView default mode | `06` | C0-TEST-UI-3 | — | `render_frame.rs`, `visualizer/` | Wrong default |
| C0-REQ-056 | DevTruth toggle mode | `06` | C0-TEST-UI-3 | — | `render_frame.rs`, `visualizer/` | No debug inspection |
| C0-REQ-057 | 6 overlay modes | `06` | — | C0-TEST-OVERLAY-1: each overlay selectable and rendering | `visualizer/` | Missing overlays |
| C0-REQ-058 | Anti-card compliance | `06` | C0-TEST-UI-1, UI-2 | — | `visualizer/` | Card-dashboard regression |
| C0-REQ-060 | 6 test fixtures | `07` | All C0-TESTs (by fixture) | — | `tests/` | Can't test without fixtures |
| C0-REQ-061 | 17 validation tests | `07` | C0-TEST-DET-1 through UI-3 | — | `tests/` | Unvalidated implementation |
| C0-REQ-062 | Manual visual smoke path | `07` | C0-TEST-UI-1 (overlaps) | — | `visualizer/`, manual | Visual defects unseen |
| C0-REQ-063 | Automated smoke path stable hash | `07` | C0-TEST-DET-1 | — | `tests/` | Nondeterminism unseen |
| C0-REQ-064 | Golden hash timing | `07` | — | — (process gate) | — | Premature hash commitment |
| C0-REQ-070 | Digging rule | `03` | C0-TEST-MAT-1 | — | `sim.rs` | Digging broken |
| C0-REQ-071 | Collapse rule | `03` | C0-TEST-MAT-2 | — | `sim.rs` | Collapse broken |
| C0-REQ-072 | Water flow rule | `03` | C0-TEST-MAT-3 | — | `sim.rs` | Water broken |
| C0-REQ-073 | Moisture diffusion rule | `03` | C0-TEST-MAT-3, MAT-4 | — | `sim.rs` | Moisture broken |
| C0-REQ-074 | Wet LooseSoil support loss | `03` | C0-TEST-MAT-2 (indirect) | C0-TEST-WET-1: verify wet LooseSoil loses support | `sim.rs` | Collapse trigger broken |
| C0-REQ-075 | Scent decay + water effect | `03` | C0-TEST-MAT-4, MAT-5 | — | `sim.rs` | Scent broken |
| C0-REQ-076 | Scent reinforcement | `03` | C0-TEST-MAT-6 | — | `sim.rs` | No trails |
| C0-REQ-077 | Residue slowdown | `03` | C0-TEST-ANT-1 | — | `sim.rs`, `ant.rs` | Residue effect missing |
| C0-REQ-078 | Worker loss | `03` | C0-TEST-ANT-2 | — | `ant.rs` | No consequence for residue |
| C0-REQ-079 | Carcass harvest | `03` | C0-TEST-MAT-7 | — | `sim.rs`, `ant.rs` | Food abstraction regression |
| C0-REQ-080 | Ant group movement | `04` | C0-TEST-ANT-1 (indirect) | C0-TEST-MOVE-1: verify greedy stepping and blocked handling | `ant.rs` | Pathfinding scope creep |
| C0-REQ-090 | Scope gate | `08` | — | Process review (no code test) | — | Feature creep |
| C0-REQ-091 | Agent stop triggers | `08` | — | Process review (no code test) | — | Unauthorized feature added |
| C0-REQ-092 | Implementation prohibition | `09`, `12` | — | Process review (no code test) | — | Premature implementation |
| C0-REQ-095 | Allowed dependencies | `06` | — | C0-TEST-DEP-1: Cargo.toml audit | `Cargo.toml` | Banned dependency introduced |
| C0-REQ-096 | Banned dependencies | `06` | — | C0-TEST-DEP-1: Cargo.toml audit | `Cargo.toml` | Scope-violating dependency |
| C0-REQ-097 | Transitive wgpu allowed | `06` | — | — (advisory) | — | False positive dep violation |

---

## Coverage Gap Summary

| Gap Type | Count | IDs |
|---|---|---|
| Requirements with existing test coverage | 34 | Most C0-REQ-070–080, 040–046, 050–058, 060–063 |
| Requirements needing new tests | 28 | C0-REQ-001, 002, 005, 006, 010–017, 020–024, 030–033, 041, 046, 051–052, 054, 057, 074, 080, 095–096 |
| Requirements that are process gates (no code test) | 5 | C0-REQ-064, 090, 091, 092, 097 |
| **Total** | **62** | — |

> **Key finding:** Nearly half the requirements lack a dedicated test. The existing C0-TEST matrix covers the most critical behavioral tests but does not cover structural properties (cell layout, coordinate correctness, enum completeness), performance characteristics, or dependency compliance. A v0.3.3 patch should add the 28 missing tests to the spec.
