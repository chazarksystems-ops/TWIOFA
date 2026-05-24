# Optimization Pass 0 Design

## Status
- **Normative status:** Design only.
- Not gameplay optimization.
- Not Slice 3.
- Not active chunks.
- Not GPU.
- Not MCTS/MCMC.

## What this document does not authorize
- Does not authorize implementing Optimization Pass 0.
- Does not authorize Slice 3.
- Does not authorize new mechanics.

## Purpose
Build a repeatable headless harness that:

- runs known command scripts,
- records hash sequences,
- checks invariants,
- captures event/delta counters,
- stress-tests local fixtures,
- creates a baseline before collapse/moisture/scent.

## Proposed files
- `chunk0_rust_scaffold/src/bin/chunk0_harness.rs`
- `chunk0_rust_scaffold/src/harness.rs` if needed
- `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md`
- optional output folder: `optimization_runs/`

## CLI commands
- `cargo run --bin chunk0_harness -- run-corpus`
- `cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100`
- `cargo run --bin chunk0_harness -- compare-baseline <a> <b>` if implemented

## Fixed replay corpus
| Script | Purpose | Rough command shape | Expected invariants |
|---|---|---|---|
| RESET_ONLY | Baseline reset hash. | `Reset` | Stable initial hash, 16,384 cells. |
| BASIC_DIG | Verify dig delta. | Reset + one valid dig. | Target becomes Tunnel; support=0. |
| BOUNDARY_DIG_BLOCKED | Verify boundary immutability. | Reset + dig boundary. | Boundary Stone unchanged; blocked receipt. |
| MOVE_CARDINAL | Verify cardinal movement. | Reset + Scout/Step. | No diagonal movement. |
| SOURBACK_ENTRY | Verify transition WorkerLoss. | Start outside residue, enter. | One WorkerLoss. |
| SOURBACK_SLOWDOWN | Verify odd-tick slowdown. | Start on residue, odd tick. | Slowed event, no movement. |
| SOURBACK_REENTRY | Verify exit/re-entry. | Local fixture, leave/re-enter. | One WorkerLoss on re-entry. |
| BLOCKED_MOVEMENT | Verify blocked command. | Unit pocket fixture. | One Blocked event. |
| GREEDY_TIE | Verify x-first tie. | Traversable local fixture. | First move x-reducing. |
| LOW_WORKERS | Verify min worker loss. | workers=2 enter residue. | workers=0, lost=2. |
| LOW_CONFIDENCE | Verify saturating confidence. | confidence=10 enter residue. | confidence=0. |
| RENDERFRAME_BOUNDARY | Verify truth boundary. | Render default frame. | No hidden truth leak. |

## Output fields per script
- `script_name`
- `command_count`
- `ticks_run`
- `hash_sequence`
- `hash_before`
- `hash_after`
- `final_ant_position`
- `final_task`
- `final_workers`
- `final_confidence`
- `event_counts`
- `chunk_delta_count`
- `render_cells_projected`
- `invariants_passed`
- `failure_reason` if any

## Invariants
- `cells.len() == 16384`
- boundary ring remains Stone
- boundary support remains 255 unless spec says otherwise
- Tunnel support remains 0
- no diagonal movement
- Sourback entry is transition-based
- WorkerLoss count obeys once-per-command rule
- workers never underflow
- confidence remains `[0,255]`
- default RenderFrame does not expose hidden truth
- identical script produces identical hash sequence
- CommandScratch/CommandFlags excluded from canonical hash

## Instrumentation counters
- `ticks_run`
- `cells_changed`
- `chunk_deltas`
- `events_emitted`
- `perception_updates`
- `hash_bytes_len` if available
- `render_cells_projected`
- `active_bbox` if available
- `touched_cells` if available

## Active-region observation only
Do not implement active chunks yet. Record:

- changed cells
- touched cells
- bounding box of changed/touched cells
- candidate neighbor halo
- percent of grid touched

## Stress-local-fixtures mode
- deterministic seed
- bounded cases
- small local 5x5 fixtures
- generated fixtures only for movement/residue/boundary/blocking
- no procedural yard
- no gameplay balancing
- no external nondeterministic randomness
- local LCG/xorshift allowed

## Dependency policy
- avoid new dependencies
- no serde unless explicitly justified
- no random crate unless explicitly justified
- plain text/CSV preferred
- JSON only if done manually or with justified dependency

## Acceptance criteria
- `cargo fmt --check` passes
- `cargo check` passes
- `cargo test` passes
- `run-corpus` works
- `stress-local-fixtures` same seed gives same summary
- no rule behavior changes
- no RenderFrame truth leak
- no `source_review/` normative usage

## What this unlocks
Only after this harness exists or is explicitly deferred by Chaz:

- Slice 3 bottom-up collapse can be started.
