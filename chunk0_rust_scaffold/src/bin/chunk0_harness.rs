use chunk0_rust_scaffold::harness;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chunk0_harness <command> [options]");
        eprintln!("Commands:");
        eprintln!("  run-corpus");
        eprintln!("  stress-local-fixtures --seed <N> --cases <N>");
        eprintln!("  compare-baseline <file_a> <file_b>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "run-corpus" => cmd_run_corpus(),
        "stress-local-fixtures" => cmd_stress(&args[2..]),
        "compare-baseline" => cmd_compare(&args[2..]),
        other => {
            eprintln!("Unknown command: {}", other);
            std::process::exit(1);
        }
    }
}

fn ensure_output_dir() {
    let dir = Path::new("optimization_runs/latest");
    fs::create_dir_all(dir).expect("failed to create optimization_runs/latest");
}

fn cmd_run_corpus() {
    ensure_output_dir();

    let results = harness::run_corpus();

    let csv_header = "script_name,command_count,ticks_run,hash_before,hash_after,hash_sequence,event_counts,chunk_delta_count,final_x,final_y,final_workers,final_confidence,invariants_passed,failure_reason";
    let mut csv_rows = vec![csv_header.to_string()];
    let mut summary_lines = Vec::new();
    let mut all_passed = true;

    for r in &results {
        if !r.invariants_passed {
            all_passed = false;
        }
        let status = if r.invariants_passed { "PASS" } else { "FAIL" };
        summary_lines.push(format!(
            "[{}] {} | cmds={} ticks={} deltas={} events={} workers={} conf={} pos=({},{}) hash_after={}",
            status,
            r.script_name,
            r.command_count,
            r.ticks_run,
            r.chunk_delta_count,
            r.event_counts,
            r.final_workers,
            r.final_confidence,
            r.final_x,
            r.final_y,
            &r.hash_after[..12],
        ));
        if let Some(ref reason) = r.failure_reason {
            summary_lines.push(format!("  FAILURE: {}", reason));
        }
        csv_rows.push(r.to_csv_row());
    }

    let overall = if all_passed {
        "ALL PASSED"
    } else {
        "SOME FAILED"
    };
    summary_lines.push(format!(
        "\nCorpus result: {} ({}/{})",
        overall,
        results.iter().filter(|r| r.invariants_passed).count(),
        results.len()
    ));

    let summary_text = summary_lines.join("\n");
    let csv_text = csv_rows.join("\n");

    println!("{}", summary_text);

    fs::write(
        "optimization_runs/latest/run_corpus_summary.txt",
        &summary_text,
    )
    .expect("failed to write summary");
    fs::write("optimization_runs/latest/run_corpus_hashes.csv", &csv_text)
        .expect("failed to write csv");

    println!("\nOutputs written to optimization_runs/latest/");

    if !all_passed {
        std::process::exit(1);
    }
}

fn cmd_stress(args: &[String]) {
    ensure_output_dir();

    let mut seed = 42u64;
    let mut cases = 100usize;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                i += 1;
                seed = args[i].parse().expect("--seed requires a u64");
            }
            "--cases" => {
                i += 1;
                cases = args[i].parse().expect("--cases requires a usize");
            }
            other => {
                eprintln!("Unknown flag: {}", other);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let result = harness::stress_local_fixtures(seed, cases);
    println!("{}", result.summary);

    fs::write(
        "optimization_runs/latest/stress_local_fixtures_summary.txt",
        &result.summary,
    )
    .expect("failed to write stress summary");

    println!("Output written to optimization_runs/latest/stress_local_fixtures_summary.txt");

    if result.failed > 0 {
        std::process::exit(1);
    }
}

fn cmd_compare(args: &[String]) {
    if args.len() < 2 {
        eprintln!("compare-baseline requires two file paths");
        std::process::exit(1);
    }
    // Guarded stub: not yet implemented beyond simple text diff
    println!("compare-baseline: not fully implemented yet (guarded stub).");
    println!("File A: {}", args[0]);
    println!("File B: {}", args[1]);
    let a = fs::read_to_string(&args[0]).unwrap_or_else(|_| format!("<cannot read {}>", args[0]));
    let b = fs::read_to_string(&args[1]).unwrap_or_else(|_| format!("<cannot read {}>", args[1]));
    if a == b {
        println!("Files are identical.");
    } else {
        println!("Files differ.");
        // Print first differing line
        for (i, (la, lb)) in a.lines().zip(b.lines()).enumerate() {
            if la != lb {
                println!("  First diff at line {}: {:?} vs {:?}", i + 1, la, lb);
                break;
            }
        }
    }
}
