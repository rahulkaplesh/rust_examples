use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_configs(&args);

    println!("Query {} to be executed on {}", config.query, config.filename);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_configs(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config{ query, filename };
}
