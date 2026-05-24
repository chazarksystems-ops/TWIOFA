# 03 — Decision Register

All decisions found in the v0.3.2 spec, plus additional implicit decisions identified during this mapping pass.

---

## Explicit Decisions (D1–D12)

| ID | Topic | Locked Decision | Status | Affected Files | Affected Tests | Impact If Wrong |
|---|---|---|---|---|---|---|
| D1 | Repo target | Fresh substrate repo, not the old card-dashboard micro-yard repo | Needs Chaz Approval | `00`, `08`, `09`, `12` | All (repo determines project structure) | Wrong repo contaminates Chunk 0 with legacy card-dashboard code; migration cost later |
| D2 | Visualizer | Tauri + HTML Canvas as default; minifb or pixels as fallback | Needs Chaz Approval | `06`, `08`, `09`, `12` | C0-TEST-UI-1, UI-2, UI-3 | Wrong visualizer choice wastes weeks; Tauri complexity may delay substrate proof |
| D3 | Chunk size | 128×128 cells | Needs Chaz Approval | `00`, `01`, `02`, `06`, `07` | All MAT tests, DET tests, UI tests | Smaller chunk may miss emergent behavior; larger chunk may slow iteration |
| D4 | Cell storage | Flat `Vec<Cell>` first pass; small copyable cell; no heap-owned fields | Needs Chaz Approval | `01`, `03`, `09` | All simulation tests | Wrong storage model causes heap churn or cache misses |
| D5 | Ant group | Single coordinate ant group representing the colony effort front | Needs Chaz Approval | `04`, `07` | C0-TEST-ANT-1, ANT-2 | If wrong, ant model needs complete rewrite for multi-entity |
| D6 | Collapse | Deterministic threshold, no random collapse roll | Needs Chaz Approval | `03`, `07` | C0-TEST-MAT-2 | Random collapse breaks determinism guarantee |
| D7 | Field Notes scope | Perception Event Ledger only; full Field Notes UI deferred | Needs Chaz Approval | `05`, `07`, `09` | C0-TEST-PERC-1, PERC-2, PERC-3 | Building full Field Notes now violates scope gate |
| D8 | Tauri policy | Tauri allowed only as canvas/grid visualizer + controls; no card-dashboard gameplay | Needs Chaz Approval | `06`, `08`, `09` | C0-TEST-UI-1, UI-2 | Tauri DOM becoming game surface = card-dashboard regression |
| D9 | Receipt naming | `CommandReceipt`, `chunk_deltas`, `dev_event_summary` (not `board_deltas`, `result_card`) | Needs Chaz Approval | `05` | All command tests | Terminology drift confuses implementation and review |
| D10 | Render boundary | `RenderFrame` / `VisibleCell`; default excludes hidden semantic truth | Needs Chaz Approval | `06` | C0-TEST-PERC-1, UI-3 | Hidden truth leaks to player if boundary wrong |
| D11 | Carcass harvest | One carcass cell = one harvestable unit; no per-cell mass field | Needs Chaz Approval | `01`, `03` | C0-TEST-MAT-7 | Per-cell mass adds hidden complexity; one-cell model simpler but less nuanced |
| D12 | Worker loss | Forage entering SourbackBitter loses `min(3, workers)` once per command | Needs Chaz Approval | `03`, `04` | C0-TEST-ANT-2 | Wrong loss formula makes residue too harsh or too mild |

---

## Implicit Decisions Found During Mapping

| ID | Topic | Locked Decision | Status | Affected Files | Impact If Wrong |
|---|---|---|---|---|---|
| D13 | Moisture diffusion threshold | Transfer 4 units when delta > 16 | Locked Candidate (spec `03` L137) | `03` | Too fast diffusion floods chunk; too slow makes water irrelevant |
| D14 | Scent decay rate | Base decay = 1 per tick; wet extra decay = 2 per tick | Locked Candidate (spec `03` L170–188) | `03` | Wrong rate makes scent permanent or instantly gone |
| D15 | Scent reinforcement amount | +12 per ant movement; +24 on harvest | Locked Candidate (spec `03` L196–205) | `03` | Imbalanced reinforcement makes trails too strong or undetectable |
| D16 | Support reduction from digging | Cardinal neighbors lose 50 support | Locked Candidate (spec `03` L86) | `01`, `03` | Too high = instant collapse everywhere; too low = no collapse risk |
| D17 | Collapse threshold | LooseSoil collapses if support < 100 and open below | Locked Candidate (spec `03` L101) | `03` | Threshold mismatch with initial layout support values |
| D18 | Greedy movement priority | Reduce x first, then y, then try up/right/down/left | Locked Candidate (spec `04` L66–70) | `04` | Determinism depends on exact priority; wrong order = different paths |
| D19 | Residue slowdown mechanism | Even-tick movement only (tick parity) | Locked Candidate (spec `03` L220–221) | `03`, `04` | Odd/even parity must be defined clearly; off-by-one = broken behavior |
| D20 | Water flow priority | down, down-left, down-right, left, right | Locked Candidate (spec `03` L117–122) | `03` | Different priority = different water behavior = different hash |
| D21 | Boundary rows | y=124..128 is stone floor (4 rows) | Locked Candidate (spec `02` L29) | `02` | Mismatch with ASCII map or boundary behavior spec |
| D22 | Fatigue implementation | Allowed but optional; must be explicitly deferred if not built | Deferred | `04` L129–135 | Half-implemented fatigue worse than no fatigue |
| D23 | OBSERVED_THIS_TICK flag | Optional; may be visual-only if excluded from hash | Deferred | `01` L100 | If included in hash, changes determinism; if excluded, may confuse tests |
| D24 | Seed acceptance | Seed accepted and stored but must not alter v0.3.2 layout | Locked Candidate (spec `02` L24) | `02` | If seed accidentally alters layout, determinism breaks |

---

## Decision Status Summary

| Status | Count |
|---|---|
| Needs Chaz Approval | 12 (D1–D12) |
| Locked Candidate (implicit, in spec body) | 12 (D13–D21, D24) |
| Deferred | 2 (D22–D23) |
| Contradictory | 0 (see consistency audit for contradictions) |
| **Total** | **26** |

---

## Notes

- **None of D1–D12 have been approved by Chaz.** The spec repeatedly and correctly states this.
- D13–D21 are numeric/algorithmic decisions embedded in pseudocode. They are treated as locked candidates because the spec provides exact values, but they have not been explicitly surfaced for approval.
- D22 (fatigue) and D23 (OBSERVED_THIS_TICK) are explicitly deferred in the spec and should remain deferred until Chunk 0 substrate is proven.
- v0.3.1 claimed "all critical decisions resolved" in spec_12. v0.3.2 correctly downgraded this to "contract candidates awaiting Chaz approval." This is an important correction.
