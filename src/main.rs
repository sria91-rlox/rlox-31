use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => {
            if let Err(e) = run_file(&args[1]) {
                println!("Error running file: {}", e);
                process::exit(1);
            }
        }
        _ => println!("Usage: rlox [script]"),
    }
}

fn run_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    println!("\nRunning file {}\n", file_name);
    let contents = fs::read_to_string(file_name)?;
    println!("Contents: {}", contents);
    Ok(())
}

fn run_repl() {
    let stdin = io::stdin();
    let input = &mut String::new();
    loop {
        print!("> ");
        input.clear();
        match stdin.read_line(input) {
            Ok(_) => println!("{}", input),
            Err(e) => println!("Error: {}", e),
        }
    }
}
