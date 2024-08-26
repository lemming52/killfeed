mod head;

use std::{env, error::Error, fs::{OpenOptions}, io::Write};
use chrono::prelude::*;
use edit::edit;
use itertools::Itertools;


static EDITOR_TEMPLATE: &str = "
# Write your note above";
 
pub fn run(config: Config, args: &[String]) -> Result<(), Box<dyn Error>> {
    match args[1].as_str() {
        "head" => head::head(config.filepath),
        "-m" => append(config, &args[1]),
        _ => default(config)
    }
}

fn append(config: Config, entry: &String) -> Result<(), Box<dyn Error>> {
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

fn default(config: Config) -> Result<(), Box<dyn Error>> {
    let lines =  match edit(EDITOR_TEMPLATE)
    {
        Ok(entry)  =>  entry,
        Err(e) => return Err(e)?,
    };
    let entry = lines.lines()
        .filter(|l| !l.starts_with("#"))
        .map(|l| l.trim())
        .join(" ");
    append(config, &entry)
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