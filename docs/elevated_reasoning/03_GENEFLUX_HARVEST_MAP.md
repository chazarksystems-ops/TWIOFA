# Geneflux Harvest Map

## Status
- **Normative status:** Pattern-harvest guidance only. This document does not import Geneflux gameplay into TWIOFA.
- **Scope:** Rust determinism, custody, replay, and audit patterns that may be useful for TWIOFA.

## What this document does not authorize
- Does not authorize Slice 3.
- Does not authorize copying Geneflux code wholesale.
- Does not authorize DNA/genome, HGT, replication, lysis, dormancy, lineage, microbial phase order, or hex-grid gameplay.

## Verdict
Geneflux helps TWIOFA as a Rust determinism/custody reference. It should not donate gameplay mechanics to Chunk 0.

## Harvestable patterns
| Pattern | What Geneflux does | TWIOFA equivalent | When to adopt | Risk if copied too literally |
|---|---|---|---|---|
| authoritative/session/telemetry separation | Separates truth state from run/session outputs and measurement. | WorldTruth vs CommandReceipt vs optimization artifacts. | Adopt naming and structure now. | Gameplay truth could be polluted by telemetry. |
| `TickContext` / scratch-state pattern | Uses per-tick scratch separate from authoritative state. | `CommandScratch` / `CommandFlags`. | Adopt now as documentation; refine type names later. | Scratch could become hidden authoritative state if hashed incorrectly. |
| CanonicalEncode-style trait | Encodes state deterministically for hashes/replay. | `write_canonical_bytes` / future trait. | Adopt as design note now; formalize when state grows. | Overengineering before more state types exist. |
| `encode_for_state_hash` / `recompute_state_hash` boundary | Keeps hash input explicit. | `compute_chunk_hash` boundary. | Adopt now as a design rule. | Raw memory hashing or UI state leakage. |
| snapshot/replay envelopes | Captures replayable run context. | Future command script + hash-sequence envelope. | Adopt later after command model stabilizes. | Premature bulky format. |
| replay determinism tests | Proves repeatability across command sequences. | Corpus and hash sequence tests. | Adopt now. | Too much replay machinery before Slice 3 if overbuilt. |
| staged deltas | Applies changes deterministically after collection. | Collapse/moisture staged changes. | Adopt for Slice 3/4 design. | Conflict rules can become nondeterministic if vague. |
| `#![forbid(unsafe_code)]` | Forbids unsafe Rust at crate level. | Same crate-level policy candidate. | Adopt after checking current scaffold friction. | May block legitimate future FFI if applied without exception policy. |

## TWIOFA mapping
| Geneflux-inspired role | TWIOFA role |
|---|---|
| Authoritative hash-owned state | WorldTruth |
| Command output / audit artifact | CommandReceipt |
| Command-local non-authoritative scratch | CommandScratch / CommandFlags |
| Derived view | RenderFrame / ColonyView |
| Debug-only view | DevTruth |
| Telemetry/artifacts | Optimization harness outputs, not gameplay truth |

## Adopt now
- Naming/documentation for CommandScratch.
- Replay hash sequence tests.
- Invariant checks.
- Canonical encode trait design notes.
- Staged delta design notes for Slice 3/4.

## Adopt later
- Snapshot/replay envelope.
- Config hash boundary.
- Richer replay corpus.
- Baseline comparison files.

## Do not adopt
- DNA/genome model.
- Genotype IDs.
- Lineage registry.
- HGT.
- Replication.
- Lysis.
- Dormancy.
- Microbial phase order.
- Hex-grid neighbors.
- Biological cell economy.

## Slice-specific use
| Slice / pass | Use |
|---|---|
| Slice 2 | CommandScratch framing. |
| Optimization Pass 0 | Replay corpus and invariant harness. |
| Slice 3 | Staged deltas for collapse. |
| Slice 4 | Double-buffer/delta discipline for moisture. |
| Slice 5 | Event/replay discipline for scent/harvest. |

## Guardrail
No Geneflux code should be copied wholesale unless separately reviewed. Use inspected patterns only.
