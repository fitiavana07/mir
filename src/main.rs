// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Fitiavana Ramanandafy <hello@fitiavana.dev>

use std::{fs, io::ErrorKind};

fn main() {
    // Get the current directory
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Error: directory not found, it may have been deleted")
                }
                _ => eprintln!("Error: Could not access current directory: {}", e),
            };
            std::process::exit(1);
        }
    };

    // Read directory entries
    let entries = match fs::read_dir(&current_dir) {
        Ok(entries) => entries,
        Err(e) => {
            match e.kind() {
                ErrorKind::PermissionDenied => eprintln!("Error: permission denied"),
                _ => eprintln!("Error: Could not read directory: {}", e),
            }
            std::process::exit(1);
        }
    };

    // Collect and sort entries
    let mut paths: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();
    paths.sort();

    // Print each entry
    for path in paths {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("???");

        if path.is_dir() {
            println!("{}/", name);
        } else {
            println!("{}", name);
        }
    }
}
