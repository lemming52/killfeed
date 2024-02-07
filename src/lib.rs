use std::{env, error::Error, fs::{OpenOptions}, io::Write};
use chrono::prelude::*;

pub fn run(config: Config, entry: &String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config.filepath)
        .unwrap();

    let time: DateTime<Local> = Local::now();
    let time = time.format("%a %b %e %Y %T").to_string();
    writeln!(&file, "[{}] {}", time, entry)?;
    Ok(())
}

pub struct Config {
    pub filepath: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let filepath = env::var("KILLFEED_FILE").unwrap_or_else(|_e| {
            let mut home = env::var("HOME").unwrap();
            home.push_str("/.killfeed");
            home
        });
        Ok(Config {filepath})
    }
}