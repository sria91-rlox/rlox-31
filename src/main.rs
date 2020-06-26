use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => println!("Interpreting..."),
        2 => println!("Running file {:?}", &args[1]),
        _ => println!("Usage: rlox [script]")
    }
}


