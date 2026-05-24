use crate::ant::{AntGroup, Task, HOME_COORD};
use crate::cell::{Flags, Material, Residue};
use crate::chunk::Chunk;
use crate::events::{Event, PerceptionEventLedger};
use crate::materials::{accepts_moisture, is_traversable};
use crate::orders::{CellDelta, Command, CommandReceipt, DebugStats};
use crate::perception::translate_event_to_colony;
use sha2::{Digest, Sha256};

/// Per-command transient flags. Reset at the start of every command and
/// intentionally excluded from canonical state hashing.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CommandFlags {
    pub sourback_entered: bool,
    pub slowed_emitted: bool,
    pub move_blocked_emitted: bool,
}

pub struct Simulation {
    pub chunk: Chunk,
    pub ant_group: AntGroup,
    pub tick_index: u32,
    pub next_command_id: u64,
    pub event_ledger: PerceptionEventLedger,
    pub sourback_earned: bool,
    pub cmd_flags: CommandFlags,
    /// Cumulative food deposited at home. Part of WorldTruth; included in canonical hash.
    pub food_returned: u32,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::default(),
            ant_group: AntGroup::default(),
            tick_index: 0,
            next_command_id: 1,
            event_ledger: PerceptionEventLedger::new(),
            sourback_earned: false,
            cmd_flags: CommandFlags::default(),
            food_returned: 0,
        }
    }

    /// Computes a deterministic canonical hash of the WorldTruth state.
    pub fn compute_chunk_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let mut buf = Vec::with_capacity(16384 * 7 + 100);

        // 1. Serialize cells
        for cell in &self.chunk.cells {
            cell.write_canonical_bytes(&mut buf);
        }

        // 2. Serialize ant group state
        self.ant_group.write_canonical_bytes(&mut buf);

        // 3. Serialize tick index
        buf.extend_from_slice(&self.tick_index.to_be_bytes());

        // 4. Serialize food_returned (WorldTruth colony state)
        buf.extend_from_slice(&self.food_returned.to_be_bytes());

        hasher.update(&buf);
        let result = hasher.finalize();

        // Convert digest output to lowercase hex String manually without external dependency
        result.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Executes a command against the simulation state, advancing ticks and generating a receipt.
    pub fn execute_command(&mut self, command: Command) -> CommandReceipt {
        let command_id = self.next_command_id;
        self.next_command_id += 1;

        let tick_start = self.tick_index;
        let hash_before = self.compute_chunk_hash();

        self.event_ledger.clear();
        // Reset per-command transient flags at the start of every command.
        self.cmd_flags = CommandFlags::default();
        let mut deltas = Vec::new();

        if let Command::Reset { .. } = command {
            self.chunk = Chunk::default();
            self.ant_group = AntGroup::default();
            self.tick_index = 0;
            self.event_ledger.clear();
            self.sourback_earned = false;
            self.cmd_flags = CommandFlags::default();
            self.food_returned = 0;

            let hash_after = self.compute_chunk_hash();
            return CommandReceipt {
                command_id,
                tick_start: 0,
                tick_end: 0,
                chunk_hash_before: hash_before,
                chunk_hash_after: hash_after,
                chunk_deltas: Vec::new(),
                perception_updates: vec![
                    "Colony environment reset to initial hardcoded state.".to_string()
                ],
                dev_event_summary: "Reset executed successfully.".to_string(),
                debug_stats: DebugStats { ticks_advanced: 0 },
            };
        }

        let mut ticks_to_run = 1;
        let mut execution_failed = false;

        match command {
            Command::StepSimulation { ticks } => {
                ticks_to_run = ticks;
            }
            Command::DigTunnel { target } => {
                let ant_pos = (self.ant_group.pos.0 as i32, self.ant_group.pos.1 as i32);
                let target_pos = (target.0 as i32, target.1 as i32);
                let dx = (target_pos.0 - ant_pos.0).abs();
                let dy = (target_pos.1 - ant_pos.1).abs();
                let is_cardinal_adjacent = (dx == 1 && dy == 0) || (dx == 0 && dy == 1);

                if !Chunk::in_bounds(target.0 as usize, target.1 as usize) {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "DigTunnel".to_string(),
                        reason: "OutOfBounds".to_string(),
                    });
                    execution_failed = true;
                } else if !is_cardinal_adjacent {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "DigTunnel".to_string(),
                        reason: "NotAdjacent".to_string(),
                    });
                    execution_failed = true;
                } else {
                    let cell = self.chunk.get(target.0 as usize, target.1 as usize);
                    if cell.material == Material::Soil || cell.material == Material::LooseSoil {
                        self.ant_group.task = Task::Dig { target };
                    } else {
                        self.event_ledger.push(Event::CommandFailed {
                            command: "DigTunnel".to_string(),
                            reason: "NotDiggable".to_string(),
                        });
                        execution_failed = true;
                    }
                }
            }
            Command::SendForagers { target } => {
                if self.ant_group.workers == 0 {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "SendForagers".to_string(),
                        reason: "NoWorkers".to_string(),
                    });
                    execution_failed = true;
                } else {
                    self.ant_group.task = Task::Forage { target };
                }
            }
            Command::ScoutResidue { target } => {
                if self.ant_group.workers == 0 {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "ScoutResidue".to_string(),
                        reason: "NoWorkers".to_string(),
                    });
                    execution_failed = true;
                } else {
                    self.ant_group.task = Task::Scout { target };
                }
            }
            Command::ReturnHome => {
                if self.ant_group.workers == 0 {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "ReturnHome".to_string(),
                        reason: "NoWorkers".to_string(),
                    });
                    execution_failed = true;
                } else {
                    self.ant_group.task = Task::ReturnHome;
                }
            }
            Command::Avoid { target } => {
                self.ant_group.task = Task::Avoid { target };
            }
            Command::InspectCell { x, y } => {
                ticks_to_run = 0;
                if !Chunk::in_bounds(x as usize, y as usize) {
                    self.event_ledger.push(Event::CommandFailed {
                        command: "InspectCell".to_string(),
                        reason: "OutOfBounds".to_string(),
                    });
                    execution_failed = true;
                }
            }
            _ => {}
        }

        if !execution_failed && ticks_to_run > 0 {
            for _ in 0..ticks_to_run {
                self.tick_index += 1;

                // 10-step tick execution slice:
                // Step 1: Apply command (already verified/set above)
                // Step 2: Execute ant group movement and task
                self.execute_movement_step();

                // Step 3: Execute digging / harvesting effects
                if let Task::Dig { target } = self.ant_group.task {
                    let cell = self.chunk.get_mut(target.0 as usize, target.1 as usize);
                    let old_material = cell.material;
                    if old_material == Material::Soil || old_material == Material::LooseSoil {
                        cell.material = Material::Tunnel;
                        cell.support = 0;
                        cell.flags |= Flags::RECENTLY_DUG;

                        deltas.push(CellDelta {
                            x: target.0,
                            y: target.1,
                            from: old_material,
                            to: Material::Tunnel,
                        });

                        let neighbors = [
                            (target.0 as i32, target.1 as i32 - 1),
                            (target.0 as i32 + 1, target.1 as i32),
                            (target.0 as i32, target.1 as i32 + 1),
                            (target.0 as i32 - 1, target.1 as i32),
                        ];
                        for &(nx, ny) in &neighbors {
                            if Chunk::in_bounds(nx as usize, ny as usize) {
                                let neighbor = self.chunk.get_mut(nx as usize, ny as usize);
                                neighbor.support = neighbor.support.saturating_sub(50);
                            }
                        }

                        self.event_ledger.push(Event::CellDug {
                            x: target.0,
                            y: target.1,
                            from: old_material,
                            to: Material::Tunnel,
                        });
                    }
                    self.ant_group.task = Task::Idle;
                }

                // Step 3 (harvest): if Forage and adjacent to Carcass, harvest one cell
                if matches!(self.ant_group.task, Task::Forage { .. }) {
                    self.execute_harvest_step(&mut deltas);
                }

                // Step 3 (return deposit): if ReturnHome and at HOME_COORD, deposit food
                if self.ant_group.task == Task::ReturnHome {
                    let (hx, hy) = HOME_COORD;
                    if self.ant_group.pos == (hx, hy) && self.ant_group.food_carried > 0 {
                        let amount = self.ant_group.food_carried;
                        self.food_returned += amount;
                        self.ant_group.food_carried = 0;
                        self.event_ledger.push(Event::FoodDeposited { amount });
                    }
                }

                // Step 4: Gravity / Collapse (bottom-up deterministic scan)
                self.execute_collapse_step(&mut deltas);

                // Step 5: Water flow (deterministic priority: down, down-left, down-right, left, right)
                self.execute_water_flow_step(&mut deltas);

                // Step 6: Moisture diffusion (double-buffer, order-independent)
                self.execute_moisture_step();

                // Step 7: Scent decay + moisture/water scent effects
                self.execute_scent_decay_step();

                // Step 8: Scent reinforcement from ant path / harvest
                self.execute_scent_reinforcement_step();

                // Step 9: Perception events generated at receipt boundary (step 10)
            }
        }

        let tick_end = self.tick_index;
        let hash_after = self.compute_chunk_hash();

        let mut perception_updates = Vec::new();
        let mut summary_parts = Vec::new();

        for event in &self.event_ledger.events {
            let perception_str = translate_event_to_colony(event, self.sourback_earned);
            perception_updates.push(perception_str.clone());
            summary_parts.push(perception_str);
        }

        let dev_event_summary = if summary_parts.is_empty() {
            "Simulation progressed with no major events.".to_string()
        } else {
            summary_parts.join("; ")
        };

        CommandReceipt {
            command_id,
            tick_start,
            tick_end,
            chunk_hash_before: hash_before,
            chunk_hash_after: hash_after,
            chunk_deltas: deltas,
            perception_updates,
            dev_event_summary,
            debug_stats: DebugStats {
                ticks_advanced: tick_end - tick_start,
            },
        }
    }

    /// Step 2 of the deterministic tick order: execute AntGroup movement and task.
    ///
    /// For Scout/Forage/ReturnHome tasks, attempt one cardinal greedy step toward
    /// the task's target. Enforces:
    /// - SourbackBitter slowdown (Scout/Forage on residue may move only on even tick_index)
    /// - Worker loss + confidence penalty on Forage entering SourbackBitter (once per command)
    /// - Cardinal-only movement (no diagonals)
    /// - Movement only into traversable cells (Air/Tunnel/Water)
    /// - Fixed neighbor order (up, right, down, left) for tie-breaking
    fn execute_movement_step(&mut self) {
        let target = match self.ant_group.task {
            Task::Scout { target } | Task::Forage { target } => Some(target),
            Task::ReturnHome => Some(HOME_COORD),
            _ => None,
        };
        let Some(target) = target else {
            return;
        };

        let is_scout_or_forage = matches!(
            self.ant_group.task,
            Task::Scout { .. } | Task::Forage { .. }
        );
        let is_forage = matches!(self.ant_group.task, Task::Forage { .. });

        let (cx, cy) = (self.ant_group.pos.0, self.ant_group.pos.1);

        // SourbackBitter slowdown: if standing on bitter residue with Scout/Forage,
        // movement is permitted only on even tick_index. Emit AntGroupSlowed once
        // per command on the first skipped (odd) tick.
        let on_sourback =
            self.chunk.get(cx as usize, cy as usize).residue == Residue::SourbackBitter;
        if on_sourback && is_scout_or_forage && (self.tick_index % 2 == 1) {
            if !self.cmd_flags.slowed_emitted {
                self.event_ledger.push(Event::AntGroupSlowed {
                    x: cx,
                    y: cy,
                    reason: "SourbackBitter residue".to_string(),
                });
                self.cmd_flags.slowed_emitted = true;
            }
            return;
        }

        // Already at target: no movement.
        if (cx, cy) == target {
            return;
        }

        // Greedy cardinal step: priority is x-reduce, then y-reduce,
        // then fallback in fixed order up, right, down, left.
        let (tx, ty) = (target.0 as i32, target.1 as i32);
        let (cxi, cyi) = (cx as i32, cy as i32);

        // Direction vectors in fixed canonical order: up, right, down, left.
        const UP: (i32, i32) = (0, -1);
        const RIGHT: (i32, i32) = (1, 0);
        const DOWN: (i32, i32) = (0, 1);
        const LEFT: (i32, i32) = (-1, 0);

        let x_pref: Option<(i32, i32)> = match cxi.cmp(&tx) {
            std::cmp::Ordering::Less => Some(RIGHT),
            std::cmp::Ordering::Greater => Some(LEFT),
            std::cmp::Ordering::Equal => None,
        };
        let y_pref: Option<(i32, i32)> = match cyi.cmp(&ty) {
            std::cmp::Ordering::Less => Some(DOWN),
            std::cmp::Ordering::Greater => Some(UP),
            std::cmp::Ordering::Equal => None,
        };

        let fixed_order = [UP, RIGHT, DOWN, LEFT];
        let mut ordered: Vec<(i32, i32)> = Vec::with_capacity(4);
        if let Some(d) = x_pref {
            ordered.push(d);
        }
        if let Some(d) = y_pref {
            if !ordered.contains(&d) {
                ordered.push(d);
            }
        }
        for d in fixed_order.iter() {
            if !ordered.contains(d) {
                ordered.push(*d);
            }
        }

        let mut moved = false;
        for (dx, dy) in ordered.into_iter() {
            let nx = cxi + dx;
            let ny = cyi + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nxu, nyu) = (nx as usize, ny as usize);
            if !Chunk::in_bounds(nxu, nyu) {
                continue;
            }
            let dest_material = self.chunk.get(nxu, nyu).material;
            if !is_traversable(dest_material) {
                continue;
            }

            // Commit the move.
            self.ant_group.pos = (nx as u8, ny as u8);
            moved = true;

            // Worker loss: triggers only on a true non-SourbackBitter → SourbackBitter
            // transition. Moving from one SourbackBitter cell to another does NOT count
            // as entry. `on_sourback` captures the cell state before this move.
            let dest_on_sourback = self.chunk.get(nxu, nyu).residue == Residue::SourbackBitter;
            let entered_sourback = !on_sourback && dest_on_sourback;
            if is_forage && entered_sourback && !self.cmd_flags.sourback_entered {
                let lost = self.ant_group.workers.min(3);
                self.ant_group.workers -= lost;
                self.ant_group.confidence = self.ant_group.confidence.saturating_sub(16);
                self.event_ledger.push(Event::WorkerLoss {
                    lost,
                    x: nx as u8,
                    y: ny as u8,
                    reason: "SourbackBitter residue".to_string(),
                });
                self.cmd_flags.sourback_entered = true;
            }
            break;
        }

        if !moved && !self.cmd_flags.move_blocked_emitted {
            self.event_ledger.push(Event::CommandFailed {
                command: "Movement".to_string(),
                reason: "Blocked".to_string(),
            });
            self.cmd_flags.move_blocked_emitted = true;
        }
    }

    // -----------------------------------------------------------------------
    // Step 4: Gravity / Collapse
    // Scan y from 126 down to 0 (bottom-up), x 0 to 127.
    // LooseSoil with support < 100 and Air/Tunnel below swaps down.
    // -----------------------------------------------------------------------
    fn execute_collapse_step(&mut self, deltas: &mut Vec<CellDelta>) {
        // Collect collapse moves first to avoid re-evaluating moved cells.
        let mut moves: Vec<(usize, usize)> = Vec::new();
        for y in (0..126usize).rev() {
            for x in 0..128usize {
                let cell = self.chunk.get(x, y);
                if cell.material != Material::LooseSoil || cell.support >= 100 {
                    continue;
                }
                let below = self.chunk.get(x, y + 1);
                if below.material == Material::Air || below.material == Material::Tunnel {
                    moves.push((x, y));
                }
            }
        }

        for (x, y) in moves {
            // Re-check: cell may have been the destination of an earlier swap this tick.
            let cell_mat = self.chunk.get(x, y).material;
            let cell_sup = self.chunk.get(x, y).support;
            let below_mat = self.chunk.get(x, y + 1).material;
            if cell_mat != Material::LooseSoil
                || cell_sup >= 100
                || (below_mat != Material::Air && below_mat != Material::Tunnel)
            {
                continue;
            }

            // Swap the two cells entirely (carries moisture/scent/residue/support/flags).
            let src_idx = y * 128 + x;
            let dst_idx = (y + 1) * 128 + x;
            self.chunk.cells.swap(src_idx, dst_idx);

            // Mark the moved (now lower) cell as recently collapsed.
            self.chunk.cells[dst_idx].flags |= Flags::RECENTLY_COLLAPSED;

            deltas.push(CellDelta {
                x: x as u8,
                y: (y + 1) as u8,
                from: Material::Air,
                to: Material::LooseSoil,
            });

            self.event_ledger.push(Event::CollapseOccurred {
                x: x as u8,
                y: (y + 1) as u8,
            });

            // AntGroup impact: if the ant is at the destination cell, apply worker-loss.
            let ant_x = self.ant_group.pos.0 as usize;
            let ant_y = self.ant_group.pos.1 as usize;
            if ant_x == x && ant_y == y + 1 {
                let lost = self.ant_group.workers.min(3);
                self.ant_group.workers -= lost;
                self.ant_group.confidence = self.ant_group.confidence.saturating_sub(16);
                self.event_ledger.push(Event::WorkerLoss {
                    lost,
                    x: x as u8,
                    y: (y + 1) as u8,
                    reason: "collapse impact".to_string(),
                });
            }
        }
    }

    // -----------------------------------------------------------------------
    // Step 5: Water flow
    // Bottom-up scan; priority: down, down-left, down-right, left, right.
    // Water swaps with Air or Tunnel only.
    // -----------------------------------------------------------------------
    fn execute_water_flow_step(&mut self, deltas: &mut Vec<CellDelta>) {
        // Collect water cell positions bottom-up.
        let mut water_cells: Vec<(usize, usize)> = Vec::new();
        for y in (0..127usize).rev() {
            for x in 0..128usize {
                if self.chunk.get(x, y).material == Material::Water {
                    water_cells.push((x, y));
                }
            }
        }

        // Priority offsets: down, down-left, down-right, left, right.
        const PRIORITY: [(i32, i32); 5] = [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)];

        for (x, y) in water_cells {
            // Re-check: this cell may have been displaced by an earlier water move.
            if self.chunk.get(x, y).material != Material::Water {
                continue;
            }
            for (dx, dy) in PRIORITY {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || nx >= 128 || ny >= 128 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                let dest = self.chunk.get(nx, ny).material;
                if dest == Material::Air || dest == Material::Tunnel {
                    let si = y * 128 + x;
                    let di = ny * 128 + nx;
                    self.chunk.cells.swap(si, di);
                    deltas.push(CellDelta {
                        x: nx as u8,
                        y: ny as u8,
                        from: dest,
                        to: Material::Water,
                    });
                    break;
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Step 6: Moisture diffusion (double-buffer)
    // Source buffer = snapshot of moisture at step start.
    // Destination buffer initialized to source; all reads from source only.
    // Transfer 4 if source[cell] > source[neighbor] + 16, neighbor accepts moisture.
    // After full pass, copy dest back to cell moisture fields.
    // Wet LooseSoil support decay: if moisture > 120, support -= 1.
    // -----------------------------------------------------------------------
    fn execute_moisture_step(&mut self) {
        let n = self.chunk.cells.len();
        // Source snapshot.
        let source: Vec<u8> = self.chunk.cells.iter().map(|c| c.moisture).collect();
        // Destination starts as a copy of source; only accumulates deltas.
        let mut dest: Vec<u8> = source.clone();

        // Process directed neighbors right and down only (each unordered pair once).
        // This avoids double-applying transfers while remaining deterministic.
        for y in 0..128usize {
            for x in 0..128usize {
                let idx = y * 128 + x;
                // Right neighbor
                if x + 1 < 128 {
                    let nidx = y * 128 + (x + 1);
                    Self::diffuse_pair(&source, &mut dest, idx, nidx, &self.chunk.cells);
                }
                // Down neighbor
                if y + 1 < 128 {
                    let nidx = (y + 1) * 128 + x;
                    Self::diffuse_pair(&source, &mut dest, idx, nidx, &self.chunk.cells);
                }
            }
        }

        // Copy dest back and apply wet LooseSoil decay.
        for i in 0..n {
            self.chunk.cells[i].moisture = dest[i];
            if self.chunk.cells[i].material == Material::LooseSoil
                && self.chunk.cells[i].moisture > 120
            {
                self.chunk.cells[i].support = self.chunk.cells[i].support.saturating_sub(1);
            }
        }
    }

    fn diffuse_pair(
        source: &[u8],
        dest: &mut Vec<u8>,
        a: usize,
        b: usize,
        cells: &[crate::cell::Cell],
    ) {
        let sa = source[a];
        let sb = source[b];
        if sa > sb.saturating_add(16) && accepts_moisture(cells[b].material) {
            let transfer = 4u8;
            dest[a] = dest[a].saturating_sub(transfer);
            dest[b] = dest[b].saturating_add(transfer);
        } else if sb > sa.saturating_add(16) && accepts_moisture(cells[a].material) {
            let transfer = 4u8;
            dest[b] = dest[b].saturating_sub(transfer);
            dest[a] = dest[a].saturating_add(transfer);
        }
    }

    // -----------------------------------------------------------------------
    // Step 7: Scent decay + moisture/water effects
    // -----------------------------------------------------------------------
    fn execute_scent_decay_step(&mut self) {
        for cell in &mut self.chunk.cells {
            if cell.material == Material::Water {
                cell.scent_home = 0;
                cell.scent_food = 0;
            } else {
                cell.scent_home = cell.scent_home.saturating_sub(1);
                cell.scent_food = cell.scent_food.saturating_sub(1);
                if cell.moisture > 100 {
                    cell.scent_home = cell.scent_home.saturating_sub(2);
                    cell.scent_food = cell.scent_food.saturating_sub(2);
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Step 8: Scent reinforcement from ant movement and harvest
    // -----------------------------------------------------------------------
    fn execute_scent_reinforcement_step(&mut self) {
        let (ax, ay) = (self.ant_group.pos.0 as usize, self.ant_group.pos.1 as usize);
        match self.ant_group.task {
            Task::ReturnHome => {
                let cell = self.chunk.get_mut(ax, ay);
                cell.scent_home = cell.scent_home.saturating_add(12).min(255);
            }
            Task::Forage { .. } => {
                let near_food = self.ant_group.food_carried > 0 || {
                    // adjacent to Carcass in up/right/down/left order
                    let offsets: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
                    offsets.iter().any(|&(dx, dy)| {
                        let nx = ax as i32 + dx;
                        let ny = ay as i32 + dy;
                        if nx >= 0 && ny >= 0 && nx < 128 && ny < 128 {
                            self.chunk.get(nx as usize, ny as usize).material == Material::Carcass
                        } else {
                            false
                        }
                    })
                };
                if near_food {
                    let cell = self.chunk.get_mut(ax, ay);
                    cell.scent_food = cell.scent_food.saturating_add(12).min(255);
                }
            }
            _ => {}
        }
    }

    // -----------------------------------------------------------------------
    // Step 3 (harvest): adjacent Carcass → Air, food_carried += 1.
    // Fixed selection order: up, right, down, left.
    // -----------------------------------------------------------------------
    fn execute_harvest_step(&mut self, deltas: &mut Vec<CellDelta>) {
        let (ax, ay) = (self.ant_group.pos.0 as usize, self.ant_group.pos.1 as usize);
        let offsets: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for &(dx, dy) in &offsets {
            let nx = ax as i32 + dx;
            let ny = ay as i32 + dy;
            if nx < 0 || ny < 0 || nx >= 128 || ny >= 128 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if self.chunk.get(nx, ny).material == Material::Carcass {
                // Convert Carcass to Air.
                self.chunk.get_mut(nx, ny).material = Material::Air;
                self.chunk.get_mut(nx, ny).support = 0;
                self.chunk.get_mut(nx, ny).flags |= Flags::HARVESTED;
                self.ant_group.food_carried += 1;

                deltas.push(CellDelta {
                    x: nx as u8,
                    y: ny as u8,
                    from: Material::Carcass,
                    to: Material::Air,
                });

                self.event_ledger.push(Event::CarcassHarvested {
                    x: nx as u8,
                    y: ny as u8,
                    food_units: 1,
                });

                // Harvest scent target: current cell if traversable, else first adjacent traversable.
                let current_traversable = is_traversable(self.chunk.get(ax, ay).material);
                if current_traversable {
                    let c = self.chunk.get_mut(ax, ay);
                    c.scent_food = c.scent_food.saturating_add(24).min(255);
                } else {
                    for &(sdx, sdy) in &offsets {
                        let sx = ax as i32 + sdx;
                        let sy = ay as i32 + sdy;
                        if sx < 0 || sy < 0 || sx >= 128 || sy >= 128 {
                            continue;
                        }
                        let (sx, sy) = (sx as usize, sy as usize);
                        if is_traversable(self.chunk.get(sx, sy).material) {
                            let c = self.chunk.get_mut(sx, sy);
                            c.scent_food = c.scent_food.saturating_add(24).min(255);
                            break;
                        }
                    }
                }
                break; // harvest one carcass only
            }
        }
    }
}
