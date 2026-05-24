# 12 — Open Decisions for Review

## Status

D1-D12 are resolved as **contract candidates**, but not yet approved for implementation. (v0.3.3 cleanup normalized language from prior D1-D8 references; D9-D12 were already present in the register.)

v0.3.3 improves readiness. Implementation is still not authorized until Chaz explicitly approves. No file should imply "begin implementation," "approved to build," or "build now."

## Approval Checklist

Chaz must explicitly approve or revise each item:

| ID | Decision | Candidate |
|---|---|---|
| D1 | Repo target | Fresh substrate repo |
| D2 | Visualizer | Tauri + HTML Canvas default; minifb/pixels fallback |
| D3 | Chunk size | 128 x 128 |
| D4 | Cell storage | Flat `Vec<Cell>` |
| D5 | Ant group | Single coordinate |
| D6 | Collapse | Deterministic threshold |
| D7 | Perception | Perception Event Ledger only; full Field Notes UI deferred |
| D8 | Tauri | Allowed only for canvas/grid + controls, not card-dashboard gameplay |
| D9 | Receipt naming | `CommandReceipt`, `chunk_deltas`, `dev_event_summary` |
| D10 | Render boundary | `RenderFrame` / `VisibleCell`, default excludes hidden semantic truth |
| D11 | Carcass harvest | One carcass cell equals one harvestable unit in Chunk 0 |
| D12 | Worker loss | Forage entering residue loses `min(3, workers)` once per command |

## Final Build-Readiness Gate

Before coding, confirm:

```text
repo target
visualizer stack
chunk size
cell schema
coordinate ranges
tick order
command list
test list
dependency policy
anti-card rule
RenderFrame truth boundary
CommandReceipt schema
```

## No Implementation Until Approval

No implementation prompt should be produced until the above gate is approved.
