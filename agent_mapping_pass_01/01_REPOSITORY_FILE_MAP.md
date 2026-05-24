# 01 — Repository File Map

## Workspace Root

`c:\Users\cheez\Downloads\twiofa_chunk0_spec_workspace_v0_3_2_corrected\`

---

## Root-Level Files

| Path | Purpose | Classification | References/Depends On | Edit Status |
|---|---|---|---|---|
| `README.md` | Workspace overview, read order, main corrections list | Review support | `SPEC_V0_3_2_CORRECTION_REPORT.md`, all spec_pack files | May be updated in v0.3.3 |
| `SOURCE_REVIEW_NOTES.md` | Documents what was corrected from Gemini v0.3.1 DOCX ingest | Review support | v0.3.1 source files, spec_pack v0.3.2 | Historical — do not edit |
| `SPEC_V0_3_2_CORRECTION_REPORT.md` | v0.3.3 cleanup report on v0.3.2 candidate (D1–D12 decisions + 13 ambiguity/audit fixes); build-readiness gate | Source spec | All spec_pack files, `SOURCE_REVIEW_NOTES.md` | Updated in v0.3.3 reconciliation (agent docs); normative spec stable |

---

## `spec_pack/` — Authoritative v0.3.2 Spec Files

| Path | Purpose | Classification | References/Depends On | Edit Status |
|---|---|---|---|---|
| `spec_pack/00_CHUNK_0_PURPOSE_AND_BOUNDARIES_V0_3_2.md` | Purpose, core direction, prototype claims C0-P1 to C0-P9, Definition of Done/Not Done, hard non-goals | Source spec | Standalone (foundational) | Editable in v0.3.3 |
| `spec_pack/01_MATERIAL_CELL_MODEL_V0_3_2.md` | Cell schema (7 fields), material enum (9 types), residue enum, flags, material semantics table, material interaction table, budget constraints | Source spec | `00` for scope; `03` for rule details | Editable in v0.3.3 |
| `spec_pack/02_CHUNK_LAYOUT_AND_SCENE_V0_3_2.md` | Coordinate convention, seeded layout contract, boundary behavior, exact coordinate bands table, ASCII mini-map, scene question | Source spec | `01` for material types | Editable in v0.3.3 |
| `spec_pack/03_SIMULATION_RULES_V0_3_2.md` | Normative tick order (10 steps), determinism policy, canonical hash rule, pseudocode for dig/collapse/water/moisture/scent/residue/harvest/receipt | Source spec | `01` for cell fields; `02` for layout; `04` for ant group | Editable in v0.3.3 |
| `spec_pack/04_ANT_GROUP_AND_ORDERS_V0_3_2.md` | Ant group state (fields, task enum), route model, movement rule, command payloads (8 commands), preconditions/failures, Avoid rule, worker loss, fatigue, confidence | Source spec | `01` for material traversability; `03` for rules | Editable in v0.3.3 |
| `spec_pack/05_PERCEPTION_EVENTS_AND_RECEIPTS_V0_3_2.md` | WorldTruth vs ColonyPerception split, language guardrail, event schema (10 types), event-to-perception mapping, CommandReceipt contract, example receipt JSON | Source spec | `01` for residue; `03` for rules; `04` for commands | Editable in v0.3.3 |
| `spec_pack/06_RENDERFRAME_VISUALIZER_AND_INSPECTION_V0_3_2.md` | Main surface rule, default/fallback visualizer, RenderFrame contract, VisibleCell contract, inspection modes (ColonyView/DevTruth), overlay modes (6), UI anti-card compliance test, dependency policy | Source spec | `05` for perception; `00` for boundaries | Editable in v0.3.3 |
| `spec_pack/07_VALIDATION_AND_QA_V0_3_2.md` | Fixture list (6 fixtures), validation test matrix (17 tests), manual visual smoke path, automated smoke path | Source spec | All preceding spec files | Editable in v0.3.3 |
| `spec_pack/08_NON_GOALS_AND_SCOPE_GATES_V0_3_2.md` | Hard non-goals, scope gate, UI gate, backend gate, agent stop triggers, human approval list | Source spec | `00` for non-goals; `06` for dependency policy | Editable in v0.3.3 |
| `spec_pack/09_IMPLEMENTATION_HANDOFF_AFTER_APPROVAL_V0_3_2.md` | Module sketch (11 modules), API contract sketch, allowed/forbidden files, micro-yard migration note, implementation report template | Source spec | All preceding spec files | Editable only after Chaz approval |
| `spec_pack/10_TRACEABILITY_MATRIX_V0_3_2.md` | Requirement-to-test trace table, decision-to-file trace table, open-to-implementation trace | Source spec | `07` for tests; `12` for decisions | Editable in v0.3.3 |
| `spec_pack/11_RISK_REGISTER_V0_3_2.md` | 15 risks (R1–R15) with severity, likelihood, detection, mitigation, owner; highest priority risks list | Source spec | All spec files | Editable in v0.3.3 |
| `spec_pack/12_OPEN_DECISIONS_FOR_REVIEW_V0_3_2.md` | D1–D12 decisions with candidates, approval checklist, build-readiness gate, implementation prohibition | Source spec | All spec files | Editable in v0.3.3 |
| `spec_pack/13_ARCHITECTURE_AND_INFLUENCE_V0_3_2.md` | Player/developer freedom goals, Noita/Diablo/Hades/BG3 influence tables, TWIOFA's unique question, future expansion ladder (Chunk 1–5) | Source spec | `00` for direction | Editable in v0.3.3 |

---

## `source_review/` — Historical v0.3.1 Source Material

| Path | Purpose | Classification | Edit Status |
|---|---|---|---|
| `source_review/SPEC_V0_3_1_HARDENING_REPORT.md.txt` | Original v0.3.1 hardening report from Gemini | Historical source | Do not edit |
| `source_review/spec_pack_00_CHUNK_0_PURPOSE_AND_BOUNDARIES_V0_3_1.md.txt` | v0.3.1 purpose and boundaries | Historical source | Do not edit |
| `source_review/spec_pack_01_MATERIAL_CELL_MODEL_V0_3_1.md.txt` | v0.3.1 cell model | Historical source | Do not edit |
| `source_review/spec_pack_02_CHUNK_LAYOUT_AND_SCENE_V0_3_1.md.txt` | v0.3.1 layout and scene | Historical source | Do not edit |
| `source_review/spec_pack_03_SIMULATION_RULES_V0_3_1.md.txt` | v0.3.1 simulation rules | Historical source | Do not edit |
| `source_review/spec_pack_04_ANT_GROUP_AND_ORDERS_V0_3_1.md.txt` | v0.3.1 ant group and orders | Historical source | Do not edit |
| `source_review/spec_pack_05_PERCEPTION_EVENTS_AND_RECEIPTS_V0_3_1.md.txt` | v0.3.1 perception and receipts | Historical source | Do not edit |
| `source_review/spec_pack_06_RENDERFRAME_VISUALIZER_AND_INSPECTION_V0_3_1.md.txt` | v0.3.1 visualizer and RenderFrame | Historical source | Do not edit |
| `source_review/spec_pack_07_VALIDATION_AND_QA_V0_3_1.md.txt` | v0.3.1 validation (partial, had missing test matrix) | Historical source | Do not edit |
| `source_review/spec_pack_08_NON_GOALS_AND_SCOPE_GATES_V0_3_1.md.txt` | v0.3.1 non-goals | Historical source | Do not edit |
| `source_review/spec_pack_09_IMPLEMENTATION_HANDOFF_AFTER_APPROVAL_V0_3_1.md.txt` | v0.3.1 handoff | Historical source | Do not edit |
| `source_review/spec_pack_10_TRACEABILITY_MATRIX_V0_3_1.md.txt` | v0.3.1 traceability (effectively empty — 2 lines) | Historical source | Do not edit |
| `source_review/spec_pack_11_RISK_REGISTER_V0_3_1.md.txt` | v0.3.1 risk register (effectively empty — 1 line) | Historical source | Do not edit |
| `source_review/spec_pack_12_OPEN_DECISIONS_FOR_REVIEW_V0_3_1.md.txt` | v0.3.1 open decisions (claimed all resolved) | Historical source | Do not edit |
| `source_review/spec_pack_13_ARCHITECTURE_AND_INFLUENCE_V0_3_1.md.txt` | v0.3.1 architecture and influence | Historical source | Do not edit |

---

## `skills/` — Agent Skill Definitions

| Path | Purpose | Classification | Edit Status |
|---|---|---|---|
| `skills/twiofa_chunk0_spec_hardener/SKILL.md` | Agent skill definition for spec hardening passes: gates, required concepts, preferred/forbidden language, review output format | Generated/auxiliary | May be updated for future hardening passes |

---

## `agent_mapping_pass_01/` — This Mapping Pass

| Path | Purpose | Classification | Edit Status |
|---|---|---|---|
| `agent_mapping_pass_01/*.md` | Spec intelligence map (this pass) | Generated/auxiliary (analysis) | May be updated during this pass only |

---

## File Count Summary

| Category | Count |
|---|---|
| Source spec (v0.3.2) | 14 files + 1 correction report |
| Review support | 2 files (README, SOURCE_REVIEW_NOTES) |
| Historical source (v0.3.1) | 15 files |
| Agent skill | 1 file |
| **Total workspace files** | **33 files** |

---

## Key Cross-Reference Observations

1. **v0.3.1 → v0.3.2 delta**: The v0.3.1 traceability matrix (spec_10) and risk register (spec_11) were effectively empty. v0.3.2 filled them. The v0.3.1 open decisions file (spec_12) claimed "all critical decisions resolved" even though they weren't Chaz-approved; v0.3.2 correctly marks them as candidates.
2. **NestWall traversability**: v0.3.1 spec_01 listed NestWall as traversable. v0.3.2 corrected this — NestWall is not traversable; the nest chamber is represented by Tunnel cells.
3. **Carcass harvest model**: v0.3.1 spec_03 said "after N harvest actions," implying a hidden mass field. v0.3.2 simplified to one cell = one harvestable unit.
4. **Tunnel support in layout**: v0.3.1 spec_02 set tunnel support to 255. v0.3.2 spec_02 set it to 0. This is a significant change (see consistency audit).
5. **Decision count**: v0.3.1 had D1–D8. v0.3.2 added D9–D12 (receipt naming, render boundary, carcass harvest, worker loss). v0.3.3 reconciliation pass normalized remaining D1–D8 references in agent_mapping docs to D1–D12 / v0.3.3 cleanup state. All agent docs now align with post-v0.3.3 spec_pack (still awaiting Chaz approval of D1–D12 gate).
