use std::error::Error;
use std::fs;

use crate::passport_details::PassportDetails;

pub fn read_into_collection(filename: &str) -> Result<Vec<PassportDetails>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents
        .split("\n\n")
        .map(|s| s.to_string())
        .map(|s| s.replacen("\n", " ", 100))
        .map(|s| s.trim().to_string())
        .map(|s| s.parse::<PassportDetails>().unwrap())
        .collect();

    Ok(parsed)
}
