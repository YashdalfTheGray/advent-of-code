use std::fs;
use std::{env, error::Error};

pub fn read_into_collection(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents.lines().map(|s| s.trim().to_string()).collect();

    Ok(parsed)
}

pub fn get_filename() -> Option<String> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 && !args[1].is_empty() {
        Some(args[1].clone())
    } else {
        None
    }
}
