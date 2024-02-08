use std::{env, process};

use killfeed::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("problem loading config: {}", err);
        process::exit(1);
    });



    if let Err(e) = killfeed::run(config, &args) {
        eprintln!("application failure: {}", e);
        process::exit(1);
    };
}

