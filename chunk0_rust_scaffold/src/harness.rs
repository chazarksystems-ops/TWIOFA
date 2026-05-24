use crate::ant::Task;
use crate::cell::{Material, Residue};
use crate::orders::Command;
use crate::orders::DebugStats;
use crate::render_frame::RenderFrame;
use crate::sim::Simulation;

// ---------------------------------------------------------------------------
// Invariant checking
// ---------------------------------------------------------------------------

pub struct InvariantResult {
    pub passed: bool,
    pub failures: Vec<String>,
}

pub fn check_invariants(sim: &Simulation) -> InvariantResult {
    let mut failures = Vec::new();

    // Cell count
    if sim.chunk.cells.len() != 16384 {
        failures.push(format!("cell count {} != 16384", sim.chunk.cells.len()));
    }

    // Boundary ring is Stone with support 255
    for x in 0..128usize {
        let top = sim.chunk.get(x, 0);
        if top.material != Material::Stone || top.support != 255 {
            failures.push(format!("boundary ({},{}) not Stone/255", x, 0));
        }
        let bot = sim.chunk.get(x, 127);
        if bot.material != Material::Stone || bot.support != 255 {
            failures.push(format!("boundary ({},{}) not Stone/255", x, 127));
        }
    }
    for y in 0..128usize {
        let left = sim.chunk.get(0, y);
        if left.material != Material::Stone || left.support != 255 {
            failures.push(format!("boundary ({},{}) not Stone/255", 0, y));
        }
        let right = sim.chunk.get(127, y);
        if right.material != Material::Stone || right.support != 255 {
            failures.push(format!("boundary ({},{}) not Stone/255", 127, y));
        }
    }

    // Workers never underflow (u32 can't be negative, but check no wrap)
    // confidence in [0,255] (u8 always true, but verify saturating sub worked)
    // These are type-guaranteed by u8/u32, so we just record current values.

    // Default RenderFrame must not expose raw residue enum
    let hash = sim.compute_chunk_hash();
    let frame = RenderFrame::generate(
        sim.tick_index,
        hash,
        &sim.chunk,
        "Material",
        Vec::new(),
        Vec::new(),
        DebugStats { ticks_advanced: 0 },
        false, // sourback_earned = false
        false, // dev_mode = false
    );
    for vc in &frame.visible_cells {
        if let Some(marker) = &vc.known_perception_marker {
            if marker.contains("SourbackBitter") || marker.contains("Sourback") {
                if !marker.contains("Sourback-associated") {
                    failures.push(format!(
                        "default RenderFrame leaks hidden truth at ({},{}): {}",
                        vc.coord.0, vc.coord.1, marker
                    ));
                }
            }
        }
        // dev fields must be None in colony mode
        if vc.dev_residue.is_some() {
            failures.push(format!(
                "dev_residue exposed in default mode at ({},{})",
                vc.coord.0, vc.coord.1
            ));
        }
    }

    InvariantResult {
        passed: failures.is_empty(),
        failures,
    }
}

// ---------------------------------------------------------------------------
// Corpus script definitions
// ---------------------------------------------------------------------------

pub struct CorpusResult {
    pub script_name: String,
    pub command_count: usize,
    pub ticks_run: u32,
    pub hash_before: String,
    pub hash_after: String,
    pub hash_sequence: Vec<String>,
    pub final_x: u8,
    pub final_y: u8,
    pub final_workers: u32,
    pub final_confidence: u8,
    pub event_counts: usize,
    pub chunk_delta_count: usize,
    pub invariants_passed: bool,
    pub failure_reason: Option<String>,
}

impl CorpusResult {
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.script_name,
            self.command_count,
            self.ticks_run,
            self.hash_before,
            self.hash_after,
            self.hash_sequence.join("|"),
            self.event_counts,
            self.chunk_delta_count,
            self.final_x,
            self.final_y,
            self.final_workers,
            self.final_confidence,
            self.invariants_passed,
            self.failure_reason.as_deref().unwrap_or(""),
        )
    }
}

/// Run a corpus script.
/// `setup` is called after an initial Reset so custom state persists.
/// `commands` are the actual test commands (do not include Reset here).
fn run_script(name: &str, commands: Vec<Command>, setup: impl Fn(&mut Simulation)) -> CorpusResult {
    let mut sim = Simulation::new();
    // Always start from a clean reset via the public API.
    sim.execute_command(Command::Reset { seed: None });
    // Apply custom fixture state after reset.
    setup(&mut sim);

    let hash_before = sim.compute_chunk_hash();
    let mut hash_sequence = vec![hash_before.clone()];
    let mut total_ticks = 0u32;
    let mut total_events = 0usize;
    let mut total_deltas = 0usize;
    let mut command_count = 0usize;

    for cmd in &commands {
        let receipt = sim.execute_command(*cmd);
        total_ticks += receipt.debug_stats.ticks_advanced;
        total_events += receipt.perception_updates.len();
        total_deltas += receipt.chunk_deltas.len();
        hash_sequence.push(receipt.chunk_hash_after.clone());
        command_count += 1;
    }

    let hash_after = sim.compute_chunk_hash();
    let inv = check_invariants(&sim);

    CorpusResult {
        script_name: name.to_string(),
        command_count,
        ticks_run: total_ticks,
        hash_before,
        hash_after,
        hash_sequence,
        final_x: sim.ant_group.pos.0,
        final_y: sim.ant_group.pos.1,
        final_workers: sim.ant_group.workers,
        final_confidence: sim.ant_group.confidence,
        event_counts: total_events,
        chunk_delta_count: total_deltas,
        invariants_passed: inv.passed,
        failure_reason: if inv.passed {
            None
        } else {
            Some(inv.failures.join("; "))
        },
    }
}

pub fn run_corpus() -> Vec<CorpusResult> {
    let mut results = Vec::new();

    // RESET_ONLY: verify stable initial hash and cell count
    results.push(run_script("RESET_ONLY", vec![], |_| {}));

    // BASIC_DIG: ant at tunnel edge (45,100), dig adjacent Soil at (44,100)
    results.push(run_script(
        "BASIC_DIG",
        vec![Command::DigTunnel { target: (44, 100) }],
        |sim| {
            sim.ant_group.pos = (45, 100);
        },
    ));

    // BOUNDARY_DIG_BLOCKED: ant adjacent to boundary Stone at (0,5)
    results.push(run_script(
        "BOUNDARY_DIG_BLOCKED",
        vec![Command::DigTunnel { target: (0, 5) }],
        |sim| {
            sim.ant_group.pos = (1, 5);
        },
    ));

    // MOVE_CARDINAL: scout along air row — no diagonal
    results.push(run_script(
        "MOVE_CARDINAL",
        vec![
            Command::ScoutResidue { target: (10, 5) },
            Command::StepSimulation { ticks: 3 },
        ],
        |sim| {
            sim.ant_group.pos = (5, 5);
        },
    ));

    // SOURBACK_ENTRY: forage from outside residue band into it — one WorkerLoss
    results.push(run_script(
        "SOURBACK_ENTRY",
        vec![Command::SendForagers { target: (84, 33) }],
        |sim| {
            sim.ant_group.pos = (84, 27);
            sim.ant_group.workers = 10;
            sim.ant_group.confidence = 200;
        },
    ));

    // SOURBACK_SLOWDOWN: start on residue, odd tick → AntGroupSlowed, no movement
    results.push(run_script(
        "SOURBACK_SLOWDOWN",
        vec![Command::ScoutResidue { target: (95, 30) }],
        |sim| {
            sim.ant_group.pos = (85, 30);
        },
    ));

    // SOURBACK_REENTRY: exit residue then re-enter — one WorkerLoss on re-entry
    results.push(run_script(
        "SOURBACK_REENTRY",
        vec![Command::StepSimulation { ticks: 6 }],
        |sim| {
            sim.chunk.get_mut(10, 10).residue = Residue::SourbackBitter;
            sim.chunk.get_mut(10, 12).residue = Residue::SourbackBitter;
            sim.ant_group.pos = (10, 10);
            sim.ant_group.workers = 100;
            sim.ant_group.task = Task::Forage { target: (10, 15) };
        },
    ));

    // BLOCKED_MOVEMENT: sealed pocket — one Blocked event per command
    results.push(run_script(
        "BLOCKED_MOVEMENT",
        vec![Command::StepSimulation { ticks: 3 }],
        |sim| {
            sim.chunk.get_mut(10, 10).material = Material::Tunnel;
            sim.chunk.get_mut(10, 9).material = Material::Soil;
            sim.chunk.get_mut(10, 11).material = Material::Soil;
            sim.chunk.get_mut(9, 10).material = Material::Soil;
            sim.chunk.get_mut(11, 10).material = Material::Soil;
            sim.ant_group.pos = (10, 10);
            sim.ant_group.task = Task::Scout { target: (12, 10) };
        },
    ));

    // GREEDY_TIE: equal x/y distance — x-reducing direction wins
    results.push(run_script(
        "GREEDY_TIE",
        vec![Command::ScoutResidue { target: (7, 7) }],
        |sim| {
            sim.ant_group.pos = (5, 5);
        },
    ));

    // LOW_WORKERS: workers=2, enter residue → workers=0, lost=2
    results.push(run_script(
        "LOW_WORKERS",
        vec![Command::SendForagers { target: (84, 33) }],
        |sim| {
            sim.ant_group.pos = (84, 27);
            sim.ant_group.workers = 2;
            sim.ant_group.confidence = 200;
        },
    ));

    // LOW_CONFIDENCE: confidence=10, enter residue → saturates to 0
    results.push(run_script(
        "LOW_CONFIDENCE",
        vec![Command::SendForagers { target: (84, 33) }],
        |sim| {
            sim.ant_group.pos = (84, 27);
            sim.ant_group.workers = 10;
            sim.ant_group.confidence = 10;
        },
    ));

    // RENDERFRAME_BOUNDARY: default RenderFrame must not expose hidden truth
    results.push(run_script("RENDERFRAME_BOUNDARY", vec![], |_| {}));

    results
}

// ---------------------------------------------------------------------------
// Stress-local-fixtures mode: deterministic LCG, 5x5 local fixtures
// ---------------------------------------------------------------------------

// Simple LCG: multiplier and increment from Knuth TAOCP vol 2
fn lcg_next(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    *state
}

pub struct StressResult {
    pub seed: u64,
    pub cases: usize,
    pub passed: usize,
    pub failed: usize,
    pub summary: String,
}

pub fn stress_local_fixtures(seed: u64, cases: usize) -> StressResult {
    let mut rng = seed;
    let mut passed = 0usize;
    let mut failed = 0usize;
    let mut failure_notes = Vec::new();

    for case_idx in 0..cases {
        let variant = lcg_next(&mut rng) % 4;
        let cx = 10u8 + (lcg_next(&mut rng) % 100) as u8; // keep in interior
        let cy = 10u8 + (lcg_next(&mut rng) % 100) as u8;
        let workers = 1 + (lcg_next(&mut rng) % 20) as u32;
        let confidence = (lcg_next(&mut rng) % 256) as u8;

        let mut sim = Simulation::new();
        sim.ant_group.pos = (cx, cy);
        sim.ant_group.workers = workers;
        sim.ant_group.confidence = confidence;

        let tx = (cx as i32 + 3).min(126) as u8;
        let ty = cy;

        match variant {
            0 => {
                // Open path scout
                sim.chunk.get_mut(cx as usize, cy as usize).material = Material::Tunnel;
                sim.chunk.get_mut(tx as usize, ty as usize).material = Material::Air;
                let _ = sim.execute_command(Command::ScoutResidue { target: (tx, ty) });
            }
            1 => {
                // Residue entry forage
                let rx = (cx as i32 + 1).min(126) as u8;
                sim.chunk.get_mut(cx as usize, cy as usize).material = Material::Air;
                sim.chunk.get_mut(rx as usize, cy as usize).material = Material::Air;
                sim.chunk.get_mut(rx as usize, cy as usize).residue = Residue::SourbackBitter;
                let _ = sim.execute_command(Command::SendForagers { target: (tx, ty) });
            }
            2 => {
                // Blocked pocket
                sim.chunk.get_mut(cx as usize, cy as usize).material = Material::Tunnel;
                let offsets: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
                for (dx, dy) in offsets {
                    let nx = (cx as i32 + dx).max(1).min(126) as usize;
                    let ny = (cy as i32 + dy).max(1).min(126) as usize;
                    sim.chunk.get_mut(nx, ny).material = Material::Soil;
                }
                let _ = sim.execute_command(Command::StepSimulation { ticks: 2 });
            }
            _ => {
                // Boundary probe
                sim.ant_group.pos = (1, cy);
                let _ = sim.execute_command(Command::DigTunnel { target: (0, cy) });
            }
        }

        let inv = check_invariants(&sim);
        if inv.passed {
            passed += 1;
        } else {
            failed += 1;
            if failure_notes.len() < 5 {
                failure_notes.push(format!("case {}: {}", case_idx, inv.failures.join("; ")));
            }
        }
    }

    let summary = if failed == 0 {
        format!(
            "stress-local-fixtures seed={} cases={}: ALL PASSED ({}/{})",
            seed, cases, passed, cases
        )
    } else {
        format!(
            "stress-local-fixtures seed={} cases={}: FAILED {}/{} -- {}",
            seed,
            cases,
            failed,
            cases,
            failure_notes.join(" | ")
        )
    };

    StressResult {
        seed,
        cases,
        passed,
        failed,
        summary,
    }
}
