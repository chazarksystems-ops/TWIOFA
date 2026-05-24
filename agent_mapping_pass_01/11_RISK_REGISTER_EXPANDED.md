# 11 — Expanded Risk Register

Comprehensive risk register for Chunk 0, expanding on the spec's R1–R15 and adding new risks identified during this mapping pass.

---

## Risk Table

| Risk ID | Risk | Severity | Likelihood | Detection Method | Mitigation | Owner | Status |
|---|---|---|---|---|---|---|---|
| R1 | Agent builds another card dashboard | High | Medium | C0-TEST-UI-1; visual review; anti-card checklist | Central grid rule; agent stop triggers; compliance checklist in `08_VISUALIZER_AND_UI_COMPLIANCE_MAP.md` | Chaz + Reviewer | Open |
| R2 | WorldTruth leaks into default ColonyView | High | Medium | C0-TEST-PERC-1, C0-TEST-UI-3; structural VisibleCell review | RenderFrame/VisibleCell boundary; language guardrails; perception mapping table | Implementer + Reviewer | Open |
| R3 | Spec turns into full Noita clone | High | Medium | Scope review; feature count audit | One chunk, 9 materials, simple rules; scope gate in spec `08` | Chaz | Open |
| R4 | Procedural generation sneaks in | Medium | Medium | Code review; layout initialization test | Hardcoded layout contract; ban in spec `08`; C0-TEST-LAYOUT-1 | Implementer | Open |
| R5 | A* or pathfinding scope creep | Medium | Medium | Dependency review; code review | Greedy deterministic movement; ban in spec `06`/`08` | Implementer | Open |
| R6 | Hash nondeterminism | High | Medium | C0-TEST-DET-1 (3× replay); cross-run comparison | Canonical serialization; stable ordering; integer math; BTreeMap if needed | Implementer | Open |
| R7 | Raw memory hash causes brittle output | Medium | Medium | Code review; hash stability test | Ban raw memory hashing; require canonical serialization | Reviewer | Open |
| R8 | Visualizer owns simulation logic | High | Low-Medium | Architecture review; module boundary check | Rust owns sim; visualizer passive; RenderFrame is read-only | Implementer | Open |
| R9 | Tauri DOM becomes game board | High | Medium | C0-TEST-UI-1; UI review | Canvas/grid-only central surface; Tauri policy spec `06` | Reviewer | Open |
| R10 | Carcass harvest becomes counter-only (no physical cell change) | Medium | Medium | C0-TEST-MAT-7 | Require physical cell conversion (Carcass → Air) | Implementer | Open |
| R11 | Perception ledger becomes full archive UI too early | Medium | Medium | Scope review; feature count | Defer full Field Notes UI; scope gate | Chaz | Open |
| R12 | GPU compute added prematurely | High | Low | Dependency review; Cargo.toml audit | Ban GPU-owned sim/custom compute; spec `06` | Reviewer | Open |
| R13 | Residue becomes full creature simulation | Medium | Medium | Spec review; event type count | Sourback as residue/pressure only; spec `01`/`08` | Chaz | Open |
| R14 | Tick order changes without tests | Medium | Medium | Test failures; code review; C0-TEST-DET-1 | Normative tick order in spec `03`; test verifies hash | Implementer | Open |
| R15 | Too much polish before substrate works | Medium | Medium | Review; task priority check | Material behavior first; defer cosmetics | Chaz + Reviewer | Open |
| R16 | Implementation before Chaz approval | Critical | Medium | Process gate; build-readiness checklist | Spec `09`/`12` prohibition; agent stop triggers; build-readiness gate | Chaz + Spec | Open |
| R17 | Overbuilding Tauri complexity | Medium | High | Implementation time audit; LOC count for visualizer vs sim | Tauri is D2 candidate; fallback to minifb/pixels available; research suggests minifb is simpler | Chaz (via D2 decision) | Open |
| R18 | Hidden truth leak via event descriptions | Medium | Medium | C0-TEST-PERC-2; string grep for "Sourback" in ColonyView output | Language guardrail table; event-to-perception mapping | Implementer | Open |
| R19 | Nondeterminism from moisture diffusion buffer choice | Medium | High | C0-TEST-DET-1 (would catch different hashes) | Resolve CA-007: lock to single-buffer or double-buffer | Spec | Open — **blocked on CA-007** |
| R20 | Incomplete test coverage (28 requirements lack dedicated tests) | Medium | High | Traceability matrix audit (this document) | Add missing tests in v0.3.3 spec patch | Spec | Open |
| R21 | Collapse kills or buries ant group (undefined behavior) | Medium | Medium | CA-011 resolution; simulation edge case test | Resolve CA-011: define collapse + ant interaction | Spec | Open — **blocked on CA-011** |
| R22 | Confidence values unspecified (CA-008) | Low | High | Implementation divergence between agents | Resolve CA-008: specify exact confidence penalty values | Spec | Open — **blocked on CA-008** |
| R23 | Scent harvest location ambiguous (CA-009) | Low | High | Hash divergence between implementations | Resolve CA-009: specify exact harvest scent cell | Spec | Open — **blocked on CA-009** |
| R24 | Unbounded feature creep beyond 11 scope-gate topics | Medium | Medium | Feature count; scope gate review | Scope gate in spec `08`; agent stop triggers | Chaz + Reviewer | Open |
| R25 | Research overfitting — spec changes too much based on external research | Medium | Low | Review of spec changes against research notes | Research is support notes only, not canon; no spec changes without Chaz approval | Chaz | Open |
| R26 | Visualizer becomes the game (polish takes over) | High | Medium | Time tracking; feature review | Substrate behavior first; visualizer is proof mechanism, not product | Chaz + Reviewer | Open |
| R27 | Double-processing bug in water flow or moisture | Medium | Medium | Determinism test; manual state inspection | Specify cell processing order and mark-as-processed strategy | Implementer | Open |
| R28 | Ant group greedy movement gets permanently stuck | Low | Medium | Pathfinding test; edge case fixture | Greedy movement has fixed fallback order; CommandFailed(Blocked) emitted | Implementer | Open |

---

## Risk Priority Matrix

| | Low Likelihood | Medium Likelihood | High Likelihood |
|---|---|---|---|
| **Critical Severity** | — | R16 (impl before approval) | — |
| **High Severity** | R12 (GPU compute) | R1 (card dash), R2 (truth leak), R3 (Noita clone), R6 (nondeterminism), R8 (viz owns sim), R9 (Tauri DOM), R26 (viz=game) | — |
| **Medium Severity** | R25 (research overfit) | R4 (procgen), R5 (A*), R7 (raw hash), R10 (counter-only), R11 (full archive), R13 (creature sim), R14 (tick order), R15 (polish), R24 (creep), R27 (double-process) | R17 (Tauri complexity), R19 (buffer choice), R20 (test gaps), R22 (confidence), R23 (harvest scent) |
| **Low Severity** | — | R21 (collapse+ant), R28 (stuck movement) | — |

---

## Top 5 Priority Risks

1. **R16 — Implementation before Chaz approval** (Critical/Medium). Direct violation of the spec's core gate. All other risks are moot if implementation starts prematurely.
2. **R1 — Card-dashboard regression** (High/Medium). Known failure mode from prior work. The single highest-consequence implementation error.
3. **R2 — WorldTruth leaks into ColonyView** (High/Medium). Breaks the epistemic model that is TWIOFA's unique design contribution.
4. **R6 — Hash nondeterminism** (High/Medium). Makes replay, testing, and debugging impossible.
5. **R20 — Incomplete test coverage** (Medium/High). 28 requirements lack dedicated tests. Gaps invite undetected bugs.

---

## Spec-Blocked Risks

These risks cannot be fully mitigated until spec issues are resolved:

| Risk | Blocking Issue | Resolution |
|---|---|---|
| R19 | CA-007 (moisture buffer choice) | Lock to single-buffer or double-buffer |
| R21 | CA-011 (collapse + ant group) | Define interaction rule |
| R22 | CA-008 (confidence penalty) | Specify numeric values |
| R23 | CA-009 (harvest scent location) | Specify exact cell |
