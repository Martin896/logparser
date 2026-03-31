use std::fs;
use std::env;
use std::process;

fn main() {

    let args : Vec<String>  = env::args().collect();
    if args.len() < 3 {
        eprintln!("An Error occured: invalid file path");
        eprintln!("Usage: cargo run -- <file path>");
        process::exit(1);
    }
    let sys_log = fs::read_to_string(&args[1])
    .expect("Failed to read System log file");
    //println!("{sys_log}");
    
    let mut error_count = 0;
    for line in sys_log.lines() {
        if line.contains(&args[2]) {
            println!("{line}");
            error_count += 1;

        }
        
    }
    println!("Found {error_count} matches for '{}'", &args[2]);
    let sys_log_count  = sys_log.lines().count();
    println!("The number of system logs: {sys_log_count}");
}