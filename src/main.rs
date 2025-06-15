use std::env;
use std::process;
use colored::Colorize;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}: {}", "Problem parsing arguments".red(), err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.filename);
    
    // Reading the file 

    if let Err(e) = minigrep::run(config) {
        eprintln!("{}: {}", "Application Error".red(), e);
        process::exit(2);
    }
}



