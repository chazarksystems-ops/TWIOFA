# 11 — Risk Register and Scoring

| ID | Risk | Severity | Likelihood | Detection | Mitigation | Owner |
|---|---|---:|---:|---|---|---|
| R1 | Agent builds another card dashboard | High | Medium | UI review / C0-TEST-UI-1 | Central grid rule, anti-card stop trigger | Chaz + reviewer |
| R2 | WorldTruth leaks into ColonyView | High | Medium | Perception tests / cell inspect | RenderFrame/VisibleCell boundary | Implementer + reviewer |
| R3 | Spec turns into full Noita clone | High | Medium | Scope review | One chunk, no full fluid/gas/fire systems | Chaz |
| R4 | Procedural generation sneaks in | Medium | Medium | Code review | Hardcoded layout contract | Implementer |
| R5 | A* or pathfinding scope creep | Medium | Medium | Dependency/code review | Greedy deterministic movement only | Implementer |
| R6 | Hash nondeterminism | High | Medium | C0-TEST-DET-1 | Canonical serialization, stable ordering | Implementer |
| R7 | Raw memory hash causes brittle output | Medium | Medium | Code review | Ban raw memory hashing | Reviewer |
| R8 | Visualizer owns simulation logic | High | Low-Medium | Architecture review | Rust owns sim, visualizer passive | Implementer |
| R9 | Tauri DOM becomes game board | High | Medium | UI review | Canvas/grid-only central surface | Reviewer |
| R10 | Carcass harvest becomes counter-only | Medium | Medium | C0-TEST-MAT-7 | Require physical cell conversion | Implementer |
| R11 | Perception ledger becomes full archive UI too early | Medium | Medium | Scope review | Defer full Field Notes UI | Chaz |
| R12 | GPU compute added prematurely | High | Low | Dependency review | Ban GPU-owned sim/custom compute | Reviewer |
| R13 | Residue becomes full creature simulation | Medium | Medium | Spec review | Sourback as residue/pressure only | Chaz |
| R14 | Tick order changes without tests | Medium | Medium | Test failures / review | Normative tick order | Implementer |
| R15 | Too much polish before substrate works | Medium | Medium | Review | Material behavior first | Chaz + reviewer |

## Highest Priority Risks

1. R1 — card-dashboard regression.
2. R2 — WorldTruth leakage.
3. R6 — nondeterministic simulation.
4. R10 — non-physical carcass/food abstraction.

These are direct failure modes from prior work and must be guarded before implementation starts.
