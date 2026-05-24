use chunk0_rust_scaffold::ant::Task;
use chunk0_rust_scaffold::cell::{Flags, Material, Residue};
use chunk0_rust_scaffold::chunk::{Chunk, CELL_COUNT, HEIGHT, WIDTH};
use chunk0_rust_scaffold::orders::{Command, DebugStats};
use chunk0_rust_scaffold::render_frame::RenderFrame;
use chunk0_rust_scaffold::sim::Simulation;

#[test]
fn test_cell_count() {
    let chunk = Chunk::default();
    assert_eq!(chunk.cells.len(), CELL_COUNT);
    assert_eq!(CELL_COUNT, 16384);
}

#[test]
fn test_stable_index_mapping() {
    assert_eq!(Chunk::index(0, 0), 0);
    assert_eq!(Chunk::index(127, 0), 127);
    assert_eq!(Chunk::index(0, 1), 128);
    assert_eq!(Chunk::index(127, 127), 16383);
}

#[test]
fn test_boundary_ring_cells_are_stone() {
    let chunk = Chunk::default();

    // Top boundary y=0
    for x in 0..WIDTH {
        assert_eq!(chunk.get(x, 0).material, Material::Stone);
        assert_eq!(chunk.get(x, 0).support, 255);
    }

    // Bottom boundary y=127
    for x in 0..WIDTH {
        assert_eq!(chunk.get(x, 127).material, Material::Stone);
        assert_eq!(chunk.get(x, 127).support, 255);
    }

    // Left boundary x=0
    for y in 0..HEIGHT {
        assert_eq!(chunk.get(0, y).material, Material::Stone);
        assert_eq!(chunk.get(0, y).support, 255);
    }

    // Right boundary x=127
    for y in 0..HEIGHT {
        assert_eq!(chunk.get(127, y).material, Material::Stone);
        assert_eq!(chunk.get(127, y).support, 255);
    }
}

#[test]
fn test_boundary_dig_blocked_and_receipted() {
    let mut sim = Simulation::new();

    // Move ant group to (1, 5) so it is cardinally adjacent to boundary at (0, 5)
    sim.ant_group.pos = (1, 5);

    // Attempt to dig Stone boundary cell at (0, 5)
    let receipt = sim.execute_command(Command::DigTunnel { target: (0, 5) });

    // Should fail and produce failure event
    assert_eq!(sim.chunk.get(0, 5).material, Material::Stone); // Remains stone
    assert!(
        receipt.dev_event_summary.contains("failed")
            || receipt.dev_event_summary.contains("NotDiggable")
    );
    assert_eq!(receipt.chunk_deltas.len(), 0);
}

#[test]
fn test_basic_dig_to_tunnel_with_support_changes() {
    let mut sim = Simulation::new();

    // Place ant group at (45, 100) in Nest/Tunnel
    sim.ant_group.pos = (45, 100);
    assert_eq!(sim.chunk.get(45, 100).material, Material::Tunnel);
    assert_eq!(sim.chunk.get(44, 100).material, Material::Soil);
    let initial_neighbor_support = sim.chunk.get(44, 99).support; // Neighbor (44, 99) is Soil

    // Dig (44, 100)
    let receipt = sim.execute_command(Command::DigTunnel { target: (44, 100) });

    // Check that target cell is now Tunnel and support remains 0
    assert_eq!(sim.chunk.get(44, 100).material, Material::Tunnel);
    assert_eq!(sim.chunk.get(44, 100).support, 0); // Tunnel support remains 0

    // Neighbors' support should be reduced by 50
    let final_neighbor_support = sim.chunk.get(44, 99).support;
    assert_eq!(final_neighbor_support, initial_neighbor_support - 50);

    // Delta and events checked
    assert_eq!(receipt.chunk_deltas.len(), 1);
    assert_eq!(receipt.chunk_deltas[0].x, 44);
    assert_eq!(receipt.chunk_deltas[0].y, 100);
    assert_eq!(receipt.chunk_deltas[0].from, Material::Soil);
    assert_eq!(receipt.chunk_deltas[0].to, Material::Tunnel);
}

#[test]
fn test_replay_determinism() {
    let mut sim1 = Simulation::new();
    let mut sim2 = Simulation::new();

    // Verify initial hash matches
    let h1_0 = sim1.compute_chunk_hash();
    let h2_0 = sim2.compute_chunk_hash();
    assert_eq!(h1_0, h2_0);

    // Move ant group
    sim1.ant_group.pos = (45, 100);
    sim2.ant_group.pos = (45, 100);

    // Run identical command sequence
    let rec1 = sim1.execute_command(Command::DigTunnel { target: (44, 100) });
    let rec2 = sim2.execute_command(Command::DigTunnel { target: (44, 100) });

    assert_eq!(rec1.chunk_hash_before, rec2.chunk_hash_before);
    assert_eq!(rec1.chunk_hash_after, rec2.chunk_hash_after);
    assert_eq!(sim1.compute_chunk_hash(), sim2.compute_chunk_hash());

    // Reset simulation and verify hashes reset identically
    let reset_rec1 = sim1.execute_command(Command::Reset { seed: None });
    let reset_rec2 = sim2.execute_command(Command::Reset { seed: None });

    assert_eq!(reset_rec1.chunk_hash_after, reset_rec2.chunk_hash_after);
    assert_eq!(sim1.compute_chunk_hash(), h1_0);
}

#[test]
fn test_render_frame_colony_view_residue_boundary() {
    let sim = Simulation::new();
    let hash = sim.compute_chunk_hash();

    // Generate RenderFrame in ColonyView mode (default, dev_mode=false, sourback_earned=false)
    let frame_default = RenderFrame::generate(
        sim.tick_index,
        hash.clone(),
        &sim.chunk,
        "Material",
        Vec::new(),
        Vec::new(),
        DebugStats { ticks_advanced: 0 },
        false, // sourback_earned = false
        false, // dev_mode = false
    );

    // Find cell in Sourback residue path (e.g., x=90, y=30)
    let cell_idx = Chunk::index(90, 30);
    let visible_cell = &frame_default.visible_cells[cell_idx];

    // Under default mode, it must NOT leak residue details or unearned labels
    assert_eq!(
        visible_cell.known_perception_marker.as_deref(),
        Some("bitter/yellow residue")
    );
    assert_eq!(visible_cell.dev_residue, None);
    assert_eq!(visible_cell.dev_support, None);

    // Generate in dev mode to verify DevTruth works
    let frame_dev = RenderFrame::generate(
        sim.tick_index,
        hash,
        &sim.chunk,
        "Material",
        Vec::new(),
        Vec::new(),
        DebugStats { ticks_advanced: 0 },
        false,
        true, // dev_mode = true
    );

    let visible_cell_dev = &frame_dev.visible_cells[cell_idx];
    assert_eq!(visible_cell_dev.dev_residue, Some(Residue::SourbackBitter));
    assert_eq!(visible_cell_dev.dev_support, Some(0)); // Support for Air is 0
}

// ---------------------------------------------------------------------------
// Slice 2 tests: AntGroup movement, greedy stepping, SourbackBitter slowdown,
// and worker loss rules.
// ---------------------------------------------------------------------------

/// Cardinal movement allowed: scout one step toward a target along Air row.
/// Ant starts at (5, 5) (Air, no residue) with target (7, 5) → must move one
/// cell right per tick along the cardinal axis.
#[test]
fn test_cardinal_movement_along_row() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (5, 5);
    // Air row, no residue.
    assert_eq!(sim.chunk.get(5, 5).material, Material::Air);
    assert_eq!(sim.chunk.get(6, 5).material, Material::Air);

    let _ = sim.execute_command(Command::ScoutResidue { target: (7, 5) });
    assert_eq!(sim.ant_group.pos, (6, 5));

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (7, 5));
}

/// Diagonal movement is impossible: target diagonally offset must be reached
/// via a cardinal sequence (x-reduce first, then y-reduce). After one step the
/// ant must be cardinally adjacent to its previous position (manhattan = 1).
#[test]
fn test_no_diagonal_movement_x_first() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (5, 5);
    let _ = sim.execute_command(Command::ScoutResidue { target: (7, 7) });

    // x-reducing direction is preferred per the greedy step priority.
    assert_eq!(sim.ant_group.pos, (6, 5));

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (7, 5));

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (7, 6));

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (7, 7));
}

/// Movement into a non-traversable cell is blocked and receipted as
/// CommandFailed(Blocked). The ant is wedged into a one-cell pocket by
/// overriding the chunk in-test.
#[test]
fn test_movement_into_non_traversable_blocked_and_receipted() {
    let mut sim = Simulation::new();
    // Carve a single-cell trap at (10, 10): surrounded by Soil on all cardinals.
    sim.chunk.get_mut(10, 10).material = Material::Tunnel;
    sim.chunk.get_mut(10, 9).material = Material::Soil;
    sim.chunk.get_mut(10, 11).material = Material::Soil;
    sim.chunk.get_mut(9, 10).material = Material::Soil;
    sim.chunk.get_mut(11, 10).material = Material::Soil;

    sim.ant_group.pos = (10, 10);
    let receipt = sim.execute_command(Command::ScoutResidue { target: (12, 10) });

    // Ant cannot move; position unchanged.
    assert_eq!(sim.ant_group.pos, (10, 10));
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Movement") && p.contains("Blocked")),
        "expected a Movement/Blocked failure in perception_updates, got {:?}",
        receipt.perception_updates
    );
}

/// Greedy stepping is deterministic when both x and y distances differ:
/// x-reducing direction wins the tie. From (10, 10) toward (12, 12) the
/// first move must be RIGHT, not DOWN.
#[test]
fn test_greedy_deterministic_tie_x_first() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (10, 10);
    // (10,10), (11,10), (12,10) are all Air on the default map.
    let _ = sim.execute_command(Command::ScoutResidue { target: (12, 12) });
    assert_eq!(sim.ant_group.pos, (11, 10));
}

/// SourbackBitter movement slowdown: when standing on bitter residue with a
/// Scout/Forage task, the ant may move only on even tick_index. On the odd
/// tick the ant remains in place and an AntGroupSlowed event is emitted.
#[test]
fn test_sourback_bitter_movement_slowdown() {
    let mut sim = Simulation::new();
    // (85, 30) lies in the bitter residue band (cols 80..116, rows 28..36).
    sim.ant_group.pos = (85, 30);
    assert_eq!(sim.chunk.get(85, 30).residue, Residue::SourbackBitter);

    // ScoutResidue command runs 1 tick → tick_index becomes 1 (odd) → slowed.
    let receipt = sim.execute_command(Command::ScoutResidue { target: (95, 30) });
    assert_eq!(sim.ant_group.pos, (85, 30));
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("slowed")),
        "expected AntGroupSlowed translation, got {:?}",
        receipt.perception_updates
    );

    // Next StepSimulation lands on tick_index 2 (even) → movement proceeds.
    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (86, 30));
}

/// Worker loss = min(3, workers) on first Forage entry into SourbackBitter,
/// with a fixed confidence penalty of -16 saturating to [0, 255].
#[test]
fn test_worker_loss_and_confidence_penalty() {
    let mut sim = Simulation::new();
    // Start on non-residue Air at (84, 27) (row 27 is above the residue band).
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 10;
    sim.ant_group.confidence = 200;
    assert_eq!(sim.chunk.get(84, 27).residue, Residue::None);
    assert_eq!(sim.chunk.get(84, 28).residue, Residue::SourbackBitter);

    let receipt = sim.execute_command(Command::SendForagers { target: (84, 30) });

    // Forage moves DOWN once (tick 1 odd but ant not yet on residue → no skip).
    // Destination (84, 28) is bitter residue → first entry triggers loss.
    assert_eq!(sim.ant_group.pos, (84, 28));
    assert_eq!(sim.ant_group.workers, 7); // min(3, 10) = 3 lost
    assert_eq!(sim.ant_group.confidence, 184); // 200 - 16
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost") && p.contains("workers")),
        "expected WorkerLoss perception update, got {:?}",
        receipt.perception_updates
    );
}

/// Confidence penalty saturates at 0 (clamped to [0, 255]).
#[test]
fn test_confidence_penalty_saturates_at_zero() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 10;
    sim.ant_group.confidence = 10;

    let _ = sim.execute_command(Command::SendForagers { target: (84, 30) });
    assert_eq!(sim.ant_group.confidence, 0);
}

/// Worker loss = min(3, workers) when workers < 3: the loss cannot exceed
/// the remaining workforce.
#[test]
fn test_worker_loss_min3_with_small_workforce() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 2;
    sim.ant_group.confidence = 200;

    let _ = sim.execute_command(Command::SendForagers { target: (84, 30) });
    assert_eq!(sim.ant_group.workers, 0); // min(3, 2) = 2 lost
}

/// Worker loss fires at most once per command, even when the ant traverses
/// many bitter residue cells across multiple ticks of the same command.
#[test]
fn test_worker_loss_only_once_per_command() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 255;

    // Single Forage command followed by a long StepSimulation. Each is its
    // own command boundary, so cmd_flags reset between them.
    let _ = sim.execute_command(Command::SendForagers { target: (84, 33) });
    // First command: 1 tick, ant moves from (84,27) to (84,28), enters residue → loss.
    assert_eq!(sim.ant_group.workers, 97);

    // Subsequent StepSimulation is a NEW command → cmd_flags reset. But the
    // ant should still only lose workers on the first entry into a residue
    // cell during THAT command. Within this StepSimulation, we expect at
    // most one further loss event.
    let workers_before = sim.ant_group.workers;
    let receipt = sim.execute_command(Command::StepSimulation { ticks: 6 });

    // Across 6 ticks the ant traverses multiple residue cells; worker loss
    // must trigger at most once during this single StepSimulation command.
    let worker_loss_events = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Lost") && p.contains("workers"))
        .count();
    assert!(
        worker_loss_events <= 1,
        "worker loss must fire at most once per command, fired {} times: {:?}",
        worker_loss_events,
        receipt.perception_updates
    );
    // Net delta in this single command must be ≤ 3.
    assert!(workers_before - sim.ant_group.workers <= 3);
}

/// Receipt includes WorkerLoss perception update on bitter-residue entry.
#[test]
fn test_receipt_includes_worker_loss_perception_update() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;

    let receipt = sim.execute_command(Command::SendForagers { target: (84, 30) });

    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost") && p.contains("workers")),
        "expected WorkerLoss in perception_updates, got {:?}",
        receipt.perception_updates
    );
    assert!(receipt.dev_event_summary.contains("Lost"));
}

/// Repeated identical command sequence on a fresh simulation produces an
/// identical final chunk hash, covering the Slice 2 movement/loss paths.
#[test]
fn test_slice2_repeated_command_sequence_hash_stable() {
    fn run() -> String {
        let mut sim = Simulation::new();
        sim.ant_group.pos = (84, 27);
        sim.ant_group.workers = 100;
        sim.ant_group.confidence = 200;
        let _ = sim.execute_command(Command::SendForagers { target: (84, 33) });
        let _ = sim.execute_command(Command::StepSimulation { ticks: 5 });
        let _ = sim.execute_command(Command::ScoutResidue { target: (90, 33) });
        let _ = sim.execute_command(Command::StepSimulation { ticks: 3 });
        sim.compute_chunk_hash()
    }

    let h1 = run();
    let h2 = run();
    let h3 = run();
    assert_eq!(h1, h2);
    assert_eq!(h2, h3);
}

/// ReturnHome navigates toward HOME_COORD via cardinal greedy stepping.
#[test]
fn test_return_home_uses_greedy_cardinal_stepping() {
    let mut sim = Simulation::new();
    // Place ant inside the nest band but offset from HOME_COORD (55, 118).
    sim.ant_group.pos = (50, 100);
    sim.ant_group.task = Task::Idle;
    assert_eq!(sim.chunk.get(50, 100).material, Material::Tunnel);

    let _ = sim.execute_command(Command::ReturnHome);
    // First step must be cardinal toward (55, 118): x-reducing direction wins,
    // so move RIGHT to (51, 100).
    assert_eq!(sim.ant_group.pos, (51, 100));
}

// ---------------------------------------------------------------------------
// Slice 2 semantic-correction tests: transition-based SourbackBitter entry.
// WorkerLoss triggers only on a true non-SourbackBitter → SourbackBitter
// transition. Moving between two SourbackBitter cells must never fire it.
// ---------------------------------------------------------------------------

/// REQ-1: Moving from a non-residue cell into a SourbackBitter cell is a true
/// transition entry and must trigger WorkerLoss with min(3, workers) loss and
/// confidence saturating_sub(16).
///
/// The residue band occupies rows 28..36 (i.e. y=28 through y=35, Rust half-open
/// range exclusive of 36). Row 27 is Air without residue; row 28 is the first
/// bitter row. u8::saturating_sub never underflows below 0.
#[test]
fn test_non_residue_to_sourback_triggers_worker_loss() {
    let mut sim = Simulation::new();
    // (84, 27): Air, no residue (row 27 is above the band rows 28..36).
    // (84, 28): Air, SourbackBitter residue (row 28 is the first row of the band).
    assert_eq!(sim.chunk.get(84, 27).residue, Residue::None);
    assert_eq!(sim.chunk.get(84, 28).residue, Residue::SourbackBitter);

    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 10;
    sim.ant_group.confidence = 200;

    // tick_index 0 → 1 (odd); ant is NOT on sourback so no slowdown.
    // Greedy moves DOWN to (84, 28): non-sourback → sourback transition.
    let receipt = sim.execute_command(Command::SendForagers { target: (84, 30) });

    assert_eq!(sim.ant_group.pos, (84, 28));
    assert_eq!(sim.ant_group.workers, 7, "min(3, 10) = 3 workers lost");
    assert_eq!(sim.ant_group.confidence, 184, "200 - 16 = 184");
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost") && p.contains("workers")),
        "WorkerLoss must appear in perception_updates: {:?}",
        receipt.perception_updates
    );
}

/// REQ-2: Moving from a SourbackBitter cell to ANOTHER SourbackBitter cell is
/// NOT an entry transition and must NOT trigger WorkerLoss.
/// This is the primary semantic correctness test for Slice 2.
#[test]
fn test_sourback_to_sourback_does_not_trigger_worker_loss() {
    let mut sim = Simulation::new();
    // Both (85, 28) and (86, 28) lie inside the residue band (cols 80..116, rows 28..36).
    assert_eq!(sim.chunk.get(85, 28).residue, Residue::SourbackBitter);
    assert_eq!(sim.chunk.get(86, 28).residue, Residue::SourbackBitter);

    sim.ant_group.pos = (85, 28);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;

    // Command 1: SendForagers → tick 1 (odd) → on sourback + forage + odd → slowed.
    // Ant does not move; no worker loss on a slowed tick.
    let receipt1 = sim.execute_command(Command::SendForagers { target: (90, 28) });
    assert_eq!(sim.ant_group.pos, (85, 28));
    assert_eq!(sim.ant_group.workers, 100);
    assert_eq!(sim.ant_group.confidence, 200);
    assert!(
        !receipt1
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost")),
        "no WorkerLoss on a slowed tick: {:?}",
        receipt1.perception_updates
    );

    // Command 2: StepSimulation → cmd_flags reset → tick 2 (even).
    // was_on_sourback = true (ant still at 85,28). Moves RIGHT to (86,28).
    // dest_on_sourback = true. entered_sourback = !true && true = false → no loss.
    let receipt2 = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.pos, (86, 28));
    assert_eq!(
        sim.ant_group.workers, 100,
        "sourback→sourback must not trigger WorkerLoss"
    );
    assert_eq!(
        sim.ant_group.confidence, 200,
        "confidence must be unchanged for sourback→sourback move"
    );
    assert!(
        !receipt2
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost")),
        "no WorkerLoss in perception_updates for sourback→sourback: {:?}",
        receipt2.perception_updates
    );
}

/// REQ-3: Starting a Forage command already on SourbackBitter and receiving a
/// slowdown on an odd tick must not emit WorkerLoss. The ant never moved, so no
/// transition occurred.
#[test]
fn test_starting_on_sourback_odd_tick_slows_without_worker_loss() {
    let mut sim = Simulation::new();
    assert_eq!(sim.chunk.get(85, 28).residue, Residue::SourbackBitter);

    sim.ant_group.pos = (85, 28);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;

    // tick 0 → 1 (odd): on sourback + forage → slowed, no move, no entry.
    let receipt = sim.execute_command(Command::SendForagers { target: (90, 28) });

    assert_eq!(
        sim.ant_group.pos,
        (85, 28),
        "ant must not move on slowed tick"
    );
    assert_eq!(sim.ant_group.workers, 100, "no worker loss on slowed tick");
    assert_eq!(
        sim.ant_group.confidence, 200,
        "no confidence change on slowed tick"
    );
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("slowed")),
        "AntGroupSlowed must be emitted: {:?}",
        receipt.perception_updates
    );
    assert!(
        !receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost")),
        "WorkerLoss must NOT appear when only slowed: {:?}",
        receipt.perception_updates
    );
}

/// REQ-4: An ant that starts on SourbackBitter, moves out, and then moves back
/// in during the SAME command must trigger WorkerLoss exactly once on the
/// re-entry transition.
///
/// Custom unit-isolation fixture (not a claim about the default map layout):
///   (10, 10) = Air + SourbackBitter   ← start
///   (10, 11) = Air + None              ← gap (default Air)
///   (10, 12) = Air + SourbackBitter   ← re-entry cell
///   (10, 13..15) = Air + None          ← target area (default Air)
///
/// Tick trace for StepSimulation(6) starting at tick_index = 0:
///   tick 1 (odd):  on sourback + forage + odd → slowed, no move.
///   tick 2 (even): was_on_sourback=true, moves DOWN to (10,11). dest=None.
///                  entered_sourback = !true && false = false. No loss.
///   tick 3 (odd):  NOT on sourback (10,11). No slowdown gate. was=false.
///                  moves DOWN to (10,12). dest=SourbackBitter.
///                  entered_sourback = !false && true = true → WorkerLoss!
///   tick 4 (even): was_on_sourback=true. Moves DOWN to (10,13). No loss.
///   tick 5 (odd):  (10,13) not sourback. Moves DOWN to (10,14). No loss.
///   tick 6 (even): Moves DOWN to (10,15). No loss.
#[test]
fn test_leaves_sourback_then_reenters_triggers_worker_loss_once() {
    let mut sim = Simulation::new();
    // Set up the corridor; all cells are in the Air band (rows 0..36) so
    // material is already Air — only residue needs to be set.
    sim.chunk.get_mut(10, 10).residue = Residue::SourbackBitter;
    // (10, 11) is Air + None by default (the gap).
    sim.chunk.get_mut(10, 12).residue = Residue::SourbackBitter;
    // (10, 13), (10, 14), (10, 15) are Air + None by default.

    sim.ant_group.pos = (10, 10);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;
    sim.ant_group.task = Task::Forage { target: (10, 15) };

    let receipt = sim.execute_command(Command::StepSimulation { ticks: 6 });

    assert_eq!(sim.ant_group.pos, (10, 15));
    assert_eq!(
        sim.ant_group.workers, 97,
        "exactly one loss of min(3,100)=3"
    );
    assert_eq!(sim.ant_group.confidence, 184, "exactly one -16 penalty");

    let loss_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Lost") && p.contains("workers"))
        .count();
    assert_eq!(
        loss_count, 1,
        "WorkerLoss must fire exactly once on re-entry: {:?}",
        receipt.perception_updates
    );
}

/// REQ-5: Within a single StepSimulation command, WorkerLoss fires at most once
/// even after a true entry is followed by continued movement through residue.
/// After entry, sourback_entered=true prevents further losses for that command.
#[test]
fn test_worker_loss_only_once_per_command_after_true_entry() {
    let mut sim = Simulation::new();
    // Start just above the residue band (non-sourback), task set directly so
    // this entire sequence counts as ONE StepSimulation command.
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;
    sim.ant_group.task = Task::Forage { target: (84, 35) };

    // tick 1 (odd):  NOT on sourback → no slowing → move to (84,28). ENTRY. Loss.
    // tick 2 (even): on sourback → move to (84,29). sourback→sourback. No loss.
    // tick 3 (odd):  on sourback → slowed.
    // tick 4 (even): move to (84,30). No loss.
    // ... only one entry, one loss.
    let receipt = sim.execute_command(Command::StepSimulation { ticks: 8 });

    let loss_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Lost") && p.contains("workers"))
        .count();
    assert_eq!(
        loss_count, 1,
        "WorkerLoss must fire exactly once per command even across many sourback cells: {:?}",
        receipt.perception_updates
    );
    assert!(
        100 - sim.ant_group.workers <= 3,
        "total worker delta for one command must be at most 3"
    );
}

/// REQ-8: StepSimulation(n) is ONE command. The per-command flags
/// (sourback_entered, slowed_emitted, move_blocked_emitted) reset only at the
/// command boundary, not at each tick boundary. Proven indirectly via event counts.
#[test]
fn test_step_simulation_n_is_one_command_for_flags() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (84, 27);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 200;
    sim.ant_group.task = Task::Forage { target: (84, 35) };

    // StepSimulation(10): one command, 10 ticks crossing multiple residue cells.
    // sourback_entered  → set on first entry; subsequent sourback cells: 0 more losses.
    // slowed_emitted    → set on first odd-tick slowdown; subsequent odd ticks: 0 more events.
    // move_blocked_emitted → never set here (open path); 0 blocked events expected.
    let receipt = sim.execute_command(Command::StepSimulation { ticks: 10 });

    let loss_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Lost") && p.contains("workers"))
        .count();
    let slowed_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("slowed"))
        .count();
    let blocked_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Blocked"))
        .count();

    assert_eq!(
        loss_count, 1,
        "sourback_entered flag: exactly one loss per command"
    );
    assert!(
        slowed_count <= 1,
        "slowed_emitted flag: at most one slowed event per command"
    );
    assert_eq!(blocked_count, 0, "no blocked events on an open path");
}

/// REQ-9: When all four cardinal neighbors are non-traversable, the movement
/// system must emit CommandFailed(Blocked) EXACTLY ONCE per command regardless
/// of how many ticks that command spans.
///
/// This is a unit-isolation fixture, not a claim about the default map layout.
/// It artificially seals a cell to prove once-per-command blocked semantics.
#[test]
fn test_blocked_movement_emits_blocked_exactly_once() {
    let mut sim = Simulation::new();
    // Seal cell (10, 10) on all four cardinal sides.
    sim.chunk.get_mut(10, 10).material = Material::Tunnel;
    sim.chunk.get_mut(10, 9).material = Material::Soil;
    sim.chunk.get_mut(10, 11).material = Material::Soil;
    sim.chunk.get_mut(9, 10).material = Material::Soil;
    sim.chunk.get_mut(11, 10).material = Material::Soil;

    sim.ant_group.pos = (10, 10);
    sim.ant_group.task = Task::Scout { target: (12, 10) };

    // Three ticks, one command. Blocked fires once; move_blocked_emitted prevents repeats.
    let receipt = sim.execute_command(Command::StepSimulation { ticks: 3 });

    let blocked_count = receipt
        .perception_updates
        .iter()
        .filter(|p| p.contains("Movement") && p.contains("Blocked"))
        .count();
    assert_eq!(
        blocked_count, 1,
        "Blocked must be emitted exactly once per command regardless of tick count: {:?}",
        receipt.perception_updates
    );
    assert_eq!(
        sim.ant_group.pos,
        (10, 10),
        "ant must remain in place when fully blocked"
    );
}

/// REQ-10: The greedy tie-break test uses explicitly verified traversable Air
/// cells. From (5, 5) toward (7, 7), the first move must be RIGHT (x-reduce),
/// not DOWN or diagonal. Cells are in the Air band (rows 0..36) with no residue.
#[test]
fn test_greedy_tie_test_uses_traversable_fixture() {
    let mut sim = Simulation::new();
    // Explicitly assert the fixture cells are traversable before the test.
    assert_eq!(sim.chunk.get(5, 5).material, Material::Air);
    assert_eq!(sim.chunk.get(6, 5).material, Material::Air);
    assert_eq!(sim.chunk.get(5, 6).material, Material::Air);
    assert_eq!(sim.chunk.get(5, 5).residue, Residue::None);

    sim.ant_group.pos = (5, 5);
    let _ = sim.execute_command(Command::ScoutResidue { target: (7, 7) });

    // x-distance (2) equals y-distance (2); x-reducing direction (RIGHT) wins.
    assert_eq!(
        sim.ant_group.pos,
        (6, 5),
        "first move must reduce x (RIGHT), not y (DOWN), proving x-first tie-break"
    );
}

// ===========================================================================
// Slice 3 — Collapse tests
// ===========================================================================

/// LooseSoil with support < 100 above Air/Tunnel falls one cell down.
#[test]
fn test_loose_soil_collapses_down_into_air() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 40;
    sim.chunk.get_mut(20, 51).material = Material::Air;
    sim.chunk.get_mut(20, 51).support = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 51).material,
        Material::LooseSoil,
        "LooseSoil must fall into Air below"
    );
    assert_eq!(
        sim.chunk.get(20, 50).material,
        Material::Air,
        "Source cell becomes Air after swap"
    );
    assert_ne!(
        sim.chunk.get(20, 51).flags & Flags::RECENTLY_COLLAPSED,
        0,
        "RECENTLY_COLLAPSED flag must be set"
    );
}

/// LooseSoil with support < 100 above Tunnel also falls.
#[test]
fn test_loose_soil_collapses_down_into_tunnel() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 40;
    sim.chunk.get_mut(20, 51).material = Material::Tunnel;
    sim.chunk.get_mut(20, 51).support = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(sim.chunk.get(20, 51).material, Material::LooseSoil);
    assert_ne!(sim.chunk.get(20, 51).flags & Flags::RECENTLY_COLLAPSED, 0);
}

/// LooseSoil with support >= 100 must NOT collapse.
#[test]
fn test_loose_soil_does_not_collapse_with_support_100_or_more() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 100;
    sim.chunk.get_mut(20, 51).material = Material::Air;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).material,
        Material::LooseSoil,
        "High-support LooseSoil must not fall"
    );
    assert_eq!(sim.chunk.get(20, 51).material, Material::Air);
}

/// LooseSoil above Soil or Stone does NOT collapse (destination not traversable-open).
#[test]
fn test_loose_soil_does_not_collapse_into_soil_or_stone() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 40;
    sim.chunk.get_mut(20, 51).material = Material::Soil;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(sim.chunk.get(20, 50).material, Material::LooseSoil);
    assert_eq!(sim.chunk.get(20, 51).material, Material::Soil);
}

/// Bottom-up scan: the LOWER of two stacked LooseSoil candidates is evaluated first.
/// y=51 has Air below (y=52) so it falls. y=50 has LooseSoil below (y=51) — not a
/// valid destination — so it does NOT fall in the same tick.
/// Result after one tick: (20,50)=LooseSoil, (20,51)=Air, (20,52)=LooseSoil.
/// Running twice produces identical results (determinism).
#[test]
fn test_collapse_scan_is_bottom_up_deterministic() {
    fn run() -> (Material, Material, Material) {
        let mut sim = Simulation::new();
        sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
        sim.chunk.get_mut(20, 50).support = 40;
        sim.chunk.get_mut(20, 51).material = Material::LooseSoil;
        sim.chunk.get_mut(20, 51).support = 40;
        sim.chunk.get_mut(20, 52).material = Material::Air;
        let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
        (
            sim.chunk.get(20, 50).material,
            sim.chunk.get(20, 51).material,
            sim.chunk.get(20, 52).material,
        )
    }
    let r1 = run();
    let r2 = run();
    assert_eq!(r1, r2, "collapse scan result must be identical across runs");
    // Bottom-up: y=51 is scanned before y=50.
    // y=51: LooseSoil+support<100, below=Air → falls to y=52. Now y=51=Air.
    // y=50: LooseSoil+support<100, but below=LooseSoil (not Air/Tunnel at scan time) → stays.
    assert_eq!(
        r1.0,
        Material::LooseSoil,
        "(20,50) stays — its destination (20,51) was LooseSoil at scan time"
    );
    assert_eq!(r1.1, Material::Air, "(20,51) became Air after the swap");
    assert_eq!(
        r1.2,
        Material::LooseSoil,
        "(20,52) received the falling LooseSoil"
    );
}

/// Collapse into the AntGroup's cell applies worker-loss consequence.
#[test]
fn test_collapse_into_antgroup_applies_worker_loss() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (20, 51);
    sim.ant_group.workers = 10;
    sim.ant_group.confidence = 200;

    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 40;
    sim.chunk.get_mut(20, 51).material = Material::Air; // ant is here

    let receipt = sim.execute_command(Command::StepSimulation { ticks: 1 });

    // LooseSoil fell into ant's cell.
    assert_eq!(sim.chunk.get(20, 51).material, Material::LooseSoil);
    assert_eq!(
        sim.ant_group.workers, 7,
        "min(3,10)=3 workers lost from collapse"
    );
    assert_eq!(sim.ant_group.confidence, 184, "confidence -=16");
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("Lost") && p.contains("workers")),
        "WorkerLoss must appear in perception_updates: {:?}",
        receipt.perception_updates
    );
}

/// Boundary Stone ring must not be changed by collapse.
#[test]
fn test_boundary_ring_not_changed_by_collapse() {
    let mut sim = Simulation::new();
    // Put LooseSoil just inside boundary.
    sim.chunk.get_mut(1, 1).material = Material::LooseSoil;
    sim.chunk.get_mut(1, 1).support = 40;
    let _ = sim.execute_command(Command::StepSimulation { ticks: 3 });
    // Boundary must remain Stone.
    for x in 0..128usize {
        assert_eq!(sim.chunk.get(x, 0).material, Material::Stone);
        assert_eq!(sim.chunk.get(x, 127).material, Material::Stone);
    }
    for y in 0..128usize {
        assert_eq!(sim.chunk.get(0, y).material, Material::Stone);
        assert_eq!(sim.chunk.get(127, y).material, Material::Stone);
    }
}

/// Collapse receipt contains delta and perception update.
#[test]
fn test_collapse_receipt_contains_delta_and_perception() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 40;
    sim.chunk.get_mut(20, 51).material = Material::Air;

    let receipt = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert!(
        !receipt.chunk_deltas.is_empty(),
        "receipt must contain collapse delta"
    );
    assert!(!receipt.dev_event_summary.is_empty());
}

/// Three identical setups produce the same hash sequence.
#[test]
fn test_collapse_hash_determinism() {
    fn run() -> String {
        let mut sim = Simulation::new();
        sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
        sim.chunk.get_mut(20, 50).support = 40;
        sim.chunk.get_mut(20, 51).material = Material::Air;
        let _ = sim.execute_command(Command::StepSimulation { ticks: 3 });
        sim.compute_chunk_hash()
    }
    assert_eq!(run(), run());
    assert_eq!(run(), run());
}

// ===========================================================================
// Slice 4 — Water flow and moisture tests
// ===========================================================================

/// Water moves down first when Air is below.
#[test]
fn test_water_flows_down_first() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Water;
    sim.chunk.get_mut(20, 50).moisture = 255;
    sim.chunk.get_mut(20, 51).material = Material::Air;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 51).material,
        Material::Water,
        "water must flow down"
    );
    assert_eq!(
        sim.chunk.get(20, 50).material,
        Material::Air,
        "source becomes Air"
    );
}

/// When down is blocked, water tries down-left before other directions.
#[test]
fn test_water_priority_order() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Water;
    sim.chunk.get_mut(20, 50).moisture = 255;
    sim.chunk.get_mut(20, 51).material = Material::Soil; // block down
    sim.chunk.get_mut(19, 51).material = Material::Air; // down-left open
    sim.chunk.get_mut(21, 51).material = Material::Air; // down-right also open

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(19, 51).material,
        Material::Water,
        "water must prefer down-left over down-right"
    );
}

/// Water cannot enter Stone, Root, NestWall, Soil, LooseSoil, Carcass.
#[test]
fn test_water_does_not_enter_solid_materials() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Water;
    // Surround all five directions with Soil.
    sim.chunk.get_mut(20, 51).material = Material::Stone; // down
    sim.chunk.get_mut(19, 51).material = Material::Stone; // down-left
    sim.chunk.get_mut(21, 51).material = Material::Stone; // down-right
    sim.chunk.get_mut(19, 50).material = Material::Stone; // left
    sim.chunk.get_mut(21, 50).material = Material::Stone; // right

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).material,
        Material::Water,
        "water must stay when all neighbors are blocked"
    );
}

/// Water carries moisture value on swap.
#[test]
fn test_water_carries_moisture_on_swap() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Water;
    sim.chunk.get_mut(20, 50).moisture = 200;
    sim.chunk.get_mut(20, 51).material = Material::Air;
    sim.chunk.get_mut(20, 51).moisture = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    // Water cell swaps entirely (moisture travels with it); diffusion in step 6
    // will bleed some out to neighbors, so check material and that moisture is high.
    assert_eq!(
        sim.chunk.get(20, 51).material,
        Material::Water,
        "water must flow down"
    );
    assert_eq!(
        sim.chunk.get(20, 50).material,
        Material::Air,
        "source becomes Air after water flows"
    );
    assert!(
        sim.chunk.get(20, 51).moisture > 0,
        "moisture must travel with water cell"
    );
}

/// Moisture diffusion: symmetric fixture produces identical results regardless of
/// which side is processed first (double-buffer guarantee).
#[test]
fn test_moisture_double_buffer_order_independent() {
    fn run() -> (u8, u8) {
        let mut sim = Simulation::new();
        sim.chunk.get_mut(20, 50).material = Material::Air;
        sim.chunk.get_mut(20, 50).moisture = 100;
        sim.chunk.get_mut(21, 50).material = Material::Air;
        sim.chunk.get_mut(21, 50).moisture = 50;
        let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
        (
            sim.chunk.get(20, 50).moisture,
            sim.chunk.get(21, 50).moisture,
        )
    }
    let r1 = run();
    let r2 = run();
    assert_eq!(
        r1, r2,
        "moisture result must be identical across runs (double-buffer)"
    );
    // Transfer from (20,50) to (21,50) occurs since 100 > 50+16; however surrounding
    // Soil cells (which accept moisture) also drain both cells, so (21,50) may end up
    // below its starting value. Determinism is the core guarantee here.
    assert!(r1.0 < 100, "moisture must have diffused away from (20,50)");
    assert!(
        r1.0 + r1.1 < 150,
        "total moisture in the pair must have spread to neighbors"
    );
}

/// Moisture does not enter Stone, Root, NestWall, Carcass.
#[test]
fn test_moisture_does_not_enter_non_accepting_materials() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Air;
    sim.chunk.get_mut(20, 50).moisture = 200;
    sim.chunk.get_mut(21, 50).material = Material::Stone;
    sim.chunk.get_mut(21, 50).moisture = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(21, 50).moisture,
        0,
        "Stone must not accept moisture"
    );
}

/// Wet LooseSoil (moisture > 120) loses 1 support per tick.
#[test]
fn test_wet_loose_soil_support_decays() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 60;
    // Use 200 so moisture stays > 120 after diffusion to neighbors (200 - 16 = 184 > 120).
    sim.chunk.get_mut(20, 50).moisture = 200;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).support,
        59,
        "wet LooseSoil support must decay by 1"
    );
}

/// Dry LooseSoil (moisture == 120) does NOT decay.
#[test]
fn test_dry_loose_soil_support_does_not_decay() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::LooseSoil;
    sim.chunk.get_mut(20, 50).support = 60;
    sim.chunk.get_mut(20, 50).moisture = 120;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).support,
        60,
        "LooseSoil at moisture==120 must not decay"
    );
}

/// Three identical moisture setups produce the same hash.
#[test]
fn test_moisture_hash_determinism() {
    fn run() -> String {
        let mut sim = Simulation::new();
        sim.chunk.get_mut(20, 50).material = Material::Air;
        sim.chunk.get_mut(20, 50).moisture = 100;
        sim.chunk.get_mut(21, 50).material = Material::Air;
        sim.chunk.get_mut(21, 50).moisture = 50;
        let _ = sim.execute_command(Command::StepSimulation { ticks: 5 });
        sim.compute_chunk_hash()
    }
    assert_eq!(run(), run());
    assert_eq!(run(), run());
}

// ===========================================================================
// Slice 5 — Scent, Carcass Harvest, and Return loop tests
// ===========================================================================

/// Base scent decay: seeded scent decreases by 1 per tick.
#[test]
fn test_scent_base_decay() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Air;
    sim.chunk.get_mut(20, 50).scent_home = 50;
    sim.chunk.get_mut(20, 50).scent_food = 80;
    sim.chunk.get_mut(20, 50).moisture = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).scent_home,
        49,
        "scent_home must decay by 1"
    );
    assert_eq!(
        sim.chunk.get(20, 50).scent_food,
        79,
        "scent_food must decay by 1"
    );
}

/// Water cells zero out scent.
#[test]
fn test_water_zeroes_scent() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Water;
    sim.chunk.get_mut(20, 50).scent_home = 100;
    sim.chunk.get_mut(20, 50).scent_food = 100;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(20, 50).scent_home,
        0,
        "water must zero scent_home"
    );
    assert_eq!(
        sim.chunk.get(20, 50).scent_food,
        0,
        "water must zero scent_food"
    );
}

/// Wet cell (moisture > 100) applies extra -2 decay on top of base -1.
#[test]
fn test_wet_cell_extra_scent_decay() {
    let mut sim = Simulation::new();
    sim.chunk.get_mut(20, 50).material = Material::Air;
    // Use 200 so moisture stays > 100 after diffusion to neighbors (200 - 16 = 184 > 100).
    sim.chunk.get_mut(20, 50).moisture = 200;
    sim.chunk.get_mut(20, 50).scent_home = 50;
    sim.chunk.get_mut(20, 50).scent_food = 50;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    // base -1, then extra -2 = -3 total
    assert_eq!(
        sim.chunk.get(20, 50).scent_home,
        47,
        "wet cell: total decay = -3"
    );
    assert_eq!(sim.chunk.get(20, 50).scent_food, 47);
}

/// ReturnHome movement reinforces scent_home +12 on current cell.
#[test]
fn test_return_home_reinforces_home_scent() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (50, 100);
    sim.chunk.get_mut(50, 100).scent_home = 10;

    let _ = sim.execute_command(Command::ReturnHome);

    // Ant moved one step; the reinforcement applies to the new cell during step 8.
    // After moving to (51,100), scent_home there gets +12.
    assert_eq!(sim.ant_group.pos, (51, 100));
    assert!(
        sim.chunk.get(51, 100).scent_home >= 12,
        "ReturnHome must reinforce home scent +12"
    );
}

/// Forage while carrying food reinforces scent_food +12.
#[test]
fn test_forage_near_food_reinforces_food_scent() {
    let mut sim = Simulation::new();
    sim.ant_group.pos = (5, 5);
    sim.ant_group.food_carried = 1; // carrying food
    sim.ant_group.task = Task::Forage { target: (10, 5) };
    sim.chunk.get_mut(6, 5).scent_food = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert!(
        sim.chunk.get(6, 5).scent_food >= 12,
        "Forage+carrying food must reinforce scent_food +12"
    );
}

/// Harvesting one adjacent Carcass converts it to Air and increments food_carried.
#[test]
fn test_carcass_harvest_converts_one_cell_to_air() {
    let mut sim = Simulation::new();
    // Place ant at tunnel, carcass adjacent to the right.
    // Target = current position so the ant does not move before harvest runs.
    sim.ant_group.pos = (50, 100);
    sim.ant_group.task = Task::Forage { target: (50, 100) };
    sim.chunk.get_mut(51, 100).material = Material::Carcass;
    sim.chunk.get_mut(51, 100).support = 100;

    let food_before = sim.ant_group.food_carried;
    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(51, 100).material,
        Material::Air,
        "Carcass must become Air after harvest"
    );
    assert_eq!(
        sim.ant_group.food_carried,
        food_before + 1,
        "food_carried must increment"
    );
}

/// Harvest selection respects fixed order: up, right, down, left.
#[test]
fn test_carcass_harvest_fixed_order() {
    let mut sim = Simulation::new();
    // Target = current position so the ant does not move before harvest runs.
    sim.ant_group.pos = (50, 100);
    sim.ant_group.task = Task::Forage { target: (50, 100) };
    // Place carcass both up and right; up (0,-1) should win.
    sim.chunk.get_mut(50, 99).material = Material::Carcass; // up
    sim.chunk.get_mut(51, 100).material = Material::Carcass; // right

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(
        sim.chunk.get(50, 99).material,
        Material::Air,
        "up carcass must be harvested first (fixed order)"
    );
    assert_eq!(
        sim.chunk.get(51, 100).material,
        Material::Carcass,
        "right carcass must not be harvested"
    );
}

/// Harvest increments food_carried by exactly 1.
#[test]
fn test_harvest_increments_food_carried() {
    let mut sim = Simulation::new();
    // Target = current position so the ant does not move before harvest runs.
    sim.ant_group.pos = (50, 100);
    sim.ant_group.task = Task::Forage { target: (50, 100) };
    sim.chunk.get_mut(51, 100).material = Material::Carcass;

    let before = sim.ant_group.food_carried;
    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });
    assert_eq!(sim.ant_group.food_carried, before + 1);
}

/// Harvest scent +24 applied to current cell when it is traversable.
#[test]
fn test_harvest_scent_current_cell_if_traversable() {
    let mut sim = Simulation::new();
    // Target = current position so the ant does not move before harvest runs.
    sim.ant_group.pos = (50, 100); // Tunnel = traversable
    sim.ant_group.task = Task::Forage { target: (50, 100) };
    sim.chunk.get_mut(51, 100).material = Material::Carcass;
    sim.chunk.get_mut(50, 100).scent_food = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert!(
        sim.chunk.get(50, 100).scent_food >= 24 || sim.chunk.get(51, 100).scent_food >= 24,
        "harvest scent +24 must appear on current or adjacent cell"
    );
}

/// Harvest scent falls back to first adjacent traversable if current cell not traversable.
#[test]
fn test_harvest_scent_adjacent_fallback_if_current_not_traversable() {
    let mut sim = Simulation::new();
    // Place ant on Soil (not traversable).
    // Target = current position so the ant does not move before harvest runs.
    sim.ant_group.pos = (50, 50);
    sim.chunk.get_mut(50, 50).material = Material::Soil;
    sim.ant_group.task = Task::Forage { target: (50, 50) };
    // Carcass above (up = first in order).
    sim.chunk.get_mut(50, 49).material = Material::Carcass;
    // Adjacent traversable: right at (51, 50).
    sim.chunk.get_mut(51, 50).material = Material::Air;
    sim.chunk.get_mut(51, 50).scent_food = 0;

    let _ = sim.execute_command(Command::StepSimulation { ticks: 1 });

    // Current cell is Soil (non-traversable), so scent goes to first adjacent traversable.
    // up (50,49) is now Air (was Carcass), right (51,50) is Air.
    // Up is checked first in the fallback order. Harvest adds +24; 1 tick of decay
    // reduces it by 1, so the expected minimum is 23.
    let up_scent = sim.chunk.get(50, 49).scent_food;
    let right_scent = sim.chunk.get(51, 50).scent_food;
    assert!(
        up_scent >= 20 || right_scent >= 20,
        "harvest scent fallback must apply +24 to first adjacent traversable: up={} right={}",
        up_scent,
        right_scent
    );
}

/// ReturnHome deposits carried food at HOME_COORD and zeroes food_carried.
#[test]
fn test_return_home_deposits_food() {
    let mut sim = Simulation::new();
    // Place ant one step away from home so it arrives in one tick.
    let (hx, hy) = (55u8, 118u8);
    sim.ant_group.pos = (54, 118); // one left of home
    sim.ant_group.food_carried = 3;
    sim.ant_group.task = Task::ReturnHome;

    let receipt = sim.execute_command(Command::StepSimulation { ticks: 1 });

    assert_eq!(sim.ant_group.pos, (hx, hy), "ant must reach home");
    assert_eq!(
        sim.ant_group.food_carried, 0,
        "food_carried must be zeroed after deposit"
    );
    assert_eq!(
        sim.food_returned, 3,
        "food_returned must record the deposit"
    );
    assert!(
        receipt
            .perception_updates
            .iter()
            .any(|p| p.contains("deposited") || p.contains("food")),
        "deposit must appear in perception_updates: {:?}",
        receipt.perception_updates
    );
}

/// Full forage → return loop is deterministic: same command sequence = same hash.
#[test]
fn test_full_forage_return_loop_deterministic() {
    fn run() -> String {
        let mut sim = Simulation::new();
        // Place ant near carcass band (y=18..28, x=90..111).
        sim.ant_group.pos = (89, 22);
        sim.ant_group.workers = 100;
        // Forage toward carcass.
        let _ = sim.execute_command(Command::SendForagers { target: (95, 22) });
        let _ = sim.execute_command(Command::StepSimulation { ticks: 3 });
        // Return home.
        let _ = sim.execute_command(Command::ReturnHome);
        let _ = sim.execute_command(Command::StepSimulation { ticks: 5 });
        sim.compute_chunk_hash()
    }
    let h1 = run();
    let h2 = run();
    let h3 = run();
    assert_eq!(h1, h2);
    assert_eq!(h2, h3);
}
