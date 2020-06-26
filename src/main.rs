use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => println!("Interpreting..."),
        2 => run_file(&args[1]),
        _ => println!("Usage: rlox [script]")
    }
}

fn run_file(file_name: &String) {
    println!("=================\nRunning file {}\n=================", file_name);
    let contents = fs::read_to_string(file_name)
        .expect("Error reading the file");
    println!("Contents: {}", contents);
}
