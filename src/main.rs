mod days;

use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <day> [input_file]", args[0]);
        println!("Example: {} 1 input/day1.txt", args[0]);
        std::process::exit(1);
    }

    let day: u32 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Error: Day must be a number between 1 and 25");
            std::process::exit(1);
        }
    };

    if day < 1 || day > 25 {
        eprintln!("Error: Day must be between 1 and 25");
        std::process::exit(1);
    }

    let input = if args.len() > 2 {
        fs::read(&args[2]).unwrap_or_else(|err| {
            eprintln!("Error reading file {}: {}", args[2], err);
            std::process::exit(1);
        })
    } else {
        eprintln!("Error: No input file specified");
        std::process::exit(1);
    };

    let start = Instant::now();
    match days::run_day(day, &input) {
        Ok(_) => {
            let elapsed = start.elapsed();
            println!("\nTime: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
