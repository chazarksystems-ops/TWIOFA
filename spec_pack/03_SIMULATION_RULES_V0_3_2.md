# 03 — Normative Simulation Rules

## Active-Region Policy

For Chunk 0, whole-chunk updates are required. Dirty-cell tracking is deferred.

## Normative Tick Order

This order is required for v0.3.2 implementation:

1. Apply queued command intent.
2. Execute ant group movement and task.
3. Execute digging / harvesting effects.
4. Execute gravity and collapse using bottom-up loop.
5. Execute water flow using deterministic priority.
6. Execute moisture diffusion using double-buffer (read from source buffer, write to destination buffer; order of cell updates cannot affect results; swap or copy dest to source after full pass).
7. Execute scent decay and moisture scent effects.
8. Execute scent reinforcement from ant path / harvest.
9. Generate perception events.
10. Produce command receipt / render frame data.

If implementation changes the order, tests and spec must be revised before coding continues.

## Determinism Policy

Allowed:

```text
fixed seed
stable command order
stable iteration order
BTreeMap if map iteration is required
```

Forbidden:

```text
SystemTime / wall-clock simulation changes
random memory-address seeds
HashMap iteration in receipts or hashes
async reads that affect simulation order
floating-point nondeterminism for core material rules
```

## Canonical Hash Rule

Hash a stable canonical serialization of simulation state, not raw Rust memory bytes.

Include:

```text
material
moisture
scent_home
scent_food
residue
support
flags
ant group state
tick index
deterministic command log position if applicable
```

Exclude:

```text
visual-only highlights
UI hover state
debug panel state
wall-clock timestamps
frame counters
non-authoritative overlay animation state
```

## Digging Rule

Pseudocode:

```text
if command/task is Dig and target is adjacent to ant group:
  if target material is Soil or LooseSoil:
    target.material = Tunnel
    target.flags |= RECENTLY_DUG
    target.support = 0
    for each cardinal neighbor:
      neighbor.support = saturating_sub(neighbor.support, 50)
    emit CellDug event
  else:
    emit CommandFailed(reason = NotDiggable)
```

## Collapse Rule

Pseudocode:

```text
for y from 126 down to 0:
  for x from 0 to 127:
    cell = (x,y)
    below = (x,y+1)
    if cell.material == LooseSoil and cell.support < 100:
      if below.material == Air or below.material == Tunnel:
        swap(cell, below)
        moved_cell.flags |= RECENTLY_COLLAPSED
        emit CollapseOccurred
```

Collapse is deterministic. There is no random collapse roll.

**Collapse + AntGroup interaction (v0.3.3, world-truth deterministic):**
If the destination cell of a collapse swap (the original `below` position) is the AntGroup's current cell at the time of resolution:
- The AntGroup receives the existing worker-loss consequence: lost = min(3, workers); workers -= lost; confidence = confidence.saturating_sub(16) clamped to [0,255].
- A WorkerLoss event is emitted.
- The consequence is included in the CommandReceipt (perception_updates and/or dev_event_summary) for the current or next command boundary.
- No health system, animation system, or combat system is invented or implied. This re-uses the exact residue worker-loss path for consistency.
- The material swap still occurs (world truth); the AntGroup is simply present in the cell that receives the falling material.

## Water Flow Rule

Water uses deterministic priority order.

For each water cell in bottom-up order:

```text
try down
then down-left
then down-right
then left
then right
```

A move is allowed only into `Air` or `Tunnel`. Water cannot move into `Stone`, `Root`, `Soil`, `LooseSoil`, `Carcass`, or `NestWall`.

When water moves, the water cell swaps with the destination cell and carries moisture value with it.

## Moisture Diffusion Rule

Moisture diffusion is deterministic and conservative enough for Chunk 0. It uses an explicit double-buffer so that update order cannot affect results.

**Double-buffer contract (locked v0.3.3):**
- Source buffer: the moisture values at the start of the diffusion step (copy of current tick state).
- Destination buffer: a separate array initialized to the source values (or zeroed + deltas).
- For every cell (in any stable order, e.g. row-major y*128+x):
  - Read current and neighbor moisture exclusively from the source buffer.
  - If source[cell].moisture > source[neighbor].moisture + 16:
    - transfer = 4
    - if neighbor material can accept moisture:
      - dest[cell].moisture = source[cell].moisture - transfer
      - dest[neighbor].moisture = source[neighbor].moisture + transfer
- After processing all cells and all neighbors, copy the entire destination buffer back to the source buffer (or swap the two arrays).
- The in-place single-buffer "stable pass" variant is not used; double-buffer guarantees identical final state regardless of traversal order.

Materials that accept moisture:

```text
Air: yes, low retention
Tunnel: yes, low retention
Soil: yes, slow retention
LooseSoil: yes, slow retention
Water: source/saturated
Carcass: no for Chunk 0
Root: no
Stone: no
NestWall: no
```

Wet `LooseSoil` support effect:

```text
if LooseSoil.moisture > 120:
  support = support.saturating_sub(1 per tick)
```

## Scent Decay and Moisture Effect

Base decay per tick:

```text
scent_home = saturating_sub(scent_home, 1)
scent_food = saturating_sub(scent_food, 1)
```

Water cell effect:

```text
if material == Water:
  scent_home = 0
  scent_food = 0
```

Wet cell extra effect:

```text
if moisture > 100:
  scent_home = saturating_sub(scent_home, 2)
  scent_food = saturating_sub(scent_food, 2)
```

## Scent Reinforcement Rule

When ant group moves through a traversable cell:

```text
if task is ReturnHome:
  scent_home = min(255, scent_home + 12)
if task is Forage and carrying/near food:
  scent_food = min(255, scent_food + 12)
```

When carcass harvest succeeds:

Fixed deterministic rule (v0.3.3):
- If the ant group's current cell is traversable (Air/Tunnel/Water per movement rules), the current cell receives: scent_food = min(255, scent_food + 24).
- Otherwise, select the first adjacent traversable cell in fixed neighbor order: up, right, down, left. Apply +24 to that cell only.
- Clamp result to [0, 255] in all cases.
- This is world-truth update; no perception required for the scent itself.

## Residue Effects

`SourbackBitter` has two distinct layers:

```text
WorldTruth: exact residue enum is SourbackBitter
ColonyView: bitter/yellow residue until interpreted later
```

Deterministic movement slowdown:

```text
if ant group is on SourbackBitter and task is Scout or Forage:
  ant may move only on even tick_index
  odd tick emits AntGroupSlowed if not already emitted for this command
```

Deterministic worker loss for Chunk 0:

```text
if task is Forage and ant enters SourbackBitter for the first time during that command:
  lost = min(3, workers)
  workers -= lost
  confidence = confidence.saturating_sub(16)  # fixed penalty; clamped to [0, 255]
  emit WorkerLoss
```

This is a prototype pressure rule, not full Sourback creature simulation.

## Carcass Harvest Rule

A carcass cell is one harvestable unit in Chunk 0.

```text
if task is Forage and ant is adjacent to a Carcass cell:
  convert one deterministic adjacent Carcass cell to Air
  food_carried += 1
  emit CarcassHarvested
```

Deterministic adjacent target order:

```text
up, right, down, left
```

No harvesting through solid soil.

## Command Receipt Generation

Every command produces a `CommandReceipt` containing at minimum:

```text
command_id
tick_start
tick_end
chunk_hash_before
chunk_hash_after
chunk_deltas
perception_updates
dev_event_summary
debug_stats
```
