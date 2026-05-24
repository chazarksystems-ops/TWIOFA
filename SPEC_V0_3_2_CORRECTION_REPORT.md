# TWIOFA Chunk 0 Spec Pack — v0.3.3 Cleanup Report (on v0.3.2 Candidate)

## Status

**v0.3.3 cleanup pass complete on v0.3.2 locked-contract candidate.**

v0.3.3 improves spec readiness (resolves 13 listed ambiguities and drift items). Implementation is STILL NOT AUTHORIZED until Chaz explicitly approves the full build-readiness gate. No file in this workspace implies "start building now," "approved to build," or "begin implementation."

## Purpose of This Pass

This v0.3.3 pass performs a narrow textual cleanup on the v0.3.2 spec workspace. It resolves the specific inconsistencies listed in the mandatory corrections (moisture diffusion method, worker-loss confidence penalty, harvest scent target, collapse+AntGroup interaction, boundary ring behavior, Tunnel support note, fixture definitions, D1-D12 normalization, CommandReceipt naming clarification, board_deltas/result_card grep claim, source_review historical warning, and build-gate language). It does not add mechanics, authorize code, or alter file organization. Historical v0.3.2 references are preserved where accurate. Grep confirmed: no normative uses in v0.3.2/v0.3.3 spec pack; board_deltas and result_card terms appear only in the banned/avoid terminology list.

This pass builds on the v0.3.2 consolidation of Gemini's v0.3.1 output. The v0.3.2 workspace corrected terminology drift, omissions, and contradictions without authorizing code.

## Resolved Decisions D1-D12

| ID | Topic | Locked Contract Candidate | Approval Status |
|---|---|---|---|
| D1 | Repo target | Fresh substrate repo, not the old card-dashboard micro-yard repo | Requires Chaz approval |
| D2 | Visualizer | Tauri + HTML Canvas as default, if the canvas is the central grid; fallback: minifb or pixels | Requires Chaz approval |
| D3 | Chunk size | 128 x 128 cells | Requires Chaz approval |
| D4 | Cell storage | Flat `Vec<Cell>` first pass; small copyable cell; no heap-owned fields | Requires Chaz approval |
| D5 | Ant group | Single coordinate ant group representing the colony effort front | Requires Chaz approval |
| D6 | Collapse | Deterministic threshold, no random collapse roll | Requires Chaz approval |
| D7 | Field Notes scope | Perception Event Ledger only; full Field Notes UI deferred | Requires Chaz approval |
| D8 | Tauri policy | Tauri allowed only as canvas/grid visualizer + controls; no card-dashboard gameplay | Requires Chaz approval |
| D9 | Receipt naming | `CommandReceipt`, `chunk_deltas`, `dev_event_summary` (not `board_deltas`, `result_card`) | Requires Chaz approval |
| D10 | Render boundary | `RenderFrame` / `VisibleCell`; default excludes hidden semantic truth | Requires Chaz approval |
| D11 | Carcass harvest | One carcass cell equals one harvestable unit in Chunk 0 | Requires Chaz approval |
| D12 | Worker loss | Forage entering residue loses `min(3, workers)` once per command | Requires Chaz approval |

## Major Corrections Applied (v0.3.2 + v0.3.3)

1. Standardized terms around `CommandReceipt`, `chunk_deltas`, `RenderFrame`, `VisibleCell`, and `dev_event_summary`.
2. Grep confirmed: no normative uses in v0.3.2/v0.3.3 spec pack; board_deltas and result_card terms appear only in the banned/avoid terminology list. (Kept banned blocks intact.)
3. Clarified that debug panels are support tools, not the game surface.
4. Clarified DevTruth vs ColonyView and ensured hidden semantic truth is not exposed by default.
5. Replaced unsafe raw-memory hashing with canonical simulation serialization.
6. Corrected coordinate ranges to half-open notation.
7. Filled missing traceability and risk files.
8. Added precise enough rules to prevent agents from inventing architecture.
9. (v0.3.3) Locked moisture diffusion to double-buffer (order-independent source/dest).
10. (v0.3.3) Defined fixed worker-loss confidence penalty (16, clamped [0,255]).
11. (v0.3.3) Locked deterministic harvest scent target rule and neighbor order.
12. (v0.3.3) Defined collapse+AntGroup interaction via existing worker-loss consequence + receipt.
13. (v0.3.3) Made boundary ring explicit immutable Stone; dig blocked+receipted.
14. (v0.3.3) Added Tunnel support=0 intentional note (open space).
15. (v0.3.3) Expanded fixture definitions (FIXTURE_* ) in validation docs.
16. (v0.3.3) Normalized D1-D12 language; added CommandReceipt vs command_receipt naming note.
17. (v0.3.3) Added source_review/ historical-only warnings.
18. (v0.3.3) Hardened build-readiness language against premature implementation implication.

## Build-Readiness Gate (Updated for v0.3.3)

v0.3.3 cleanup improves readiness by closing the 13 listed ambiguities. However, implementation is STILL NOT AUTHORIZED. Chaz must explicitly approve before any code is written. No file may imply "begin implementation," "approved to build," or "build now."

Before implementation, Chaz must approve (D1-D12 plus supporting items):

1. repo target (D1);
2. visualizer default and fallback (D2);
3. exact chunk size (D3);
4. cell schema (D4);
5. coordinate ranges;
6. tick order (incl. double-buffer moisture);
7. command list;
8. test list + fixture definitions;
9. dependency policy;
10. no-card-dashboard UI rule (D8);
11. `RenderFrame` / `VisibleCell` truth boundary (D10);
12. CommandReceipt schema + naming (D9);
13. Carcass harvest model (D11);
14. Worker loss rule + confidence penalty (D12);
15. Boundary/collapse/ant interaction rules;
16. All CA items from consistency audit resolved in spec.

Until this approval happens, no implementation code should be generated. v0.3.3 does not change this gate.
