# 02 — Requirement Index

All requirements extracted from the v0.3.2 spec pack. IDs are stable across mapping passes.

---

## Simulation Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-001 | Whole-chunk updates required; dirty-cell tracking deferred | `03` L4–5 | Simulation | Full 16,384-cell iteration each tick | Low |
| C0-REQ-002 | Normative tick order must be followed exactly (10 steps) | `03` L8–21 | Simulation | Tick order matches spec; tests fail if order changes | High |
| C0-REQ-003 | Simulation must be deterministic: fixed seed, stable iteration, no wall-clock, no HashMap iteration, no float nondeterminism | `03` L24–43 | Simulation | Same commands → same chunk hash across runs | High |
| C0-REQ-004 | Chunk hash uses canonical serialization, not raw memory bytes | `03` L46–73 | Simulation | Hash includes all spec fields; excludes UI state; stable across runs | High |
| C0-REQ-005 | One full tick must run comfortably on a single CPU thread | `01` L120 | Simulation | Sub-millisecond tick for 16,384 cells | Low |
| C0-REQ-006 | No per-tick heap churn after initialization | `01` L121 | Simulation | No alloc/dealloc in hot path; profiler confirms | Medium |

## Cell Model Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-010 | Chunk is a flat 1D array of 16,384 cells (128×128) | `01` L5–11 | Cell Model | `Vec<Cell>` length == 16384; index == y*128+x | Low |
| C0-REQ-011 | Cell must be small, copyable, no heap allocation | `01` L13–19 | Cell Model | Cell implements Copy; no String/Vec/Box/HashMap/Rc/Arc | Low |
| C0-REQ-012 | Cell contains 7 expected fields: material, moisture, scent_home, scent_food, residue, support, flags | `01` L24–32 | Cell Model | All fields present and typed as specified | Low |
| C0-REQ-013 | Do not claim exact byte size unless verified by test | `01` L34 | Cell Model | No hardcoded size assertion without `size_of::<Cell>()` test | Low |
| C0-REQ-014 | Material enum must include at minimum 9 types: Air, Soil, LooseSoil, Tunnel, Water, Carcass, Root, Stone, NestWall | `01` L49–61 | Cell Model | All 9 variants present | Low |
| C0-REQ-015 | Residue enum must include at minimum: None, SourbackBitter, Rot (placeholder), Alarm (placeholder) | `01` L82–88 | Cell Model | All variants present; only SourbackBitter has behavior in Chunk 0 | Low |
| C0-REQ-016 | Flags must include at minimum: RECENTLY_DUG, RECENTLY_COLLAPSED, OBSERVED_THIS_TICK (optional), HARVESTED (optional) | `01` L94–101 | Cell Model | Bit flags defined; hash-included flags are in canonical state | Low |
| C0-REQ-017 | Visual-only flags must be stored outside Cell | `01` L103 | Cell Model | No visual-only data inside Cell struct if it would affect hash | Medium |
| C0-REQ-018 | Material semantics table must be respected (traversability, diggability, support, moisture, scent, harvest, render) | `01` L63–76 | Cell Model | Each material behaves according to table | Medium |

## Layout Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-020 | Coordinate convention: x 0..128 left-to-right, y 0..128 top-to-bottom, half-open ranges | `02` L4–18 | Layout | Index formula matches; ranges are half-open | Low |
| C0-REQ-021 | Initial layout is hardcoded deterministically; procedural generation banned | `02` L20–24 | Layout | No RNG in layout init; seed stored but unused | Low |
| C0-REQ-022 | Boundary behavior: x=0, x=127, y=0, y=127 solid; y=124..128 stone; no wrapping; no off-screen flow/exit | `02` L28–32 | Layout | Boundary cells are stone or treated as impassable; water/ants cannot leave | Medium |
| C0-REQ-023 | Exact coordinate bands must match the 9-row layout table | `02` L36–49 | Layout | Each band initialized to specified material/support/residue | Medium |
| C0-REQ-024 | Ant group starts at x=55, y=118 in Idle state | `02` L47 | Layout | Ant group position and task match at tick 0 | Low |

## Command Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-030 | 8 commands supported: StepSimulation, DigTunnel, SendForagers, ScoutResidue, ReturnHome, Avoid, InspectCell, Reset | `04` L74–83 | Commands | All 8 commands accepted and processed | Medium |
| C0-REQ-031 | Each command has specified preconditions; violations emit CommandFailed | `04` L87–94 | Commands | Precondition checks match table; failure events emitted | Medium |
| C0-REQ-032 | Avoid does not alter WorldTruth; only changes ColonyPerception/policy | `04` L96–117 | Commands | No material/residue/water changes from Avoid | Medium |
| C0-REQ-033 | Every command produces a CommandReceipt with specified fields | `03` L257–269, `05` L89–101 | Commands | Receipt contains command_id, tick_start, tick_end, chunk_hash_before/after, chunk_deltas, perception_updates, dev_event_summary, debug_stats | Medium |

## Event and Perception Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-040 | WorldTruth and ColonyPerception are strictly separated | `05` L4–8 | Perception | No hidden semantic data in ColonyPerception without earned observation | High |
| C0-REQ-041 | WorldTruth includes: cells, ant group state, tick index, seed/RNG state, hidden semantics, command history | `05` L10–21 | Perception | All listed data in WorldTruth | Low |
| C0-REQ-042 | ColonyPerception includes: observed facts, suspected hazards, confidence, interpretations, misreads, assignment policy | `05` L24–32 | Perception | Perception Event Ledger tracks these | Medium |
| C0-REQ-043 | Language guardrail: "bitter/yellow residue" before earned observation; "Sourback-associated" only after confirmation event | `05` L40–53 | Perception | No "Sourback" label in ColonyView before earning | High |
| C0-REQ-044 | 10 minimum event types with stable ordering and stable IDs | `05` L57–72 | Events | All event types implemented with required fields | Medium |
| C0-REQ-045 | Event-to-perception mapping must follow the specified translation table | `05` L76–83 | Events | WorldTruth events correctly map to ColonyPerception outputs | High |
| C0-REQ-046 | CommandReceipt uses `chunk_deltas`, `command_receipt`, `dev_event_summary`; avoids `board_deltas`, `result_card` | `05` L103–119 | Events | No banned terminology in API or output | Medium |

## Visualizer Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-050 | Material grid/canvas is the main surface; debug panels support it | `06` L4–9 | Visualizer | Grid is central, dominant visual element | High |
| C0-REQ-051 | Default visualizer: Rust sim + Tauri + HTML Canvas | `06` L13–24 | Visualizer | Canvas renders 128×128 grid; HTML limited to controls/debug | Medium |
| C0-REQ-052 | Fallback visualizer: minifb or pixels | `06` L28–30 | Visualizer | Fallback can render same grid without Tauri | Medium |
| C0-REQ-053 | RenderFrame excludes hidden semantic truth unless DevTruth mode | `06` L34–48 | Visualizer | Default RenderFrame contains no hidden residue meanings | High |
| C0-REQ-054 | VisibleCell must not include hidden residue semantics, hazard identity, exact collapse probability, unearned Sourback labels, raw debug fields | `06` L52–71 | Visualizer | VisibleCell struct has no forbidden fields in default mode | High |
| C0-REQ-055 | ColonyView mode (default): shows only colony-accessible information | `06` L75–87 | Visualizer | Hover/click shows observed material, signs, confidence, events | Medium |
| C0-REQ-056 | DevTruth mode (explicit toggle): may expose raw WorldTruth | `06` L89–106 | Visualizer | Toggle shows all cell fields including hidden semantics | Low |
| C0-REQ-057 | 6 overlay modes required: Material, Moisture, Scent, Residue, Support, Perception | `06` L110–119 | Visualizer | All 6 overlays selectable and rendering correct data | Medium |
| C0-REQ-058 | Anti-card UI compliance: build fails if central element is cards/logs/buttons | `06` L122–138 | Visualizer | Automated or manual check confirms grid-first layout | High |

## Testing Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-060 | 6 test fixtures must be provided | `07` L5–12 | Testing | All fixtures loadable and producing expected initial state | Medium |
| C0-REQ-061 | 17 validation tests (C0-TEST-*) must pass | `07` L16–35 | Testing | All tests implemented and passing | High |
| C0-REQ-062 | Manual visual smoke path (10 steps) must pass | `07` L39–48 | Testing | Human reviewer confirms all 10 visual checks | Medium |
| C0-REQ-063 | Automated smoke path must produce stable chunk hash | `07` L52–75 | Testing | Command sequence produces identical hash across runs | High |
| C0-REQ-064 | Golden hash generated only after first approved deterministic implementation | `07` L75 | Testing | No premature golden hash commitment | Low |

## Simulation Rule Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-070 | Digging: adjacent Soil/LooseSoil → Tunnel; support reduced by 50 for cardinal neighbors; CellDug event | `03` L78–90 | Simulation | Dig produces Tunnel, sets RECENTLY_DUG, reduces neighbor support, emits event | Medium |
| C0-REQ-071 | Collapse: bottom-up loop; LooseSoil with support < 100 above Air/Tunnel swaps down; deterministic, no random | `03` L94–108 | Simulation | Collapse matches pseudocode exactly; RECENTLY_COLLAPSED set | Medium |
| C0-REQ-072 | Water flow: bottom-up, deterministic priority (down, down-left, down-right, left, right); swaps into Air/Tunnel only | `03` L112–127 | Simulation | Water moves according to priority; carries moisture | Medium |
| C0-REQ-073 | Moisture diffusion: stable order; transfer 4 if delta > 16; only to moisture-accepting materials | `03` L130–156 | Simulation | Diffusion matches pseudocode; respects material acceptance table | Medium |
| C0-REQ-074 | Wet LooseSoil (moisture > 120) loses 1 support per tick | `03` L158–163 | Simulation | Support decreases by 1 per tick when wet | Low |
| C0-REQ-075 | Scent decay: both scents decrease by 1 per tick; Water zeroes both; wet cells (moisture > 100) reduce by extra 2 | `03` L167–188 | Simulation | Scent values match decay rules | Medium |
| C0-REQ-076 | Scent reinforcement: +12 from ant movement (ReturnHome → home scent, Forage → food scent); +24 on harvest | `03` L192–205 | Simulation | Scent increases match spec values | Medium |
| C0-REQ-077 | SourbackBitter slowdown: movement only on even tick_index during Scout/Forage; AntGroupSlowed event | `03` L217–222 | Simulation | Ant moves only on even ticks in residue; event emitted | Medium |
| C0-REQ-078 | Worker loss: Forage + first entry into SourbackBitter during command → lose min(3, workers); confidence penalty; WorkerLoss event | `03` L226–232 | Simulation | Worker count decreases; confidence drops; event emitted | Medium |
| C0-REQ-079 | Carcass harvest: one cell = one unit; adjacent Carcass → Air; food_carried +1; deterministic target order (up, right, down, left) | `03` L238–253 | Simulation | Cell converts to Air; food increments; CarcassHarvested event | Medium |
| C0-REQ-080 | Ant group movement: greedy target stepping; traversable = Air/Tunnel/Water(slow); not through Soil/LooseSoil/Carcass/Root/Stone/NestWall | `04` L46–70 | Simulation | Movement follows priority; blocked → CommandFailed or Idle | Medium |

## Scope Gate Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-090 | Feature must prove at least one of 11 listed substrate capabilities to belong in Chunk 0 | `08` L29–45 | Scope Gate | No feature added that doesn't prove a listed capability | Medium |
| C0-REQ-091 | Agent must halt and request approval before adding scope-expanding features | `08` L98–111 | Agent Boundary | Stop trigger fires before implementation | High |
| C0-REQ-092 | Implementation prohibited until Chaz approves build-readiness gate | `09` L115, `12` L47 | Agent Boundary | No code generation before explicit approval | Critical |

## Dependency Requirements

| ID | Requirement | Source | Category | Acceptance Criteria | Risk |
|---|---|---|---|---|---|
| C0-REQ-095 | Allowed deps: Rust, serde, deterministic RNG, Tauri+canvas (default), minifb/pixels (fallback) | `06` L142–149 | Dependency | Only allowed dependencies in Cargo.toml | Medium |
| C0-REQ-096 | Banned deps: Unreal, GPU compute, Bevy, procedural gen, A*, multiplayer, card-dashboard | `06` L153–163 | Dependency | No banned dependency in project | High |
| C0-REQ-097 | `pixels` may use `wgpu` internally; ban is on custom GPU compute, not transitive rendering | `06` L165 | Dependency | Clear distinction between rendering and compute | Low |

---

## Requirement Summary

| Category | Count |
|---|---|
| Simulation | 6 |
| Cell Model | 9 |
| Layout | 5 |
| Commands | 4 |
| Events / Perception | 7 |
| Visualizer | 9 |
| Testing | 5 |
| Simulation Rules | 11 |
| Scope Gate / Agent Boundary | 3 |
| Dependency | 3 |
| **Total** | **62** |
