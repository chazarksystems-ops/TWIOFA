# 10 — Traceability Matrix

## Requirement-to-Test Trace

| Requirement | Spec file | Test(s) | Future surface |
|---|---|---|---|
| Physical chunk grid is central | 00, 06 | C0-TEST-UI-1 | visualizer / RenderFrame |
| Soil can become tunnel | 01, 03 | C0-TEST-MAT-1 | cell, sim, orders |
| Collapse can occur | 01, 03 | C0-TEST-MAT-2 | sim collapse loop |
| Water/moisture changes state | 01, 03 | C0-TEST-MAT-3, MAT-4 | sim water/moisture |
| Scent decays/reinforces | 01, 03 | C0-TEST-MAT-5, MAT-6 | sim scent |
| Carcass physically depletes | 01, 03, 04 | C0-TEST-MAT-7 | ant/forage/material delta |
| Sourback residue affects behavior | 01, 03, 04 | C0-TEST-ANT-1, ANT-2 | residue/ant/perception |
| ColonyView hides hidden truth | 05, 06 | C0-TEST-PERC-1, UI-3 | perception/render_frame |
| Bitter residue observed before Sourback label | 05 | C0-TEST-PERC-2 | perception events |
| Later evidence can reframe | 05 | C0-TEST-PERC-3 | event ledger |
| Same commands produce same hash | 03, 07 | C0-TEST-DET-1 | sim/hash/tests |
| UI state excluded from hash | 03, 06, 07 | C0-TEST-DET-2 | hash/render separation |
| Invalid commands fail safely | 04, 07 | C0-TEST-CMD-1 | orders/events |
| No card-dashboard regression | 06, 08 | C0-TEST-UI-1, UI-2 | visualizer review |

## Decision-to-File Trace

| Decision | Files affected |
|---|---|
| D1 Fresh substrate repo | 00, 08, 09, 12 |
| D2 Tauri canvas default / fallback | 06, 08, 09, 12 |
| D3 128x128 chunk | 00, 01, 02, 06, 07 |
| D4 Vec<Cell> | 01, 03, 09 |
| D5 Single-coordinate ant group | 04, 07 |
| D6 Deterministic collapse | 03, 07 |
| D7 Perception Event Ledger | 05, 07, 09 |
| D8 Conditional Tauri policy | 06, 08, 09 |

## Open-to-Implementation Trace

Implementation prompt may be written only after:

```text
D1-D8 confirmed by Chaz
Build-readiness gate approved
C0-TEST matrix accepted
Dependency policy accepted
```
