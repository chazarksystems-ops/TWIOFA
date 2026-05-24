# 09 — Research Support Notes

External research to support or challenge the Chunk 0 spec design. All items are support notes only—not canon.

---

## 1. Falling-Sand / Cellular Automata Implementation Patterns

### 1a. Noita GDC Talk — "Exploring the Tech and Design of Noita"
- **Source**: Petri Purho, GDC 2019
- **URL**: https://www.youtube.com/watch?v=prXuyMCgbTc
- **Key Takeaway**: Noita uses a "Falling Everything" cellular automata engine. Each material has simple local rules (sand checks below then diagonals; liquids check horizontally too). World divided into 64×64 chunks with dirty-rectangle tracking. Multi-threading uses checkerboard pattern. Rigid bodies integrated via Box2D.
- **Relevance to Chunk 0**: Directly validates the falling-sand approach. A 128×128 grid is 2× a single Noita chunk—trivially small. Bottom-up iteration and swap-based movement are confirmed best practices.
- **Verdict**: **Supports** core simulation architecture (bottom-up iteration, swap-based movement, chunk-based processing).

### 1b. Cellular Automata Blog Series — Directional Bias Warning
- **Source**: Seppe vanden Broucke (Macuyiko), "An Exploration of Cellular Automata and Graph Based Game Systems: Part 4"
- **URL**: https://blog.macuyiko.com/post/2020/an-exploration-of-cellular-automata-and-graph-based-game-systems-part-4.html
- **Key Takeaway**: Single-buffered bottom-up approach works without double-buffering. The order of diagonal checks creates directional bias—alternating or randomizing diagonal order avoids it.
- **Relevance to Chunk 0**: Spec's water flow uses fixed priority (down, down-left, down-right, left, right). This will create a leftward bias. Acceptable for determinism, but worth noting.
- **Verdict**: **Warns** about directional bias in diagonal checks. Spec accepts this tradeoff for determinism. **Supports** in-place single-buffer updates.

### 1c. Noita Technical Breakdown (80.lv)
- **Source**: 80.lv technical article
- **URL**: https://80.lv/articles/noita-developers-discuss-the-technical-challenges-behind-the-game/
- **Key Takeaway**: Key optimizations: (1) only simulate active chunks, (2) dirty rectangles, (3) particles can escape CA grid temporarily. All unnecessary at 128×128 scale.
- **Relevance to Chunk 0**: Confirms dirty-rect optimization is correctly deferred. Whole-chunk updates are appropriate at this scale.
- **Verdict**: **Supports** deferring dirty-cell tracking. **Supports** whole-chunk updates.

---

## 2. Deterministic Simulation and Replay

### 2a. "Fix Your Timestep!" — Glenn Fiedler
- **Source**: Glenn Fiedler, Gaffer on Games
- **URL**: https://gafferongames.com/post/fix_your_timestep/
- **Key Takeaway**: For deterministic replay, use a fixed simulation timestep decoupled from rendering framerate. Accumulator pattern: renderer produces time, simulation consumes it in fixed chunks.
- **Relevance to Chunk 0**: Essential for replay/hash verification. Tick must be fixed, not tied to render frame.
- **Verdict**: **Supports** fixed-timestep architecture. Spec's tick-based simulation is correct.

### 2b. "Deterministic Lockstep" — Glenn Fiedler
- **Source**: Glenn Fiedler, Gaffer on Games
- **URL**: https://gafferongames.com/post/deterministic_lockstep/
- **Key Takeaway**: Simulation = pure function: `NewState = Simulate(OldState, Inputs)`. Avoid HashMap iteration. Use seeded PRNG. Floating-point differences across platforms are a desync source.
- **Relevance to Chunk 0**: Flat array iteration is inherently deterministic. Integer-only math avoids FP issues. Spec correctly bans HashMap iteration and wall-clock time.
- **Verdict**: **Supports** deterministic design. **Supports** canonical serialization. **Warns** about floating-point (spec correctly uses integer math).

### 2c. Canonical Serialization Best Practices
- **Source**: Aggregated game development resources
- **URL**: N/A
- **Key Takeaway**: Standard Protobuf is NOT canonical by default. Custom serialization with deterministic field order is needed. Flat array of cells with fixed struct layout serializes deterministically by nature.
- **Relevance to Chunk 0**: Spec's `Vec<Cell>` with fixed struct fields is inherently canonical. Using `serde` with deterministic format (e.g., bincode) is a good fit.
- **Verdict**: **Supports** canonical serialization approach.

---

## 3. Data-Oriented Rust Grid/Chunk Storage

### 3a. Flat Array Best Practices
- **Source**: Rust game dev community patterns
- **URL**: N/A
- **Key Takeaway**: Use flat `Vec<T>` of size `width * height`—never `Vec<Vec<T>>`. Access with `index = y * width + x`. At 16,384 cells with ≤16 bytes each, entire grid fits in ~256KB—within L2 cache.
- **Relevance to Chunk 0**: Spec's `Vec<Cell>` is optimal. 7 × u8 fields = 7 bytes per cell (plus padding). Total = ~112KB–128KB raw data, easily cache-resident.
- **Verdict**: **Supports** flat `Vec<Cell>` storage.

### 3b. AoS vs SoA
- **Source**: Rust gamedev community
- **URL**: N/A
- **Key Takeaway**: AoS (Array of Structs, i.e., `Vec<Cell>`) is simpler and works well when all fields are accessed together. SoA benefits kick in at larger scales or with SIMD.
- **Relevance to Chunk 0**: AoS is the right default for 128×128. Cell struct should implement `Copy, Clone`.
- **Verdict**: **Supports** AoS (`Vec<Cell>`) for this scale.

---

## 4. Lightweight Rust Visualizer Options

### 4a. `minifb` — Software-Rendered Pixel Buffer
- **Source**: minifb crate documentation
- **URL**: https://crates.io/crates/minifb
- **Key Takeaway**: Extremely simple: provide `Vec<u32>` buffer, call `window.update_with_buffer()`. Software-rendered. ~10 lines to get pixels on screen. Built-in key handling.
- **Relevance to Chunk 0**: **Best prototype option.** API directly maps to flat grid buffer. 128×128 is trivially fast for software rendering.
- **Verdict**: **Supports** as excellent fallback/prototype visualizer.

### 4b. `pixels` — GPU-Accelerated via wgpu
- **Source**: pixels crate documentation
- **URL**: https://crates.io/crates/pixels
- **Key Takeaway**: GPU-powered via wgpu. Provides `&mut [u8]` RGBA buffer. Requires winit for windowing (more boilerplate). Better for larger grids.
- **Relevance to Chunk 0**: Good upgrade path from minifb. Overkill for 128×128 but future-proof.
- **Verdict**: **Supports** as production visualizer. Note: uses wgpu internally for rendering—this is allowed per spec `06` L165.

### 4c. `egui` — Immediate-Mode GUI
- **Source**: egui GitHub
- **URL**: https://github.com/emilk/egui
- **Key Takeaway**: Best for buttons, sliders, debug panels alongside pixel grid. Can display grid as texture. Not optimized for raw pixel pushing.
- **Relevance to Chunk 0**: Useful for debug UI (cell inspector, parameter tweaking). Not a primary renderer.
- **Verdict**: **Supports** as complementary debug UI. Not recommended as primary renderer.

### 4d. Tauri + HTML Canvas
- **Source**: Tauri framework
- **URL**: https://tauri.app/
- **Key Takeaway**: Rust backend + web frontend. Great for polished desktop apps with web-tech UI. Significant architectural overhead vs pure Rust solutions. Communication bridge adds complexity.
- **Relevance to Chunk 0**: The research suggests Tauri may be over-engineered for this use case. Pure Rust solutions (minifb/pixels) are simpler and faster.
- **Verdict**: **Warns** against Tauri as default—adds complexity that doesn't benefit a 128×128 grid prototype. However, the spec nominates it as default. This creates a tension worth flagging.

### Visualizer Recommendation Summary

| Phase | Research Recommendation | Spec Choice | Tension? |
|---|---|---|---|
| Prototype | `minifb` | Tauri + Canvas (default) | ⚠️ Yes — research suggests minifb is simpler |
| Production | `pixels` | minifb/pixels (fallback) | No — aligned |
| Debug UI | `egui` | Tauri DOM | Mild — egui is simpler |

> **Note**: This is research opinion, not a spec correction. The spec's Tauri choice may have broader project reasons (e.g., future UI needs, web tech familiarity). Flagged as D2 decision for Chaz.

---

## 5. Ant Pheromone Trail / Foraging Basics

### 5a. Sebastian Lague — Ant & Slime Simulations
- **Source**: Sebastian Lague, YouTube
- **URL**: https://www.youtube.com/watch?v=X-iSQQgOd1A
- **Key Takeaway**: Two pheromone types (home/food). Ants sample concentration in forward-facing regions to decide turning. Pheromones evaporate over time. Random movement for exploration.
- **Relevance to Chunk 0**: Dual-pheromone model (home/food) maps directly to spec's `scent_home` and `scent_food`. Evaporation = spec's scent decay.
- **Verdict**: **Supports** dual-pheromone model and gradient following.

### 5b. Pheromone Mechanics — Tunable Parameters
- **Source**: Aggregated academic/tutorial sources
- **URL**: N/A
- **Key Takeaway**: Three critical parameters: (1) evaporation rate, (2) diffusion (spreading to neighbors), (3) reinforcement. Diffusion is commonly overlooked—without it, trails are too narrow.
- **Relevance to Chunk 0**: Spec has evaporation (decay) and reinforcement but **no scent diffusion** (spreading to neighboring cells). This could make trails too narrow for the ant group to follow reliably.
- **Verdict**: **Modifies** — spec should consider adding scent diffusion. Currently, scent only reinforces on the exact cell the ant passes through, with no lateral spread. **Flagged as potential v0.3.3 enhancement.**

### 5c. Exploration vs Exploitation
- **Source**: Aggregated ant foraging algorithm resources
- **URL**: N/A
- **Key Takeaway**: Ants need stochastic movement when no pheromone is detected for initial exploration. Balance between random exploration and gradient exploitation determines colony efficiency.
- **Relevance to Chunk 0**: Spec's greedy movement has no exploration component. The ant group always moves greedily toward target. This is acceptable for Chunk 0 (commands specify targets) but may need addressing in later chunks.
- **Verdict**: **Supports** greedy movement for Chunk 0 scope. **Modifies** — later chunks should add exploration.

---

## 6. Soil/Moisture/Collapse Simplification for Games

### 6a. Terraria Liquid System
- **Source**: Terraria Wiki
- **URL**: https://terraria.wiki.gg/wiki/Liquid
- **Key Takeaway**: Integer-based volume per tile (0–255 units). Liquids flow down first, spread horizontally. Different viscosities. Integer math causes rounding artifacts.
- **Relevance to Chunk 0**: Validates u8 moisture model. Simple integer-based flow is proven at scale.
- **Verdict**: **Supports** u8 moisture per cell. **Warns** about rounding artifacts (acceptable for Chunk 0).

### 6b. Noita-style Structural Collapse
- **Source**: GDC talk and community discussions
- **URL**: N/A
- **Key Takeaway**: Noita uses connected-component analysis for structural collapse. For simpler games, just check if support is below threshold and space below is open → treat like falling sand.
- **Relevance to Chunk 0**: Spec's collapse rule (support < 100 + open below = swap) is the simplified "treat like sand" approach. Correct for Chunk 0 scope.
- **Verdict**: **Supports** simplified collapse model. No rigid body physics needed.

---

## 7. Noita-Inspired Material Simulation Lessons

### 7a. Engine-First Development
- **Source**: Nolla Games / Petri Purho, GDC 2019
- **URL**: https://www.youtube.com/watch?v=prXuyMCgbTc
- **Key Takeaway**: "The engine was looking for a game." Emergent chaos is fun to watch but hard to design around. Balance between simulation freedom and design constraints is the hardest part.
- **Relevance to Chunk 0**: Start with simulation engine, prove substrate works before worrying about gameplay. At 128×128, iteration is fast.
- **Verdict**: **Supports** engine-first, substrate-proof approach (exactly what Chunk 0 is doing).

### 7b. Common Pitfalls
- **Source**: Noita-like project retrospectives (Reddit, dev blogs)
- **URL**: https://www.meatbatgames.com/
- **Key Takeaway**: Top pitfalls: (1) OOP Particle class per pixel — too slow; use flat arrays. (2) GPU compute too early — branching logic bad for GPU. (3) Not implementing chunking early enough. (4) Projects fail from lack of persistence.
- **Relevance to Chunk 0**: All pitfalls avoided: flat array ✓, CPU sim ✓, single chunk ✓. Persistence is a project management concern.
- **Verdict**: **Supports** all spec technical decisions.

### 7c. Double-Processing Prevention
- **Source**: Game dev community on falling sand engines
- **URL**: N/A
- **Key Takeaway**: Use a "processed this frame" flag or alternating frame counter to prevent cells from being processed twice in one tick (e.g., a cell falls, then is processed again at its new position).
- **Relevance to Chunk 0**: Spec does not address double-processing prevention. Bottom-up iteration mitigates this for falling (processed cell moves down, loop moves up), but water flow and moisture diffusion could have issues.
- **Verdict**: **Warns** — spec should specify how double-processing is prevented for water flow. **Flagged for v0.3.3.**

---

## 8. Pixel/Grid Rendering Performance for 128×128

### 8a. Performance Analysis
- **Source**: Game dev rendering resources (aggregated)
- **URL**: N/A
- **Key Takeaway**: 128×128 = 16,384 pixels. **Trivially fast.** Entire grid fits in L2 cache. Single texture upload of 64KB RGBA per frame is negligible. Rendering will never be the bottleneck.
- **Relevance to Chunk 0**: Rendering is a non-issue. Even software rendering (minifb) runs at thousands of FPS at this scale.
- **Verdict**: **Supports** simple rendering approach. No optimization needed.

### 8b. Rendering Architecture
- **Source**: GPU rendering best practices
- **URL**: N/A
- **Key Takeaway**: Optimal pattern: grid as single texture, update changed pixels, render as one quad. Nearest-neighbor filtering for pixel-art scaling.
- **Relevance to Chunk 0**: Whether minifb (software buffer) or pixels (GPU texture), the pattern is identical.
- **Verdict**: **Supports** single-buffer rendering approach.

### 8c. Simulation/Rendering Decoupling
- **Source**: "Fix Your Timestep!" (Glenn Fiedler)
- **URL**: https://gafferongames.com/post/fix_your_timestep/
- **Key Takeaway**: Decouple simulation tick rate from render frame rate. Fixed simulation rate (e.g., 30Hz) with independent render rate.
- **Relevance to Chunk 0**: Essential architecture. Simulation ticks at fixed rate; visualizer renders current state.
- **Verdict**: **Supports** fixed-tick simulation.

---

## Research Summary Table

| Research Area | Spec Aspect | Verdict | Action |
|---|---|---|---|
| Falling sand patterns | Bottom-up iteration, swap movement | ✅ Supports | None needed |
| Deterministic simulation | Fixed tick, no HashMap, canonical hash | ✅ Supports | None needed |
| Flat array storage | `Vec<Cell>` with Copy struct | ✅ Supports | None needed |
| Visualizer choice | Tauri as default | ⚠️ Warns | D2 decision — research suggests minifb/pixels simpler |
| Pheromone trails | Dual scent, decay, reinforcement | ✅ Supports | Consider adding scent diffusion in v0.3.3 |
| Soil/collapse model | Support threshold + swap | ✅ Supports | None needed |
| Noita lessons | Engine-first, flat arrays, CPU sim | ✅ Supports | None needed |
| Rendering performance | 128×128 trivially fast | ✅ Supports | No optimization needed |
| Double-processing | Not addressed in spec | ⚠️ Warns | Spec should address; flagged for v0.3.3 |
| Diagonal bias | Fixed priority = leftward bias | ℹ️ Note | Acceptable for determinism |
