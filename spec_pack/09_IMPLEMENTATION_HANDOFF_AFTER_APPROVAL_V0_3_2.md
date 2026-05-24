# 09 — Implementation Handoff After Approval

## Status

This is not an implementation prompt.

Use this file only after Chaz explicitly approves the v0.3.3 (or later) build-readiness gate. v0.3.3 cleanup improves readiness but does not authorize implementation. No file in the workspace implies "start building now."

## Future Implementation Module Sketch

Recommended Rust module layout:

```text
cell.rs           Cell struct, materials, residue, flags
chunk.rs          Vec<Cell>, coordinate helpers, layout initialization
materials.rs      material semantics helpers
sim.rs            tick order and material rules
orders.rs         command enum and preconditions
ant.rs            ant group state and movement/task logic
events.rs         perception events and command receipts
perception.rs     ColonyView / DevTruth projection logic
render_frame.rs   RenderFrame / VisibleCell generation
visualizer/       Tauri canvas or native fallback
tests/            fixtures and C0-TEST implementation
```

## API Contract Sketch

These are conceptual surfaces, not required exact function names:

```text
init_chunk(seed) -> InitReceipt
apply_command(command) -> CommandReceipt
step(ticks) -> CommandReceipt
inspect_cell(x, y, mode) -> CellInspectionData
render_frame(mode, overlay) -> RenderFrame
chunk_hash() -> ChunkHash
dump_events() -> Vec<Event>
reset(seed) -> InitReceipt
```

## Expected Future Files Changed

Allowed source files after approval:

```text
Rust simulation source
visualizer source
unit/integration tests
implementation report
minimal README/run instructions
```

Forbidden during implementation unless explicitly approved:

```text
spec files rewritten as design changes
lore/canon docs
full game docs
unrelated project files
old micro-yard dashboard expansion
```

## Micro-Yard Migration Note

If importing code from the old TWIOFA micro-yard testbench:

Salvage only:

```text
event enum ideas
command receipt idea
Tauri IPC mechanics if useful
append-only event ledger concept
determinism test patterns
```

Discard:

```text
zone-based abstract logic
card dashboard UI
result-card gameplay surface
four-zone menu structure
anything that hides the material chunk behind panels
```

## Required Implementation Report

The coding agent must generate:

```text
CHUNK_0_IMPLEMENTATION_REPORT.md
```

Required report sections:

```text
Files Changed
How to Run
Controls
Validation Commands Run
C0-TEST Results
Determinism Proof / Hashes
Visualizer Check
Anti-Card Compliance Check
Known Deviations
Known Limitations
Next Recommendations
Current git status
```

## Handoff Rule

No implementation is authorized until the user explicitly says this v0.3.2 spec is approved for build.
