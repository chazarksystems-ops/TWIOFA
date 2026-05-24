# Minimal Play Surface — CLI/REPL/Text Visualizer Spec

## Status
- **Normative status:** implementation spec for the first playable surface.
- **Preferred order:** CLI/REPL first, then optional Tauri later.

## What this document does not authorize
- Tauri polish.
- Card-dashboard gameplay.
- Full UI architecture.
- Hidden truth leaks in default view.

## Goal
A user should be able to run a Chunk 0 session without writing Rust tests.

## Files to add or modify
| File | Action |
|---|---|
| `chunk0_rust_scaffold/src/bin/chunk0_cli.rs` | Add command/REPL binary. |
| `chunk0_rust_scaffold/src/play.rs` | Optional reusable play/session helpers. |
| `chunk0_rust_scaffold/src/lib.rs` | Export play module if added. |
| `CHUNK_0_PLAYABLE_SLICE_REPORT.md` | Final report. |
| Tests | Add smoke tests for parser/session if practical. |

## CLI modes
Support both one-shot commands and REPL.

### One-shot examples
```text
cargo run --bin chunk0_cli -- reset
cargo run --bin chunk0_cli -- run-script smoke
cargo run --bin chunk0_cli -- render-text colony
cargo run --bin chunk0_cli -- render-text devtruth
```

### REPL commands
```text
help
reset
step <ticks>
dig <x> <y>
scout <x> <y>
forage <x> <y>
return-home
inspect <x> <y> colony
inspect <x> <y> devtruth
render colony
render devtruth
receipt
hash
quit
```

## Text render requirements
Default `render colony`:
- 128x128 may be shown as a cropped viewport if full grid is too wide.
- Must include AntGroup marker.
- Must show material categories.
- Must not expose raw SourbackBitter name.
- Must not show raw support/moisture/scent numbers unless in devtruth mode.

Suggested symbols:
| Symbol | Meaning |
|---|---|
| `#` | Stone / boundary |
| `.` | Air |
| `s` | Soil |
| `l` | LooseSoil |
| `t` | Tunnel |
| `w` | Water |
| `c` | Carcass |
| `r` | Root |
| `n` | NestWall |
| `A` | AntGroup |
| `?` | perceived bitter/yellow sign, if visible/known |

DevTruth render may show residue/debug overlays, but must be explicitly requested.

## Session state
- CLI may keep state only in memory for this pass.
- Save/load is deferred unless trivial and deterministic.
- Every command should print latest receipt summary and hash.

## Smoke script
Implement `run-script smoke` as:
1. reset
2. render colony or inspect home
3. dig one valid adjacent soil/loose soil cell using a controlled fixture if needed
4. scout toward residue
5. forage toward carcass/residue edge
6. step enough ticks for movement effect
7. if Slice 5 exists, harvest one carcass and return home
8. print final hash and event summary

If full harvest loop is not yet reachable, script must say `PARTIAL_SMOKE` and list the missing system.

## Tests
- parser accepts known commands.
- parser rejects malformed commands without panic.
- `run-script smoke` completes.
- `render colony` omits hidden truth strings.
- `render devtruth` is opt-in.
- repeated smoke script gives identical final hash.

## Acceptance criteria
- User can run the smoke script from terminal.
- User can issue at least reset/step/dig/scout/forage/return/render/hash.
- Receipts are visible after commands.
- Default view does not leak hidden truth.
- No card dashboard.
