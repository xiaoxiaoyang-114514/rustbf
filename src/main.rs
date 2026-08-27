use std::{ env, fs };
use rustbf::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help();
        return;
    }
    if args.len() < 2 {
        eprintln!("Too few arguments. Expected 1 but {}", args.len()-1);
        std::process::exit(1);
    }
    let src = match fs::read_to_string(args[1].clone()) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
    };
    let output = if args.len() == 4 && args[2] == "--with-input" {
        run_bf(&src, Some(parse_input(&args[3])), true)
    } else if args.len() == 2 {
        run_bf(&src, None, true)
    } else if args.len() == 3 && args[2] == "--with-input" {
        eprintln!("Too few arguments. Expected 2 (--with-input <input>) but {}", args.len()-1);
        std::process::exit(1);
    } else {
        eprintln!("Too many arguments. Expected 1 or 2 but {}", args.len()-1);
        std::process::exit(1);
    };
    match output {
        Ok(Some(a)) => {
            for i in a{
                print!("{}",i);
            }
        },
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        },
        _ => {},
    }
}

fn parse_input(s: &str) -> Vec<char> {
    s.split(',')
        .filter_map(|c| c.trim().trim_matches('\'').chars().next())
        .collect()
}

fn print_help() {
    println!("Usage: rustbf <file> [--with-input <input>]");
    println!();
    println!("Run a Brainfuck source file.");
    println!();
    println!("Arguments:");
    println!("  <file>              Path to the .bf source file");
    println!("  --with-input <in>   Provide input as a list of chars, e.g. '1','2','3'");
    println!("  -h, --help          Show this help message");
    println!();
    println!("Without --with-input, the program reads input from stdin.");
}
