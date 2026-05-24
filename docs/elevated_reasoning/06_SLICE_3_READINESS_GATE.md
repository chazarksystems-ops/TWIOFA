# Slice 3 Readiness Gate

## Status
Slice 3 is not authorized by this document.

## Normative status
Gate/checklist guidance. This document does not replace Chaz approval or the v0.3.3 spec pack.

## What this document does not authorize
- Does not authorize Slice 3 implementation.
- Does not authorize Optimization Pass 0 implementation.
- Does not authorize moisture, scent, harvest, UI, procedural yard, pathfinding, doctrine/economy/combat, GPU/Margolus, MCTS/MCMC, or card-dashboard behavior.

## Gate checklist
- [ ] Slice 2 freeze report exists
- [ ] transition-based Sourback entry documented
- [ ] current test count documented as 27/27 passing or newer equivalent
- [ ] `CHUNK_0_IMPLEMENTATION_REPORT.md` has no stale movement-deferred contradiction
- [ ] Agent reasoning ledger exists
- [ ] Geneflux harvest map exists
- [ ] Gemini optimization review exists
- [ ] Optimization Pass 0 design exists
- [ ] Optimization Pass 0 harness implemented OR Chaz explicitly approves deferring it
- [ ] baseline corpus can run OR defer approved
- [ ] invariants checked after corpus scripts OR defer approved
- [ ] no `source_review/` normative use
- [ ] no card-dashboard regression
- [ ] no hidden truth leak
- [ ] no new mechanics introduced in docs
- [ ] Chaz explicitly approves Slice 3 start

## Slice 3 allowed initial target
Only:

- deterministic bottom-up gravity/collapse
- no random roll
- no moisture
- no scent
- no harvest
- no UI
- no pathfinding
- no procedural yard

## Slice 3 recommended first constraints
- use staged-delta design
- deterministic scan order
- explicit conflict resolution
- AntGroup landing/impact interaction follows existing worker-loss consequence if specified
- receipts/perception events included
- no new health/combat system

## Slice 3 forbidden
- moisture diffusion
- scent decay/reinforcement
- carcass harvest
- Tauri/UI
- pathfinding
- procedural yard
- GPU/Margolus
- MCTS/MCMC
- doctrine/economy/combat
- card dashboard

## Required prompt shape for Slice 3
```text
Read BUILD_HANDOFF_V0_3_3.md, CHUNK_0_IMPLEMENTATION_REPORT.md, spec_pack/03, spec_pack/04, spec_pack/05, spec_pack/07, and docs/elevated_reasoning/06_SLICE_3_READINESS_GATE.md.
Confirm the Slice 3 gate.
Implement only deterministic bottom-up collapse.
Add fixture tests for collapse and AntGroup landing interaction.
Run cargo fmt --check, cargo check, cargo test.
Update CHUNK_0_IMPLEMENTATION_REPORT.md.
Stop.
```

## Stop conditions
- ambiguous collapse/AntGroup semantics
- nondeterministic conflict resolution
- hidden truth leak
- harness baseline fails
- `source_review/` used as normative input
- card-dashboard wording returns
- agent tries to implement moisture/scent/harvest/UI
