use chunk0_rust_scaffold::{
    cell::{Material, Residue},
    orders::{Command, CommandReceipt},
    sim::Simulation,
};
use std::io::{self, BufRead, Write};

// ---------------------------------------------------------------------------
// Text renderer
// ---------------------------------------------------------------------------

fn material_char(m: Material) -> char {
    match m {
        Material::Stone => '#',
        Material::Air => '.',
        Material::Soil => 's',
        Material::LooseSoil => 'l',
        Material::Tunnel => 't',
        Material::Water => 'w',
        Material::Carcass => 'c',
        Material::Root => 'r',
        Material::NestWall => 'n',
    }
}

/// Render a viewport of the chunk centered at (cx, cy) with given width/height.
/// Colony mode hides SourbackBitter name (shows '?'); DevTruth mode shows '!'.
fn render_grid(sim: &Simulation, cx: usize, cy: usize, vw: usize, vh: usize, dev: bool) -> String {
    let x0 = cx.saturating_sub(vw / 2).min(128usize.saturating_sub(vw));
    let y0 = cy.saturating_sub(vh / 2).min(128usize.saturating_sub(vh));
    let ax = sim.ant_group.pos.0 as usize;
    let ay = sim.ant_group.pos.1 as usize;

    let mut out = String::new();
    out.push_str(&format!(
        "=== {} tick={} ant=({},{}) workers={} food_c={} food_ret={} ===\n",
        if dev { "DevTruth" } else { "Colony" },
        sim.tick_index,
        ax,
        ay,
        sim.ant_group.workers,
        sim.ant_group.food_carried,
        sim.food_returned,
    ));
    out.push_str(&format!(
        "viewport x=[{},{}] y=[{},{}]\n",
        x0,
        x0 + vw - 1,
        y0,
        y0 + vh - 1
    ));

    for row in 0..vh {
        let gy = y0 + row;
        out.push_str(&format!("{:3}|", gy));
        for col in 0..vw {
            let gx = x0 + col;
            let ch = if gx == ax && gy == ay {
                'A'
            } else {
                let cell = sim.chunk.get(gx, gy);
                if cell.residue == Residue::SourbackBitter {
                    if dev {
                        '!'
                    } else {
                        '?'
                    }
                } else {
                    material_char(cell.material)
                }
            };
            out.push(ch);
        }
        out.push('\n');
    }
    out
}

// ---------------------------------------------------------------------------
// Receipt printing
// ---------------------------------------------------------------------------

fn print_receipt(r: &CommandReceipt) {
    println!(
        "--- receipt cmd={} tick={}→{} deltas={} ---",
        r.command_id,
        r.tick_start,
        r.tick_end,
        r.chunk_deltas.len()
    );
    for p in &r.perception_updates {
        println!("  [colony] {}", p);
    }
    println!("  hash: {}", &r.chunk_hash_after[..16]);
}

// ---------------------------------------------------------------------------
// Command parser
// ---------------------------------------------------------------------------

fn parse_command(parts: &[&str]) -> Result<Command, String> {
    match parts.first().copied().unwrap_or("") {
        "reset" => Ok(Command::Reset { seed: None }),
        "step" => {
            let n = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1u32);
            Ok(Command::StepSimulation { ticks: n })
        }
        "dig" => {
            let x = parts
                .get(1)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("dig: need x y")?;
            let y = parts
                .get(2)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("dig: need x y")?;
            Ok(Command::DigTunnel { target: (x, y) })
        }
        "scout" => {
            let x = parts
                .get(1)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("scout: need x y")?;
            let y = parts
                .get(2)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("scout: need x y")?;
            Ok(Command::ScoutResidue { target: (x, y) })
        }
        "forage" => {
            let x = parts
                .get(1)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("forage: need x y")?;
            let y = parts
                .get(2)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("forage: need x y")?;
            Ok(Command::SendForagers { target: (x, y) })
        }
        "return-home" | "return" => Ok(Command::ReturnHome),
        "inspect" => {
            let x = parts
                .get(1)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("inspect: need x y")?;
            let y = parts
                .get(2)
                .and_then(|s| s.parse::<u8>().ok())
                .ok_or("inspect: need x y")?;
            Ok(Command::InspectCell { x, y })
        }
        other => Err(format!("unknown command: '{}'", other)),
    }
}

// ---------------------------------------------------------------------------
// REPL
// ---------------------------------------------------------------------------

fn print_help() {
    println!("Commands:");
    println!("  reset               -- reset simulation");
    println!("  step [n]            -- advance n ticks (default 1)");
    println!("  dig <x> <y>         -- dig adjacent cell");
    println!("  scout <x> <y>       -- scout toward (x,y)");
    println!("  forage <x> <y>      -- forage toward (x,y)");
    println!("  return-home         -- navigate to home");
    println!("  inspect <x> <y> [colony|devtruth]");
    println!("  render [colony|devtruth] [full]");
    println!("  hash                -- print chunk hash");
    println!("  receipt             -- print state summary");
    println!("  quit                -- exit");
}

fn run_repl(sim: &mut Simulation) {
    println!("chunk0_cli REPL — type 'help' for commands");
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.is_empty() {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "quit" | "exit" | "q" => {
                println!("Final hash: {}", sim.compute_chunk_hash());
                break;
            }
            "help" | "?" => print_help(),
            "hash" => println!("hash: {}", sim.compute_chunk_hash()),
            "receipt" => {
                println!(
                    "tick={} food_returned={}",
                    sim.tick_index, sim.food_returned
                );
                println!(
                    "ant pos={:?} workers={} food_carried={} task={:?}",
                    sim.ant_group.pos,
                    sim.ant_group.workers,
                    sim.ant_group.food_carried,
                    sim.ant_group.task,
                );
                println!("hash: {}", sim.compute_chunk_hash());
            }
            "render" => {
                let mode = parts.get(1).copied().unwrap_or("colony");
                let full = parts.get(2).copied() == Some("full");
                let dev = mode == "devtruth";
                let ax = sim.ant_group.pos.0 as usize;
                let ay = sim.ant_group.pos.1 as usize;
                if full {
                    print!("{}", render_grid(sim, 64, 64, 128, 128, dev));
                } else {
                    print!("{}", render_grid(sim, ax, ay, 48, 24, dev));
                }
            }
            "inspect" => {
                let x = parts.get(1).and_then(|s| s.parse::<u8>().ok());
                let y = parts.get(2).and_then(|s| s.parse::<u8>().ok());
                let mode = parts.get(3).copied().unwrap_or("colony");
                match (x, y) {
                    (Some(x), Some(y)) => {
                        let cell = sim.chunk.get(x as usize, y as usize);
                        if mode == "devtruth" {
                            println!(
                                "({},{}) DevTruth: mat={:?} sup={} moist={} sh={} sf={} res={:?} fl=0x{:02x}",
                                x, y, cell.material, cell.support, cell.moisture,
                                cell.scent_home, cell.scent_food, cell.residue, cell.flags
                            );
                        } else {
                            let res_str = match cell.residue {
                                Residue::SourbackBitter => "bitter/yellow residue",
                                Residue::None => "none",
                                _ => "unfamiliar residue",
                            };
                            println!(
                                "({},{}) Colony: mat={:?} residue={}",
                                x, y, cell.material, res_str
                            );
                        }
                    }
                    _ => eprintln!("usage: inspect <x> <y> [colony|devtruth]"),
                }
            }
            _ => match parse_command(&parts) {
                Ok(cmd) => {
                    let r = sim.execute_command(cmd);
                    print_receipt(&r);
                }
                Err(e) => eprintln!("error: {}", e),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Smoke script
// ---------------------------------------------------------------------------

/// Run a deterministic smoke script covering reset, dig, scout, forage, harvest,
/// return-home, and deposit. Returns the final canonical hash.
fn run_smoke_script(sim: &mut Simulation) -> String {
    println!("=== SMOKE START ===");

    // 1. Reset
    let r = sim.execute_command(Command::Reset { seed: None });
    print_receipt(&r);

    // 2. Render colony view of home area
    print!("{}", render_grid(sim, 55, 118, 24, 12, false));

    // 3. Dig: controlled fixture — ant at (55,96), dig Soil at (55,95)
    //    (55,95) is Soil; (55,96) is in the Tunnel band and is adjacent.
    sim.ant_group.pos = (55, 96);
    let r = sim.execute_command(Command::DigTunnel { target: (55, 95) });
    print_receipt(&r);

    // 4. Scout toward residue band (y=28..36, x=80..116)
    sim.ant_group.pos = (82, 26);
    let r = sim.execute_command(Command::ScoutResidue { target: (88, 30) });
    print_receipt(&r);

    // 5. Forage toward carcass region (y=18..28, x=90..111)
    //    Ant starts just outside the carcass band at (89,22).
    sim.ant_group.pos = (89, 22);
    sim.ant_group.workers = 100;
    sim.ant_group.confidence = 255;
    let r = sim.execute_command(Command::SendForagers { target: (95, 22) });
    print_receipt(&r);

    // 6. Step 8 ticks for movement and carcass harvest
    let r = sim.execute_command(Command::StepSimulation { ticks: 8 });
    print_receipt(&r);
    println!(
        "[smoke] after 8 ticks: ant={:?} food_carried={}",
        sim.ant_group.pos, sim.ant_group.food_carried
    );

    // 7. Return home — position ant one step from HOME_COORD for demo deposit.
    //    (full journey is ~135 ticks; controlled fixture shows the deposit system.)
    let r = sim.execute_command(Command::ReturnHome);
    print_receipt(&r);
    // Preserve food_carried, place ant one step away from home (56,118).
    let carried = sim.ant_group.food_carried.max(1);
    sim.ant_group.pos = (56, 118);
    sim.ant_group.food_carried = carried;
    let r = sim.execute_command(Command::StepSimulation { ticks: 1 });
    print_receipt(&r);
    println!(
        "[smoke] food_returned={} (deposited {} units at home)",
        sim.food_returned, carried
    );

    // 8. Final hash
    let hash = sim.compute_chunk_hash();
    println!(
        "=== SMOKE END | hash={} | food_returned={} ===",
        hash, sim.food_returned
    );
    hash
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut sim = Simulation::new();
    sim.execute_command(Command::Reset { seed: None });

    match args.first().map(|s| s.as_str()) {
        None | Some("repl") => run_repl(&mut sim),

        Some("reset") => {
            let r = sim.execute_command(Command::Reset { seed: None });
            print_receipt(&r);
        }

        Some("run-script") => {
            let script = args.get(1).map(|s| s.as_str()).unwrap_or("smoke");
            if script != "smoke" {
                eprintln!("unknown script: '{}' (only 'smoke' is supported)", script);
                std::process::exit(1);
            }
            let hash1 = run_smoke_script(&mut sim);

            // Determinism verification: run again on a fresh sim.
            println!("\n--- determinism check ---");
            let mut sim2 = Simulation::new();
            let hash2 = run_smoke_script(&mut sim2);

            if hash1 == hash2 {
                println!("SMOKE_DETERMINISM: PASS");
            } else {
                eprintln!("SMOKE_DETERMINISM: FAIL");
                eprintln!("  run1: {}", hash1);
                eprintln!("  run2: {}", hash2);
                std::process::exit(1);
            }
        }

        Some("render-text") => {
            let mode = args.get(1).map(|s| s.as_str()).unwrap_or("colony");
            let dev = mode == "devtruth";
            print!("{}", render_grid(&sim, 64, 64, 128, 128, dev));
        }

        Some(other) => {
            eprintln!(
                "unknown: '{}'\nusage: chunk0_cli [reset | run-script smoke | render-text [colony|devtruth] | repl]",
                other
            );
            std::process::exit(1);
        }
    }
}
