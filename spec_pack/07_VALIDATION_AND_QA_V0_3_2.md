# 07 — Validation and QA

## Fixture List (v0.3.3 expanded for determinism)

| Fixture | Starting Conditions | Expected Result Shape |
|---|---|---|
| `FIXTURE_INITIAL` | Exact band layout from spec_pack/02 (ant at 55,118; carcass 90-111/18-28; etc.); all defaults applied in table order. | Full 128x128 WorldTruth; known initial chunk_hash; ant state Idle, workers>0. |
| `FIXTURE_WEAK_ROOF` | AntGroup at (55,100); cell (55,99)=LooseSoil support=40; cell (55,100)=Tunnel (open below); no residue. | StepSimulation(1-3): LooseSoil swaps down; RECENTLY_COLLAPSED flag; CollapseOccurred event; support change; receipt has CellChanged + delta. |
| `FIXTURE_MOISTURE_SCENT` | Water at (65,70) moisture=255; adjacent traversable cell with scent_food=50; ant elsewhere. | StepSimulation(5): adjacent scent reduced more than control dry cell; moisture diffuses per double-buffer rule; no order-dependent variance. |
| `FIXTURE_RESIDUE_ROUTE` | AntGroup at (85,30) on SourbackBitter path; task=Forage; workers=10; confidence=200. | Forage command: even-tick movement only; AntGroupSlowed + WorkerLoss events; workers -= min(3,10); confidence -=16 clamped; receipt includes perception_updates. |
| `FIXTURE_CARCASS_HARVEST` | AntGroup at (95,25) adjacent to Carcass (96,25); task=SendForagers; traversable path. | SendForagers: one Carcass -> Air; food_carried +=1; CarcassHarvested event; if on traversable, current cell +24 scent_food (clamped); receipt has chunk_deltas + harvest event. |
| `FIXTURE_BLOCKED_DIG` | AntGroup at (55,95); target (56,95) is Stone (or boundary ring Stone); command=DigTunnel. | CommandFailed(NotDiggable); no cell change; chunk_hash unchanged; receipt has failure reason and dev_event_summary; no material delta. |
| `FIXTURE_BASIC_DIG` | AntGroup at (55,118); target (56,118) is Soil (traversable-adjacent); command=DigTunnel. | Target becomes Tunnel; RECENTLY_DUG flag; cardinal neighbors support -=50; CellDug event; CommandReceipt with chunk_deltas (one CellMaterial change) + hash delta. |
| `FIXTURE_COLLAPSE_RISK` | Same as WEAK_ROOF but with AntGroup at the destination cell (55,100) when LooseSoil at (55,99) support<100. | Collapse swap occurs; AntGroup receives worker-loss consequence (min(3,workers) lost, confidence -=16 clamped, WorkerLoss emitted); receipt/perception includes the loss; material truth updated. |
| `FIXTURE_HARVEST_SCENT` | AntGroup on non-traversable (e.g. edge of Sourback); successful carcass harvest adjacent; first neighbor in up/right/down/left order is traversable. | Scent +24 applied only to the first valid adjacent traversable cell (not current); clamped [0,255]; no change to non-traversable cell. |
| `FIXTURE_BOUNDARY_DIG_BLOCKED` | AntGroup adjacent to x=0 or y=0 or x=127 or y=127 Stone cell; DigTunnel on the boundary Stone. | Blocked; CommandFailed(NotDiggable) receipted; no mutation of immutable boundary Stone; WorldTruth ring remains Stone. |

## Validation Test Matrix

| Test ID | Validates | Fixture | Action | Pass Condition |
|---|---|---|---|---|
| C0-TEST-DET-1 | Determinism | INITIAL | Same command list run 3 times | Final chunk hash identical. |
| C0-TEST-DET-2 | Hash excludes UI | INITIAL | Change hover/overlay state only | Chunk hash unchanged. |
| C0-TEST-MAT-1 | Digging | INITIAL | `DigTunnel(target)` | Target `Soil/LooseSoil` becomes `Tunnel`. |
| C0-TEST-MAT-2 | Collapse | WEAK_ROOF | `StepSimulation(5)` | Loose soil moves into open tunnel/air below. |
| C0-TEST-MAT-3 | Water flow | INITIAL | `StepSimulation(N)` | At least one water cell moves or moisture changes deterministically. |
| C0-TEST-MAT-4 | Moisture/scent interaction | MOISTURE_SCENT | `StepSimulation(10)` | Scent decreases more in wet/water cell than dry control. |
| C0-TEST-MAT-5 | Scent decay | INITIAL | seed scent, `StepSimulation(10)` | Scent values decrease by expected deterministic amount. |
| C0-TEST-MAT-6 | Scent reinforcement | INITIAL | ant moves/harvests | Relevant scent increases on path/cell. |
| C0-TEST-MAT-7 | Carcass harvest | CARCASS_HARVEST | `SendForagers` | One carcass cell becomes Air and harvest event emits. |
| C0-TEST-ANT-1 | Residue slowdown | RESIDUE_ROUTE | scout/forage through residue | AntGroupSlowed event and slower progress. |
| C0-TEST-ANT-2 | Worker loss | RESIDUE_ROUTE | Forage through residue | WorkerLoss emits deterministic loss. |
| C0-TEST-PERC-1 | Truth/perception split | INITIAL | Inspect colony view before scout | Hidden Sourback label absent. |
| C0-TEST-PERC-2 | Bitter residue observation | RESIDUE_ROUTE | `ScoutResidue` | PhysicalFact says bitter/yellow residue, not Sourback. |
| C0-TEST-PERC-3 | Later reframe | INITIAL | Scout residue, then carcass edge | Reframe/Correction links earlier bitter residue evidence. |
| C0-TEST-CMD-1 | Command failure | BLOCKED_DIG | invalid `DigTunnel` | CommandFailed emitted; chunk unchanged except receipt/log. |
| C0-TEST-UI-1 | Anti-card UI | Visual manual | Boot app | 128x128 canvas/grid is central surface. |
| C0-TEST-UI-2 | Grid-first receipt | Visual manual | Issue dig command | Grid changes visibly before debug event panel summary. |
| C0-TEST-UI-3 | DevTruth toggle | Visual manual | Inspect same cell in two modes | ColonyView hides hidden semantic truth; DevTruth shows it. |

## Manual Visual Smoke Path

1. Boot application.
2. Verify 128x128 material grid/canvas is the primary visible element.
3. Toggle Material / Moisture / Scent / Residue overlays.
4. Issue `DigTunnel` near the nest/tunnel edge.
5. Verify a soil pixel becomes tunnel and recent-dig marker appears.
6. Run ticks and observe loose soil / moisture / scent changes.
7. Send foragers toward carcass edge.
8. Inspect residue in ColonyView: should show bitter/yellow sign, not hidden Sourback semantics.
9. Toggle DevTruth and inspect same cell: debug truth may show `SourbackBitter`.
10. Confirm debug event panel supports the grid but does not dominate the UI.

## Automated Smoke Path

Command sequence for deterministic smoke:

```text
Reset(seed=CHUNK0_FIXED)
InspectCell(55,118, ColonyView)
DigTunnel(target near nest)
StepSimulation(5)
ScoutResidue(target near residue path)
SendForagers(target carcass edge)
ReturnHome
```

Expected:

```text
chunk hash stable across runs
at least one CellChanged event
at least one PhysicalFact residue event
no unearned Sourback label in ColonyView
one carcass delta if foragers reach accessible edge
WorkerLoss event if foragers cross residue
```

The exact golden hash should be generated only after implementation is approved and first deterministic implementation exists.
