use std::fs;
use std::env;
use std::process;

struct LogEntry {
    timestamp: String,
    hostname: String,
    process: String,
    message: String
}

fn main() {

    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Expected Error: No file path provided");
        eprintln!("Usage: cargo run -- <file path>");

        process::exit(1);
    } 
    let sys_log = fs::read_to_string(&args[1])
    .expect("Failed to read system log");

    let syslog_count = sys_log.lines().count();
    println!("Total system log: {syslog_count}");
    
    let mut keyword_count = 0;
    if args.len() >= 3 {

        for line in sys_log.lines() {
            if line.contains(&args[2]) {
                if keyword_count < 5 {
                    let log_entry = LogEntry {
                        timestamp: line.split_whitespace().nth(0).unwrap().to_string(),
                        hostname: line.split_whitespace().nth(1).unwrap().to_string(),
                        process: line.split_whitespace().nth(2).unwrap().to_string(),
                        message: line.split_whitespace().skip(3).collect::<Vec<_>>().join(" ").to_string()

                    
                    };
                    println!("____________________________________________________________________________________");
                    println!("Timestamp: {}", log_entry.timestamp);
                    println!("Hostname: {}", log_entry.hostname);
                    println!("Process: {}", log_entry.process);
                    println!("Message: {}", log_entry.message);
                }
                keyword_count += 1;
            }

        }
        println!("____________________________________________________________________________________");
        println!("Found {keyword_count} matches for {}", &args[2]);
    }




}