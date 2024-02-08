use std::{error::Error, fs};

pub fn head(filename: String) -> Result<(), Box<dyn Error>> {

    let lines = fs::read_to_string(filename)?;
    for line in lines.lines() {
        println!("{}", line)
    }
    Ok(())
}