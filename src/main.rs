use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => println!("Interpreting..."),
        2 => run_file(&args[1]).unwrap_or_else(|err| {
            println!("Error running file: {}", err);
            process::exit(1);
        }),
        _ => println!("Usage: rlox [script]")
    }
}

fn run_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    println!("\nRunning file {}\n", file_name);
    let contents = fs::read_to_string(file_name)?;
    println!("Contents: {}", contents);
    Ok(())
}
