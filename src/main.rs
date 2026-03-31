use std::fs;
use std::env;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No file path provided");
        eprintln!("Usage: cargo run -- <file path>          - For total log entries\n       cargo run -- <file path> <keyword> - Search log for keyword" );
        process::exit(1);

    }

    let sys_log = fs::read_to_string(&args[1])
    .expect("Failed to read System log");

    // println!("{sys_log}");
    let sys_log_count = sys_log.lines().count();

    println!("Total system log: {sys_log_count} ");

    let mut error_count  = 0;

    for line in sys_log.lines() {
        if line.contains(&args[2]) {
            error_count += 1;

        }
    }
    println!("Found {error_count} matches for keyword {}", &args[2]);
        
    
}