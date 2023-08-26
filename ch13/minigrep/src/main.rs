use std::{env, process};

use minigrep::Config;

fn main() {
    let _args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // for x in env::args() {
    //     println!("{}", x);
    // }

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
