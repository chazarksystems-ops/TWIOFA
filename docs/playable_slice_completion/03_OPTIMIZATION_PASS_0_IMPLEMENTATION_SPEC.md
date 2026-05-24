# Optimization Pass 0 — Implementation Spec

## Status
- **Normative status:** implementation spec for the harness pass.
- **Kind:** proof/instrumentation infrastructure, not gameplay optimization.

## What this document does not authorize
- No active chunks.
- No sleep lists.
- No GPU/Margolus/block CA.
- No MCTS/MCMC.
- No gameplay balancing.

## Files to add or modify
| File | Action |
|---|---|
| `chunk0_rust_scaffold/src/bin/chunk0_harness.rs` | Add CLI binary. |
| `chunk0_rust_scaffold/src/harness.rs` | Add reusable harness module if useful. |
| `chunk0_rust_scaffold/src/lib.rs` | Export `harness` only if module added. |
| `chunk0_rust_scaffold/tests/` | Add harness/unit tests if needed. |
| `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | Create report. |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Append pass summary. |

## CLI commands
Implement these commands exactly:

```text
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_harness -- compare-baseline <a> <b>
```

`compare-baseline` may be implemented as a simple text/CSV comparison. If too large, include a stub that prints a guarded “not implemented yet” message and does not affect simulation.

## Corpus scripts
Each script must run through public `Simulation::execute_command` calls, not private mutation, except where a fixture explicitly needs setup.

| Script | Commands / setup | Required invariant |
|---|---|---|
| `RESET_ONLY` | `Reset` | stable initial hash, 16,384 cells. |
| `BASIC_DIG` | reset; set ant adjacent if needed; `DigTunnel` valid Soil/LooseSoil | one material delta to Tunnel; Tunnel support=0. |
| `BOUNDARY_DIG_BLOCKED` | reset; ant adjacent to boundary if needed; dig boundary Stone | boundary unchanged; CommandFailed. |
| `MOVE_CARDINAL` | reset; `ScoutResidue` or local traversable fixture; step | no diagonal movement. |
| `SOURBACK_ENTRY` | start outside residue; `SendForagers`; enter residue | one WorkerLoss. |
| `SOURBACK_SLOWDOWN` | start on residue; odd tick | slowed event, no movement. |
| `SOURBACK_REENTRY` | local fixture; leave residue then re-enter in one command | one WorkerLoss on re-entry. |
| `BLOCKED_MOVEMENT` | local 5x5 blocked pocket | one Blocked event. |
| `GREEDY_TIE` | traversable local fixture | x-reducing first step. |
| `LOW_WORKERS` | workers=2; enter residue | workers=0; lost=2. |
| `LOW_CONFIDENCE` | confidence=10; enter residue | confidence=0. |
| `RENDERFRAME_BOUNDARY` | create default RenderFrame | no hidden truth in default view. |

## Output format
Prefer plain text plus CSV. Avoid serde unless explicitly justified.

Recommended files:

```text
optimization_runs/latest/run_corpus_summary.txt
optimization_runs/latest/run_corpus_hashes.csv
optimization_runs/latest/stress_local_fixtures_summary.txt
```

CSV columns for corpus:

```text
script_name,command_count,ticks_run,hash_before,hash_after,hash_sequence,event_counts,chunk_delta_count,final_x,final_y,final_workers,final_confidence,invariants_passed,failure_reason
```

## Invariants to check
- `cells.len() == 16384`.
- boundary ring remains Stone.
- boundary support remains 255 unless a future spec says otherwise.
- Tunnel support remains 0.
- no diagonal movement.
- Sourback entry is transition-based.
- WorkerLoss obeys once-per-command rule.
- workers never underflow.
- confidence remains within `[0,255]`.
- default RenderFrame does not expose hidden truth.
- identical script produces identical hash sequence.
- CommandFlags excluded from canonical hash.

## Stress-local-fixtures mode
- Deterministic local LCG or xorshift only; no `rand` crate unless explicitly justified.
- Bounded cases.
- Local 5x5 fixtures only.
- Generate fixtures only for movement/residue/boundary/blocking.
- No procedural yard.
- Same seed + cases must produce identical summary.

## Active-region observation only
Record these counters if practical, but do not optimize from them yet:

```text
touched_cells
changed_cells
active_bbox_min_x, active_bbox_min_y, active_bbox_max_x, active_bbox_max_y
percent_grid_touched
```

## Acceptance criteria
- `cargo fmt --check` passes.
- `cargo check` passes.
- `cargo test` passes.
- `run-corpus` completes with all invariants passing.
- `stress-local-fixtures --seed 42 --cases 100` is deterministic across two runs.
- No rule behavior changes.
- No RenderFrame truth leak.
- No `source_review/` normative usage.
