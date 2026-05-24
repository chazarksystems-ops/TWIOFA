# 02 — Numeric Chunk Layout and Initial Scene

## Coordinate Convention

The chunk is 128x128.

```text
x: 0..128 left to right
y: 0..128 top to bottom
index = y * 128 + x
```

Coordinate ranges are half-open:

```text
x_start..x_end includes x_start and excludes x_end
y_start..y_end includes y_start and excludes y_end
```

## Seeded Layout Contract

For Chunk 0, the initial layout is hardcoded deterministically. Procedural generation is banned.

The seed may be accepted and stored for future compatibility, but it must not alter this v0.3.2 layout unless Chaz approves a later seeded layout spec.

## Boundary Behavior (v0.3.3 locked)

The boundary ring cells at x=0, x=127, y=0, and y=127 are **explicit Stone cells in WorldTruth**.
- They are immutable during Chunk 0.
- Attempts to dig boundary Stone (or any Stone) are blocked; a `CommandFailed(NotDiggable)` receipt is emitted. No material change occurs.
- Water, ant movement, scent, moisture, and collapse cannot cross or alter the ring.
- `y=124..128` (including y=127) is the stone floor band (explicit Stone, support 255).
- The full ring (left/right/top/bottom edges) is Stone in the initial layout bands (overrides default Soil/Air).
- No wrapping; nothing enters or exits the 0..128 half-open chunk.

## Exact Coordinate Bands

Apply default soil first, then overrides in table order.

| Element | X Range | Y Range | Initial State |
|---|---|---|---|
| Air / surface | `0..128` | `0..36` | `Material::Air`, support 0 |
| Carcass edge | `90..111` | `18..28` | `Material::Carcass`, support 100 |
| Sourback residue path | `80..116` | `28..36` | `Material::Air`, `residue = SourbackBitter` |
| Root column | `30..36` | `36..116` | `Material::Root`, support 255 |
| Water pocket | `60..71` | `64..73` | `Material::Water`, moisture 255, support 0 |
| Loose soil risk band | `50..81` | `80..86` | `Material::LooseSoil`, support 60 |
| Nest / initial tunnel | `45..66` | `96..121` | `Material::Tunnel`, support 0 |
| Ant group start | `x=55` | `y=118` | AntGroup state: Idle |
| Stone floor | `0..128` | `124..128` | `Material::Stone`, support 255 |
| Soil default | all other non-overridden cells | all other non-overridden cells | `Material::Soil`, support 200 |

## ASCII Mini-Map

Conceptual reference only; numeric bands above are authoritative.

```text
[Y: 00-15] .................................... Air / surface
[Y: 16-27] .............................OOOOO.. Carcass edge
[Y: 28-35] ...........................SSSSSSS.. Sourback residue path
[Y: 36-63] #######RR########################### Soil + root column
[Y: 64-83] #######RR#########WW################ Water pocket
[Y: 84-95] #######RR#########LLLL############## Loose soil risk
[Y: 96-115]#######RR###########  ############## Partial tunnel/nest approach
[Y: 116-123]################## @ ############## Nest and ant group
[Y: 124-127]XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX Stone bedrock
```

Legend:

```text
. = Air
# = Soil
R = Root
W = Water
L = LooseSoil
O = Carcass
S = Sourback residue over Air
@ = Ant group start
X = Stone
space = Tunnel / nest chamber
```

## Scene Question

The initial scene should create this development question:

```text
Can the colony physically open and maintain a route to food without misreading the danger in the material path?
```
