use std::env;
use std::process;

use command_line_example::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = command_line_example::run(config) {
        println!("Application error is : {}", e);
        process::exit(1);
    };
}
