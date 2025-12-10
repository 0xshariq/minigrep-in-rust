use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Initialize configuration from command-line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", config.filename, e);
            return Err(Box::new(e));
        }
    };

    println!("File contents:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: <program> <query> <filename>");
        }

        // Access query from command-line arguments
        let query = &args[1];

        // Filename from command-line arguments
        let filename = &args[2];
        Ok(Config {
            query: query.to_string(),
            filename: filename.to_string(),
        })
    }
}
