use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub fn read_contents(filename: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let parsed: Vec<u32> = contents
        .lines()
        .map(|ns| ns.parse::<u32>().unwrap())
        .collect();

    Ok(parsed)
}

pub fn load_into_set(numbers: &[u32]) -> HashSet<u32> {
    let mut result = HashSet::new();

    for n in numbers {
        result.insert(*n);
    }

    result
}
