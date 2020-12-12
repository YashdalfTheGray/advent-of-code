use std::fs;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InsufficientStepsError {
    max: u16,
    seq: String,
}

impl fmt::Display for InsufficientStepsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The sequence {} does not have enough steps to narrow down the range [0, {}] down to a single number.",
            self.seq,
            self.max
        )
    }
}

pub fn read_into_collection(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents.lines().map(|s| s.to_string()).collect();

    Ok(parsed)
}

pub fn find_position(
    max: u16,
    sequence: String,
    dir_low: char,
    dir_high: char,
) -> Result<u16, InsufficientStepsError> {
    let mut range = (0u16, max);

    for c in sequence.chars() {
        match c {
            c if (c == dir_low) => range = (range.0, range.1 - get_step(range.0, range.1)),
            c if (c == dir_high) => range = (range.0 + get_step(range.0, range.1), range.1),
            _ => range = (range.0, range.1),
        }
    }

    if range.1 - range.0 != 0 {
        return Err(InsufficientStepsError { max, seq: sequence });
    }

    Ok(range.0)
}

fn get_step(low: u16, high: u16) -> u16 {
    let result = (high as f64 - low as f64) / 2f64;

    if result > ((result as u16) as f64) {
        (result as u16) + 1
    } else {
        result as u16
    }
}
