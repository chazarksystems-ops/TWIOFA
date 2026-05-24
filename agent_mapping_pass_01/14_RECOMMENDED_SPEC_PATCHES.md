# 14 — Recommended Spec Patches

Patch recommendations for the v0.3.2 spec. No original files are modified by this document.

---

## Applied v0.3.3 Corrections (historical recommendations vs actual resolutions)

**Note (v0.3.3 reconciliation):** The patches below were addressed during the v0.3.3 cleanup pass. Actual resolutions in spec_pack/ may differ in detail from the original "Recommended" suggestions here (this document is historical analysis). All listed items are now reflected in the normative spec_pack files. See individual v0.3.3 notes below.

| # | Target File | Current Issue (pre-v0.3.3) | Original Recommended / Fix | Priority | CA Reference | v0.3.3 Status |
|---|---|---|---|---|---|---|
| P-01 | `spec_pack/03` L131 | Moisture diffusion says "stable pass or double-buffer" — unresolved OR | Lock to one approach. Recommended: "in-place single-buffer pass..." | **High** | CA-007 | **APPLIED** (double-buffer locked; source/dest buffers, order-independent; see spec_pack/03 L136-151 — variance from single-buffer rec here) |
| P-02 | `spec_pack/03` L229 | "confidence -= fixed penalty" — no numeric value | Replace with exact values. Recommended: "confidence = ...saturating_sub(20)..." | **High** | CA-008 | **APPLIED** (fixed 16, clamped [0,255]; see spec_pack/03 L241 + 04 L127 — variance from suggested values) |
| P-03 | `spec_pack/03` L203–205 | "current cell or adjacent traversable cell gains scent_food +24" — ambiguous cell selection | Replace with: "The ant group's current cell gains..." | **High** | CA-009 | **APPLIED** (current if traversable, else first in up/right/down/left; clamp 255; see spec_pack/03 L210-216) |
| P-04 | `spec_pack/03` L94–108 | Collapse pseudocode doesn't address ant group on destination cell | Add rule after L106: "...the collapse swap is blocked..." | **High** | CA-011 | **APPLIED** (swap occurs; AntGroup gets existing worker-loss + event + receipt; no new systems; see spec_pack/03 L110-116 — variance from "blocked" rec) |
| P-05 | `spec_pack/02` L28–29 | "Solid boundary behavior" is ambiguous — could mean boundary cells are Stone or just that flow/movement stops at edges | Replace with: "Cells at x=0... use their layout-defined materials but act as impassable..." | **High** | CA-006 | **APPLIED** (explicit immutable Stone ring in WorldTruth; dig blocked+receipted; see spec_pack/02 L26-34 — variance from "layout materials" rec) |
| P-06 | `spec_pack/02` L46 | Tunnel support = 0 changed from v0.3.1's 255 without explicit callout | Add note: "Tunnel support is 0 because tunnels are open space..." | **Medium** | CA-001 | **APPLIED** (intentional note added; see spec_pack/01 after material table) |

---

## Optional Clarifications (improve spec quality but not blocking)

| # | Target File | Current Issue | Suggested Fix | Priority | CA Reference |
|---|---|---|---|---|---|
| C-01 | `spec_pack/03` L114 | Water cell iteration order within a row is not specified | Add: "Within each row, water cells are processed left-to-right (x=0 to x=127)." | Medium | CA-012 |
| C-02 | `spec_pack/03` L221 | AntGroupSlowed event emission frequency unclear | Replace with: "AntGroupSlowed is emitted once per command, on the first odd tick where movement is skipped due to SourbackBitter residue." | Medium | CA-010 |
| C-03 | `spec_pack/01` L77 | Inline "Correction:" note about NestWall traversability is unusual | Remove the inline note. The material semantics table (L75) already shows NestWall as non-traversable. The correction note is redundant. | Low | CA-002 |
| C-04 | `spec_pack/06` L153–165 | wgpu/pixels clarification is far from the banned list | Move L165 clarification to immediately after L163 (end of banned list), or add a footnote marker on L155–156 pointing to L165. | Low | CA-004 |
| C-05 | `spec_pack/09` L1–7 | Implementation handoff file exists in workspace, may be misread as authorization | Add a prominent warning banner at top: `> ⚠️ WARNING: IMPLEMENTATION NOT AUTHORIZED. This file is a post-approval reference only. Do not use until Chaz explicitly approves the build-readiness gate.` | Medium | CA-005 |
| C-06 | `spec_pack/05` L136 | `dev_event_summary` example reads like narrative prose | Add note: "`dev_event_summary` is a terse debug string, not a narrative. Example shown is illustrative; actual format should be compact." | Low | CA-015 |
| C-07 | Multiple files | `ColonyPerception` (data) vs `ColonyView` (rendering mode) used interchangeably | Add glossary entry: "ColonyPerception = epistemic data model. ColonyView = the rendering mode that presents ColonyPerception to the player. They are related but distinct: one is data, the other is presentation." | Medium | CA-018 |
| C-08 | `spec_pack/07` L12 | FIXTURE_BLOCKED_DIG has no layout definition | Add brief fixture layout: "FIXTURE_BLOCKED_DIG: Ant group at (55, 95). Stone wall at (56, 95). Dig target (56, 95) is Stone (not diggable). Expected: CommandFailed(NotDiggable)." Similar for other non-INITIAL fixtures. | Medium | CA-013 |

---

## Future Enhancements (for v0.4.0+ or later chunks)

| # | Topic | Description | Priority | Notes |
|---|---|---|---|---|
| E-01 | Scent diffusion | Add lateral scent diffusion rule (similar to moisture diffusion) so trails are wider than 1 cell | Medium | Research supports this; deferred for Chunk 0 scope |
| E-02 | Exploration probability | Add stochastic exploration when no scent gradient detected | Low | Not needed for Chunk 0 (commands specify targets) |
| E-03 | Double-processing prevention | Add explicit strategy for preventing cells from being processed twice per tick (processed flag or frame counter) | Medium | Research identifies this as common pitfall |
| E-04 | Diagonal bias documentation | Document that water flow has a leftward bias due to fixed priority (down-left before down-right) | Low | Acceptable tradeoff for determinism |
| E-05 | Fixture layout specs | Full coordinate-band definitions for all 6 fixtures (not just FIXTURE_INITIAL) | Medium | Currently only INITIAL has a full layout |
| E-06 | Performance benchmarks | Add expected performance targets for tick and render times | Low | 128×128 is trivially fast; benchmarks can wait |
| E-07 | Confidence increase values | Specify exact confidence increase amounts for successful scout, harvest, and route maintenance | Medium | Currently only decreases are discussed (incompletely) |

---

## No-Action Items (reviewed and confirmed correct)

| # | Item | Reason No Action Needed |
|---|---|---|
| N-01 | `board_deltas` / `result_card` terminology | Grep confirmed: no normative uses in v0.3.2/v0.3.3 spec pack; terms appear only in the banned/avoid terminology list. |
| N-02 | v0.3.1 "all decisions resolved" claim | Already corrected in v0.3.2 spec_12 |
| N-03 | Cell byte-size claims | v0.3.2 correctly says "do not claim exact byte size unless verified by test" |
| N-04 | Half-open coordinate ranges | v0.3.2 correctly uses half-open notation throughout |
| N-05 | Carcass mass field | v0.3.2 correctly removed per-cell mass (one cell = one unit) |

---

## Suggested v0.3.3 Patch Plan (Historical — executed during v0.3.3 cleanup)

**Status (v0.3.3 reconciliation):** The patches and clarifications below were addressed (with some variance from original recommendations) as part of the v0.3.3 narrow cleanup pass on spec_pack/. See the "Applied v0.3.3 Corrections" table above for details. This section preserved as historical record.

### Phase 1: Required Corrections (P-01 through P-06) — APPLIED
- **Scope**: 6 corrections across 2 spec files (addressed in v0.3.3)
- **Files changed**: `spec_pack/02`, `spec_pack/03` (plus fixture expansions in 07)
- **Risk**: Low — these are precision fixes, not design changes
- **Status**: Completed during v0.3.3 (actual resolutions documented in Applied table; some variance from 2016 recommendations here)

### Phase 2: Optional Clarifications (C-01 through C-08) — PARTIALLY APPLIED
- **Scope**: 8 clarifications across 5 spec files
- **Files changed**: `spec_pack/01`, `spec_pack/03`, `spec_pack/05`, `spec_pack/06`, `spec_pack/07`, `spec_pack/09`
- **Risk**: Very low — editorial improvements
- **Status**: Several addressed (e.g. fixture definitions expanded in 07; others remain optional or deferred)

### Phase 3: Test Gap Closure — PARTIALLY ADDRESSED
- **Scope**: Add 28 missing test definitions to spec `07`
- **Files changed**: `spec_pack/07` (expanded test matrix + fixture definitions)
- **Risk**: Low — additional tests don't change design
- **Status**: Fixture list significantly expanded in v0.3.3 (see spec_pack/07); full 28-test matrix remains future work per scope.

### Patch Sequence (Historical)

1. Apply P-01 through P-06 (required corrections) — **DONE in v0.3.3**
2. Apply C-01 through C-08 (optional clarifications) — partially done
3. Expand test matrix in spec `07` for 28 missing tests — partially done (fixtures)
4. Update spec `10` (traceability matrix) to reflect new tests
5. Update spec `12` (open decisions) with any new decisions
6. Re-run consistency audit to verify no new issues — **this reconciliation pass**
7. Present v0.3.3 to Chaz for review — pending Chaz approval of D1-D12 gate

### Version Bump Rule

```
v0.3.3 = corrections and clarifications only
v0.4.0 = design changes (new rules, new materials, etc.)
```

---

## Summary

| Category | Count |
|---|---|
| Required corrections | 6 |
| Optional clarifications | 8 |
| Future enhancements | 7 |
| No-action items | 5 |
| **Total items reviewed** | **26** |
