//! Lock 1d CLI — recipe safety scan.
//!
//! `recipe-scan --path <file-or-dir>`: scans recipe files (yaml/yml/md)
//! for invisible/deceptive Unicode. Exit 0 = clean; exit 1 = smuggling
//! found (each hit listed); exit 2 = usage/IO error. Fails closed; never
//! auto-strips.

use holmes_guard::scan::recipes;
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut path: Option<PathBuf> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--path" => path = args.next().map(PathBuf::from),
            _ => {
                eprintln!("usage: recipe-scan --path <file-or-dir>");
                std::process::exit(2);
            }
        }
    }
    let Some(path) = path else {
        eprintln!("usage: recipe-scan --path <file-or-dir>");
        std::process::exit(2);
    };

    match recipes::scan_path(&path) {
        Ok((files, hits)) => {
            println!("recipe safety scan (invisible/deceptive Unicode)");
            println!("  files scanned: {files}");
            if hits.is_empty() {
                println!("  verdict: CLEAN");
                std::process::exit(0);
            }
            for hit in &hits {
                println!("  smuggling: {hit}");
            }
            println!("  verdict: FAIL ({} hit(s))", hits.len());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("recipe-scan: cannot scan {}: {e}", path.display());
            std::process::exit(2);
        }
    }
}
