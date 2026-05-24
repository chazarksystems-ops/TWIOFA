# 00 — Chunk 0 Purpose and Boundaries

## Purpose

**TWIOFA Chunk 0** is the first material-yard substrate spec for *The World Is on FIRE Ants*.

It exists to test whether TWIOFA can support a Noita-inspired physical world without becoming a Noita clone. The prototype should prove one small, inspectable, physically changing chunk where colony-scale ant decisions interact with soil, tunnels, moisture, scent, residue, carcass material, and collapse.

The purpose is not to build the whole game. The purpose is to prove the material foundation.

## Core Direction

TWIOFA should move toward:

```text
Noita-like material freedom
+ colony-scale ant strategy
+ persistent yard memory
+ physical evidence
+ perception / misinterpretation
+ later rich spatial UI/UX
```

The player should eventually feel like they are interacting with a living yard, not selecting options from a card dashboard.

## Chunk 0 Definition

Chunk 0 is one bounded simulatable material area:

```text
128 x 128 cells
one nest chamber
one tunnel / partial tunnel
one surface / grassline band
one carcass edge
one moisture source
one Sourback residue path
one scent trail
one dig rule
one collapse rule
one ant group
```

## Locked Prototype Claims

| ID | Claim | Observable? | Testable? |
|---|---|---:|---:|
| C0-P1 | Soil and tunnel cells can physically change. | Yes | Yes |
| C0-P2 | Ant-group digging can alter the map. | Yes | Yes |
| C0-P3 | Moisture can change route safety or material behavior. | Yes | Yes |
| C0-P4 | Scent can guide, decay, smear, or mislead. | Yes | Yes |
| C0-P5 | Sourback residue can affect ant behavior and route confidence. | Yes | Yes |
| C0-P6 | Carcass material can be harvested as a physical place. | Yes | Yes |
| C0-P7 | Collapse can emerge from weakened/support-poor terrain. | Yes | Yes |
| C0-P8 | Physical facts and colony interpretation can diverge. | Yes | Yes |
| C0-P9 | Simulation can be deterministic enough for replay/debugging. | Debug-visible | Yes |

## Definition of Done

Chunk 0 is complete only when:

1. all required validation tests pass;
2. a 128x128 material grid/canvas is the central visible surface;
3. digging visibly changes cells;
4. moisture/water visibly affects cells or overlays;
5. scent visibly changes or can be inspected;
6. collapse visibly changes the chunk;
7. carcass harvesting physically changes carcass cells;
8. ColonyView and DevTruth inspection are separated;
9. command receipts and chunk hashes are deterministic;
10. no card-dashboard gameplay is introduced.

## Definition of Not Done

Chunk 0 is not done if:

- food counters change but carcass cells do not;
- logs/cards/panels are the main gameplay surface;
- the material grid is hidden behind menus;
- the simulation depends on wall-clock time;
- hidden Sourback truth leaks into ColonyView before observation;
- implementation relies on A* pathfinding, procedural generation, or GPU-owned simulation;
- tests require the visualizer to pass instead of testing the simulation headlessly.

## Hard Non-Goals

Do not build:

```text
full yard
procedural world generation
final Unreal UI
GPU compute / GPU-owned simulation
multiplayer
full packet campaign layer
full ant AI
full food economy
doctrine system
Deep Queen layer
spider/centipede arcs
complete biology simulation
card-dashboard gameplay
```
