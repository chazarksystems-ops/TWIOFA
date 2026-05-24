# Agent Reasoning Elevation Ledger

## Status
- **Normative status:** Elevated reasoning guidance. This ledger records which agent reasoning can be reused and which must not be reintroduced.
- **Scope:** Slice 2 semantics, optimization discussion, Geneflux harvesting, and future-agent guardrails.

## What this document does not authorize
- Does not authorize Slice 3.
- Does not authorize Optimization Pass 0 implementation.
- Does not authorize new mechanics or full-game canon.
- Does not authorize `source_review/` as normative input.

## Ledger
| Source / moment | Claim or reasoning | Status | Why | Resulting rule / note |
|---|---|---|---|---|
| Coordinate audit | Map bands use half-open range reasoning. | PROMOTED | Matches Rust `start..end` semantics used by implementation. | Treat `28..36` as 28 through 35. |
| Tick audit | `tick_index` starts at 0; first executed tick is 1. | PROMOTED | Explains first odd tick slowdown behavior. | Tests must account for tick 1 being odd. |
| Sourback slowdown audit | Odd tick while on Sourback slows Scout/Forage. | PROMOTED | Matches Slice 2 implementation and tests. | `AntGroupSlowed` emits once per command. |
| Worker-loss arithmetic | `min(3, workers)`. | PROMOTED | Prevents worker underflow and matches D12. | Use exact min rule. |
| Confidence arithmetic | `confidence.saturating_sub(16)`. | PROMOTED | Prevents u8 underflow/wrap. | Confidence stays in `[0,255]`. |
| Blocked fixture design | Contrived blocked fixture is valid as unit isolation. | PROMOTED | Default map does not naturally provide a clean blocked pocket. | Test comments must say fixture is unit isolation, not map claim. |
| Greedy tie tests | Tests must use traversable fixtures. | PROMOTED | A coordinate like `(50,50)` may be non-traversable in default map. | Explicitly assert or set traversable cells before tie tests. |
| Hash boundary | Command-local flags should not be included in stable post-command hash. | PROMOTED | Current replay model hashes stable post-command state. | If mid-command resume is required later, serialize scratch or replay command. |
| Earlier Slice 2 code | Destination-only Sourback entry was wrong. | CORRECTED | It counted Sourback -> Sourback as entry. | Do not reintroduce destination-only WorkerLoss. |
| Semantics audit | Sourback -> Sourback movement is not “entry.” | CORRECTED | Entry requires crossing into the zone from outside. | No WorkerLoss for Sourback -> Sourback. |
| Semantics audit | Starting on Sourback does not itself trigger WorkerLoss. | CORRECTED | Starting position is not entry. | WorkerLoss needs movement crossing from non-Sourback to Sourback. |
| Corrected implementation | Transition-based Sourback entry. | PROMOTED AFTER CORRECTION | Matches ordinary entry semantics and tests. | Use `!was_on_sourback && dest_on_sourback`. |
| Gemini optimization discussion | Monte Carlo/MCMC balancing. | DEFERRED | No stable tunable parameters or objective metrics yet. | Revisit after deterministic harness and metrics exist. |
| Gemini optimization discussion | MCTS/adversarial bot search. | DEFERRED | Command API and interaction space are not rich enough yet. | Future adversarial command search only. |
| Optimization discussion | GPU/Margolus/block CA. | DEFERRED | CPU correctness and profiling must come first. | Future research only. |
| Optimization discussion | Active chunk/sleep lists. | DEFERRED | Metrics are needed before optimizing scan regions. | Collect active-region counters first. |
| Replay discussion | Full snapshot/replay envelope. | DEFERRED | Command/hash model should stabilize first. | Design notes now; richer envelope later. |
| Geneflux review | Geneflux biology/gameplay import. | REJECTED FOR CHUNK 0 | TWIOFA should not import those mechanics. | Use patterns, not gameplay. |
| Geneflux review | DNA/genome model. | REJECTED FOR CHUNK 0 | Out of scope and wrong abstraction. | Do not implement. |
| Geneflux review | Lineage registry. | REJECTED FOR CHUNK 0 | Not needed for Chunk 0 substrate. | Do not implement. |
| Geneflux review | HGT. | REJECTED FOR CHUNK 0 | Biology mechanic import, not needed. | Do not implement. |
| Geneflux review | Replication. | REJECTED FOR CHUNK 0 | Not a Chunk 0 material-grid requirement. | Do not implement. |
| Geneflux review | Lysis. | REJECTED FOR CHUNK 0 | Imported biology mechanic. | Do not implement. |
| Geneflux review | Dormancy. | REJECTED FOR CHUNK 0 | Imported biology mechanic. | Do not implement. |
| Geneflux review | Hex-grid coordinate system. | REJECTED FOR CHUNK 0 | Chunk 0 is 128x128 square grid. | Do not replace grid. |
| Geneflux review | Microbial ecology phase order. | REJECTED FOR CHUNK 0 | Wrong simulation domain. | Do not import. |
| Prior UI risk | Card-dashboard gameplay. | REJECTED FOR CHUNK 0 | Conflicts with material-yard substrate. | Keep forbidden. |
| Movement implementation | Greedy fallback may move away from target. | WATCHLIST | Deterministic and documented, but should remain tested. | Keep if spec supports; document and test it. |
| Save/replay | Command-boundary save semantics. | WATCHLIST | Acceptable now, but mid-command resume changes requirements. | Serialize scratch or replay from boundary if needed later. |
| Future deltas | Staged deltas must avoid nondeterministic conflict resolution. | WATCHLIST | Collapse/moisture will need deterministic conflict rules. | Enforce scan order and conflict policy in Slice 3/4. |

Future agents must cite this ledger when revisiting movement, worker loss, optimization, Geneflux reuse, or Gemini optimization ideas.
