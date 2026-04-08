use std::fs;
use std::env;
use std::process;

fn main() {

    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Expected Error: No file path provided");
        eprintln!("Usage: cargo run -- <file path>");
        process::exit(1);

    }
   

    let sys_log = fs::read_to_string(&args[1])
    .expect("Failed to read file system");

    let count_log = sys_log.lines().count();
    println!("Total system log: {count_log}");

    let mut keyword_count = 0;
    
    if args.len() >= 3 {
        for line in sys_log.lines(){

            if line.contains(&args[2]){
               if keyword_count < 5 { 
                println!("{line}");
            }
            keyword_count +=1;

        }
        
    }
    println!("Found {keyword_count} matches for the keyword {}", &args[2]);
    
    }

}