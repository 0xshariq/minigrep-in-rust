use cli_tool::Config;
use cli_tool::run;
use std::env;
use std::process;

fn main() {
    // Initialize configuration from command-line arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("Filename: {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
