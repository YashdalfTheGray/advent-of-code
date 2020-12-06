use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let contents = read_contents("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });
    for (index, n) in contents.iter().enumerate() {
        println!("{}: {}", index + 1, n);
    }
}

fn read_contents(filename: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let parsed: Vec<u32> = contents
        .lines()
        .map(|ns| ns.parse::<u32>().unwrap())
        .collect();

    Ok(parsed)
}
