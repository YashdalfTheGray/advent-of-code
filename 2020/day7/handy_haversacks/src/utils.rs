use std::error::Error;
use std::fs;

use regex::Regex;

lazy_static! {
    static ref BAG_DETAIL_REGEX: Regex = Regex::new(r#"(.*) bags contain (.*)\."#).unwrap();
}

pub fn read_into_collection(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents.lines().map(|s| s.trim().to_string()).collect();

    Ok(parsed)
}

pub fn parse_bags(bag_details: Vec<String>) -> Vec<(String, Vec<String>)> {
    bag_details
        .iter()
        .map(|s| {
            let captures = BAG_DETAIL_REGEX.captures(s).unwrap();
            let bags = &captures[2]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            (captures[1].to_string(), bags.clone())
        })
        .collect::<Vec<(String, Vec<String>)>>()
}
