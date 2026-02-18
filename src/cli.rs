mod core;

use std::env;
use std::process;

use core::{FileOrganizerCore, FileOperationMode};

fn print_usage() {
    println!("Usage: organizer-cli <path> [OPTIONS]");
    println!();
    println!("Organize files by extension in the specified path");
    println!();
    println!("Arguments:");
    println!("  <path>              Path to the directory to organize");
    println!();
    println!("Options:");
    println!("  -c, --copy          Copy files instead of moving them");
    println!("  -h, --help          Print help information");
    println!();
    println!("Examples:");
    println!("  organizer-cli /home/user/Downloads");
    println!("  organizer-cli /home/user/Downloads --copy");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No path provided");
        print_usage();
        process::exit(1);
    }

    let mut path: Option<String> = None;
    let mut mode = FileOperationMode::Cut;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-c" | "--copy" => {
                mode = FileOperationMode::Copy;
            }
            arg if !arg.starts_with('-') => {
                if path.is_none() {
                    path = Some(arg.to_string());
                } else {
                    eprintln!("Error: Unexpected argument '{}'", arg);
                    print_usage();
                    process::exit(1);
                }
            }
            arg => {
                eprintln!("Error: Unknown option '{}'", arg);
                print_usage();
                process::exit(1);
            }
        }
        i += 1;
    }

    let path = match path {
        Some(p) => p,
        None => {
            eprintln!("Error: No path provided");
            print_usage();
            process::exit(1);
        }
    };

    let operation = match mode {
        FileOperationMode::Cut => "Moving",
        FileOperationMode::Copy => "Copying",
    };
    println!("{} files from: {}", operation, path);
    println!();

    match FileOrganizerCore::organize_by_extension(&path, mode) {
        Ok(result) => {
            println!("{}", result.summary);

            if let Some(errors) = &result.errors {
                println!("\nErrors:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
