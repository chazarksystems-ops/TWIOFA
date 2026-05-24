# 04 — Ant Group State Machine and Orders

## Ant Group Model

Chunk 0 uses one ant group represented by a single coordinate.

Minimum fields:

```text
id
pos: (x, y)
workers: u32
task
food_carried: u32
fatigue: u8
confidence: u8
task_memory / command-local flags
```

## Task Enum

```text
Idle
Scout(target)
Dig(target)
Forage(target)
ReturnHome
Avoid(target_region)
```

## Route Model

Chunk 0 uses a minimal hybrid route model:

```text
scent gradients in cells
+ simple ant group memory / target intent
```

There is no A* pathfinding and no explicit route graph.

Movement toward a target uses deterministic greedy stepping.

## Movement Rule

Ant group may move through:

```text
Air
Tunnel
Water, but slow/costly
```

Ant group may not move through:

```text
Soil
LooseSoil
Carcass
Root
Stone
NestWall
```

Greedy target step priority:

1. reduce x distance if possible;
2. reduce y distance if possible;
3. try alternate cardinal directions in fixed order: up, right, down, left;
4. if no move is possible, emit `CommandFailed(Blocked)` or remain idle depending command type.

## Command Payloads

| Command | Payload | Purpose |
|---|---|---|
| `StepSimulation` | `{ ticks: u32 }` | Advance material simulation. |
| `DigTunnel` | `{ target: Coord }` | Dig adjacent soil/loose soil into tunnel. |
| `SendForagers` | `{ target: Coord }` | Move toward carcass and harvest if accessible. |
| `ScoutResidue` | `{ target: Coord }` | Move/scout and emit perception events. |
| `ReturnHome` | none | Move toward nest/tunnel/home scent and deposit carried food. |
| `Avoid` | `{ target_region_or_coord }` | Set perception/assignment policy, not WorldTruth. |
| `InspectCell` | `{ x, y, mode }` | Return inspection data without mutating sim. |
| `Reset` | optional seed | Restore initial fixture. |

## Command Preconditions and Failure Events

| Command | Preconditions | Failure event |
|---|---|---|
| `DigTunnel` | target adjacent and material is `Soil` or `LooseSoil` | `CommandFailed(NotAdjacent / NotDiggable)` |
| `SendForagers` | workers > 0 | `CommandFailed(NoWorkers / Blocked / NoCarcassAccess)` |
| `ScoutResidue` | workers > 0 | `CommandFailed(NoWorkers / Blocked)` |
| `ReturnHome` | workers > 0 | `CommandFailed(NoWorkers / NoKnownHomeRoute)` |
| `Avoid` | target region/cell valid | `CommandFailed(InvalidTarget)` |
| `InspectCell` | coordinates in bounds | `CommandFailed(OutOfBounds)` |

## Avoid Rule

Avoid does not alter `WorldTruth`.

Avoid may alter:

```text
ColonyPerception
assignment policy
confidence/risk marker
future command suggestions
```

Avoid must not:

```text
destroy routes
change materials
remove residue
remove carcass
move water
```

## Worker Loss Rule

For Chunk 0:

```text
Forage + first entry into SourbackBitter during command = deterministic loss of min(3, workers)
```

Loss emits a `WorkerLoss` event, perception receipt, and applies fixed confidence penalty: confidence = confidence.saturating_sub(16), clamped to [0, 255]. (See spec_pack/03 for exact pseudocode.)

## Fatigue

Fatigue is allowed but must remain simple.

Required only if used to prove residue/moisture slowdown.

If fatigue is not implemented in the first build, it must be explicitly marked as deferred rather than half-implemented.

## Confidence

Confidence is a simple colony-facing scalar or band.

Confidence decreases when:

```text
worker loss occurs (fixed penalty -16, clamped [0,255])
bitter/yellow residue is observed
forage route is blocked/collapsed
```

Confidence increases when:

```text
scout returns without loss
food is harvested and returned
route remains passable after ticks
```
