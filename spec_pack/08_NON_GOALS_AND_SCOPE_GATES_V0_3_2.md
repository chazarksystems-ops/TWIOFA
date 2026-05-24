# 08 — Non-Goals and Scope Gates

## Hard Non-Goals

Do not build:

```text
full yard
procedural world generation
final UI
Unreal integration
GPU-owned simulation
custom GPU compute
multiplayer
full packet campaign layer
A* pathfinding
full ant AI
full food economy
doctrine system
Deep Queen layer
spider/centipede arcs
complete biology simulation
shader-heavy presentation
card-dashboard gameplay
```

## Scope Gate

A proposed feature belongs in Chunk 0 only if it directly proves at least one of:

```text
material cell behavior
digging
collapse
moisture
scent
residue
carcass harvesting
ant-group interaction
perception split
deterministic replay
visible material-grid change
```

If it does not support one of those, defer it.

## UI Gate

Allowed UI work:

```text
central grid/canvas
overlay toggles
cell inspection
command buttons/keys
debug event panel
chunk hash/readout
```

Forbidden UI work:

```text
card gameplay
polished menus
lore screens
final HUD design
cosmetic animation focus
text-only simulation surface
```

## Backend Gate

Allowed backend work after approval:

```text
chunk cell arrays
simulation ticks
deterministic command handling
hash/replay tests
perception events
render-frame generation
```

Forbidden backend work:

```text
networking
database persistence
large save system
plugin architecture
full ECS migration
AI director
procedural generation
```

## Agent Must Stop If

An agent must halt and ask for approval if it intends to add:

```text
another UI framework
full yard generation
Unreal integration
custom GPU compute
A* pathfinding
multiplayer
card-dashboard gameplay
extra creatures beyond residue/pressure
full Field Notes archive UI
new canon/lore
```

## Human Approval Required

Before implementation:

```text
repo target
visualizer stack
cell schema
coordinate bands
tick order
command set
test list
dependency policy
RenderFrame truth boundary
```
