use std::{env, error::Error, fs::{self, OpenOptions}, io::Write};
use chrono::prelude::*;
use chrono::Utc;
use edit::edit;
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;



static EDITOR_TEMPLATE: &str = "
# Please enter the message for your work log. Lines starting
# with '#' will be ignored, and an empty message aborts the notation";
 
pub fn run(config: Config, args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.len() == 1 {
        return default(config)
    }
    match args[1].as_str() {
        "head" => head(config.filepath),
        "backup" =>  backup(config, &args[2]),
        _ => append(config, &args[1]),
    }
}

fn append(config: Config, text: &String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(config.filepath)
    .unwrap();

    let entry = Entry {
        text: text.to_string(),
        timestamp: Utc::now(),
    };
    let j = serde_json::to_string(&entry)?;
    writeln!(&file, "{}", j)?;
    Ok(())
}

fn backup(config: Config, filename: &String) -> Result<(), Box<dyn Error>> {
    match fs::copy(config.filepath, filename) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)?,
    }
}

fn default(config: Config) -> Result<(), Box<dyn Error>> {
    let lines =  match edit(EDITOR_TEMPLATE)
    {
        Ok(entry)  =>  entry,
        Err(e) => return Err(e)?,
    };
    let entry = lines.lines()
        .filter(|l| !l.starts_with("#") & !l.is_empty())
        .map(|l| l.trim())
        .join(" ");
    if entry != "" {
        return append(config, &entry)
    }
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

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub text: String,
    #[serde(serialize_with = "serialize", deserialize_with  = "deserialize")]
    pub timestamp: DateTime<Utc>,
}

impl Entry {
    fn print(&self) -> String {
        let time = self.timestamp.format("%a %b %e %Y %T").to_string();
        format!("[{}] {}", time, self.text)
    }
}

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // let s = String::deserialize(deserializer)?;
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(
            Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)?
        )
    }

    Ok(Utc::now())
}

pub fn head(filename: String) -> Result<(), Box<dyn Error>> {

    let lines = fs::read_to_string(filename)?;
    for line in lines.lines() {
        let e: Option<Entry> = match serde_json::from_str (line) {
            Ok (e) => e,
            Err (_) => None
        };
        if let Some(e) = e {
            println!("{}", e.print())
        } else {
            println!("{}", line)
        }
    }
    Ok(())
}