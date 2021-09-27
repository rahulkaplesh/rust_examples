use std::env;
use std::process;

use command_line_example::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = command_line_example::run(config) {
        eprintln!("Application error is : {}", e);
        process::exit(1);
    };
}
