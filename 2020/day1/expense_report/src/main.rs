use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let contents = read_contents("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let set_of_numbers = load_into_set(&contents);

    let mut found_number = 0;
    for n in contents {
        if set_of_numbers.contains(&(2020 - n)) {
            found_number = n;
            break;
        }
    }

    println!(
        "Found {} and {} in the given list that add up to 2020.\nTheir product is {}",
        found_number,
        2020 - found_number,
        found_number * (2020 - found_number)
    );
}

fn read_contents(filename: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let parsed: Vec<u32> = contents
        .lines()
        .map(|ns| ns.parse::<u32>().unwrap())
        .collect();

    Ok(parsed)
}

fn load_into_set(numbers: &[u32]) -> HashSet<u32> {
    let mut result = HashSet::new();

    for n in numbers {
        result.insert(*n);
    }

    result
}
