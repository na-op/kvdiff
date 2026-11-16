use kvdiff::{diff_maps, parse_kv, EntryChange};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mask = args.iter().any(|a| a == "--mask-values");
    let files: Vec<&String> = args.iter().skip(1).filter(|a| !a.starts_with("--")).collect();
    if files.len() != 2 {
        eprintln!("Usage: kvdiff [--mask-values] <left.env> <right.env>");
        process::exit(2);
    }
    let left_raw = fs::read_to_string(files[0]).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", files[0], e);
        process::exit(2);
    });
    let right_raw = fs::read_to_string(files[1]).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", files[1], e);
        process::exit(2);
    });
    let changes = diff_maps(&parse_kv(&left_raw), &parse_kv(&right_raw));
    if changes.is_empty() {
        println!("No differences found.");
        return;
    }
    for (k, ch) in &changes {
        match ch {
            EntryChange::Added(v) => {
                let shown = if mask { "***" } else { v.as_str() };
                println!("+ {}={}", k, shown);
            }
            EntryChange::Removed(v) => {
                let shown = if mask { "***" } else { v.as_str() };
                println!("- {}={}", k, shown);
            }
            EntryChange::Modified { old, new } => {
                let (o, n) = if mask { ("***", "***") } else { (old.as_str(), new.as_str()) };
                println!("~ {}: {} -> {}", k, o, n);
            }
        }
    }
    process::exit(1);
}
