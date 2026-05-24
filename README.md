# The World Is on FIRE Ants / TWIOFA

## Current Status

- **Chunk 0 playable/testable deterministic slice** — verified locally by Chaz
- Not full-game canon
- Not final polished UI
- Not procedural full yard

## What Is Chunk 0?

Chunk 0 is the first playable deterministic 128×128 yard-substrate slice. It proves movement, digging, Sourback residue risk, collapse, water/moisture, scent, harvest, return-home loop, deterministic receipts, harness validation, and CLI smoke play.

This slice does not lock full-game canon and does not authorize expansion beyond Chunk 0 without Chaz approval.

## Validation

```sh
cd chunk0_rust_scaffold
cargo fmt --check
cargo check
cargo test
cargo run --bin chunk0_harness -- run-corpus
cargo run --bin chunk0_harness -- stress-local-fixtures --seed 42 --cases 100
cargo run --bin chunk0_cli -- run-script smoke
```

**Expected results:**

| Command | Expected |
|---|---|
| `cargo test` | 57 passed / 0 failed |
| `run-corpus` | 12/12 passed |
| `stress-local-fixtures` | 100/100 passed |
| `run-script smoke` | `SMOKE_DETERMINISM: PASS` |

## Interactive CLI / REPL

```sh
cd chunk0_rust_scaffold
cargo run --bin chunk0_cli -- repl
```

**REPL commands:**

```
> reset                    # reset to initial state
> render colony            # 48x24 viewport around ant
> render devtruth          # same viewport with raw debug info
> step 5                   # advance 5 ticks
> scout 84 28              # send scouts toward residue band
> forage 95 22             # send foragers toward carcass region
> inspect 85 30 colony     # inspect cell (colony-safe view)
> inspect 85 30 devtruth   # inspect cell (full debug view)
> return-home              # start return journey
> hash                     # print canonical hash
> receipt                  # print full state summary
> quit                     # exit REPL
```

**Full smoke script:**

```sh
cargo run --bin chunk0_cli -- run-script smoke
```

## Documentation Map

| Path | Contents |
|---|---|
| `spec_pack/` | Corrected v0.3.3 Chunk 0 spec pack |
| `docs/elevated_reasoning/` | Elevated reasoning and Slice 2 freeze |
| `docs/playable_slice_completion/` | Bounded completion path docs |
| `agent_mapping_pass_01/` | Reconciled mapping / audit support |
| `chunk0_rust_scaffold/` | Verified Rust implementation |
| `CHUNK_0_PLAYABLE_SLICE_REPORT.md` | Final acceptance report |
| `CHUNK_0_IMPLEMENTATION_REPORT.md` | Full implementation history |
| `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md` | Harness / corpus documentation |
| `BUILD_HANDOFF_V0_3_3.md` | Build handoff reference |
| `archive/source_review/` | Historical provenance only — non-normative |

## Normative Hierarchy

1. `README.md` and `AGENTS.md` — repo workflow
2. `CHUNK_0_PLAYABLE_SLICE_REPORT.md`
3. `CHUNK_0_IMPLEMENTATION_REPORT.md`
4. `CHUNK_0_OPTIMIZATION_PASS_0_REPORT.md`
5. `spec_pack/`
6. `docs/elevated_reasoning/`
7. `docs/playable_slice_completion/`
8. `agent_mapping_pass_01/`
9. `chunk0_rust_scaffold/` — implementation truth

**Non-normative:** `archive/source_review/` — historical provenance only. Do not implement from this folder.

## Requirements

- Rust stable toolchain (`cargo`)
- No additional dependencies beyond `sha2 = "0.10"` (declared in `chunk0_rust_scaffold/Cargo.toml`)
