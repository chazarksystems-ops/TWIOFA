# 06 — RenderFrame, Visualizer, and Inspection

## Main Surface Rule

The material grid/canvas is the main surface.

Debug panels support the grid.

Debug panels must never become the game.

## Default Visualizer

Default visualizer after approval:

```text
Rust simulation core + Tauri + HTML canvas
```

Allowed only if:

1. central game surface is a canvas rendering the 128x128 material chunk;
2. HTML/DOM is limited to controls, debug panels, and inspectors;
3. simulation remains Rust-owned;
4. no card-dashboard gameplay is created.

Fallback if Tauri becomes distracting:

```text
minifb or pixels native visualizer
```

## RenderFrame Contract

Rust owns WorldTruth. The default visualizer receives a derived `RenderFrame`, not raw hidden simulation truth.

RenderFrame fields:

```text
tick_index
chunk_hash
visible_cells or pixel/material render buffer
overlay_mode
chunk_deltas
perception_markers
debug_stats
```

RenderFrame must exclude hidden semantic truth unless explicit DevTruth mode is enabled.

## VisibleCell Contract

For ColonyView/default rendering, each visible cell may include:

```text
coord
visible_material_category
visible_color_id or palette index
visible_overlay_value for selected overlay
known_perception_marker if observed
recent_delta_marker
```

It must not include:

```text
hidden residue semantic meaning
hidden hazard identity
exact hidden collapse probability
unearned Sourback labels
raw debug-only fields
```

## Inspection Modes

### ColonyView Mode — Default

Player-facing presentation.

Hover/click shows only colony-accessible information:

```text
observed material category
observed moisture/scent/residue signs
known or suspected interpretation
confidence band
related perception events
```

### DevTruth Mode — Explicit Toggle

Development tool only.

May expose raw WorldTruth for selected cell or debug dump:

```text
material
moisture
scent_home
scent_food
residue enum
support
flags
hidden semantic meaning
```

DevTruth mode is not a player-facing mode.

## Overlay Modes

Required overlays:

| Overlay | Shows | Default truth exposure |
|---|---|---|
| Material | visible material category | Colony-safe |
| Moisture | observed/visible moisture or debug moisture | Colony-safe unless DevTruth toggle for exact values |
| Scent | home/food scent intensity | Colony-safe if scent observed/accessible |
| Residue | known bitter/yellow residue markers | Colony-safe only for observed cells |
| Support | collapse/support debug | DevTruth/debug-only by default |
| Perception | known/suspected/misread markers | Colony-safe |

## UI Anti-Card Compliance Test

A build fails if:

```text
central visual element is a list of cards/logs/buttons
main interaction is a menu rather than the chunk grid
material changes are not visible on the grid
```

A build passes only if:

```text
128x128 material grid/canvas is visible
commands visibly alter cells
overlays change what is drawn
inspection is tied to cells/regions
```

## Dependency Policy

Allowed after approval:

```text
Rust
serde
deterministic RNG if needed
Tauri + canvas as default visualizer
minifb or pixels as fallback
```

Banned for Chunk 0:

```text
Unreal integration
GPU-owned simulation
custom GPU compute
Bevy
procedural generation
A* pathfinding
multiplayer
card-dashboard gameplay
```

Note: `pixels` may use `wgpu` internally for rendering pixels to the screen. The ban is on custom GPU compute and GPU-owned simulation, not on transitive rendering internals.
