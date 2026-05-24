# 12 — Build-Readiness Gate

**v0.3.3 note:** This cleanup pass resolves CA-001, CA-006, CA-007, CA-008, CA-009, CA-011 (see spec_pack/01-03 updates). Implementation is **STILL NOT AUTHORIZED** until every gate item below is checked and explicitly approved by Chaz. v0.3.3 improves readiness; no file authorizes "build now."

Implementation is **NOT AUTHORIZED** until every gate item below is checked and approved by Chaz.

---

## Gate Items

| # | Gate Item | Spec Source | Status |
|---|---|---|---|
| 1 | Repo target approved (D1: fresh substrate repo) | `12` D1 | ⬜ NOT APPROVED |
| 2 | Visualizer default approved (D2: Tauri+Canvas or alternative) | `12` D2 | ⬜ NOT APPROVED |
| 3 | Visualizer fallback approved (D2: minifb or pixels) | `12` D2 | ⬜ NOT APPROVED |
| 4 | Chunk size approved (D3: 128×128) | `12` D3 | ⬜ NOT APPROVED |
| 5 | Cell schema approved (D4: 7 fields, Copy, no heap) | `12` D4 | ⬜ NOT APPROVED |
| 6 | Material enum approved (9 types minimum) | `01` | ⬜ NOT APPROVED |
| 7 | Coordinate ranges approved (half-open, layout bands) | `02` | ⬜ NOT APPROVED |
| 8 | Tick order approved (10 steps, spec `03`) | `03` | ⬜ NOT APPROVED |
| 9 | Command list approved (8 commands, spec `04`) | `04` | ⬜ NOT APPROVED |
| 10 | RenderFrame contract approved (D10: fields, truth boundary) | `06`, `12` D10 | ⬜ NOT APPROVED |
| 11 | WorldTruth / ColonyView / DevTruth boundary approved | `05`, `06` | ⬜ NOT APPROVED |
| 12 | CommandReceipt schema approved (D9: naming convention) | `05`, `12` D9 | ⬜ NOT APPROVED |
| 13 | Carcass harvest model approved (D11: one cell = one unit) | `12` D11 | ⬜ NOT APPROVED |
| 14 | Worker loss rule approved (D12: min(3, workers) once per command) | `12` D12 | ⬜ NOT APPROVED |
| 15 | Test list approved (17 C0-TESTs + 6 fixtures) | `07` | ⬜ NOT APPROVED |
| 16 | Dependency policy approved (allowed/banned list) | `06` L140–165 | ⬜ NOT APPROVED |
| 17 | Anti-card-dashboard UI rule approved | `06` L122–138 | ⬜ NOT APPROVED |
| 18 | Implementation report template approved | `09` L88–111 | ⬜ NOT APPROVED |

---

## Additional Gate Items Identified by Mapping Pass

| # | Gate Item | Source | Status |
|---|---|---|---|
| 19 | Moisture diffusion buffer choice resolved (CA-007) | Consistency Audit | ⬜ NOT RESOLVED |
| 20 | Confidence penalty values specified (CA-008) | Consistency Audit | ⬜ NOT RESOLVED |
| 21 | Scent harvest location specified (CA-009) | Consistency Audit | ⬜ NOT RESOLVED |
| 22 | Collapse + ant group interaction defined (CA-011) | Consistency Audit | ⬜ NOT RESOLVED |
| 23 | Boundary cell material behavior clarified (CA-006) | Consistency Audit | ⬜ NOT RESOLVED |
| 24 | Tunnel support value (0 vs 255) confirmed (CA-001) | Consistency Audit | ⬜ NOT RESOLVED |

---

## Pre-Implementation Verification

Before coding starts, the following must also be true:

- [ ] All 6 major consistency audit items (CA-001, CA-006, CA-007, CA-008, CA-009, CA-011) are resolved
- [ ] No blocker-severity risks remain in the risk register
- [ ] Agent handoff checklist (`13_AGENT_HANDOFF_CHECKLIST.md`) is reviewed
- [ ] Spec patch recommendations (`14_RECOMMENDED_SPEC_PATCHES.md`) are addressed or explicitly deferred

---

## Signature Area

```
Status:        NOT APPROVED
Approved by:   _______________
Date:          _______________
Notes:         _______________
```

---

## Gate Rules

1. **No partial approval.** All 18 original gate items must be approved as a set.
2. **Additional gate items (19–24) may be resolved in a v0.3.3 spec patch** before or concurrent with approval of the main gate.
3. **Approval must be explicit.** "Looks good" is not approval. Chaz must state: "v0.3.2 build-readiness gate is approved."
4. **If any gate item is rejected**, the spec must be revised (v0.3.3) and the gate re-evaluated.
5. **No implementation code may be generated** while this gate status reads "NOT APPROVED."
