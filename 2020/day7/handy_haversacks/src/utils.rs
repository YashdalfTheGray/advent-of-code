use std::error::Error;
use std::fs;

pub fn read_into_collection(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents.lines().map(|s| s.trim().to_string()).collect();

    Ok(parsed)
}
