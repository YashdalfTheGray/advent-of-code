use std::fs;
use std::io::prelude::*;
use std::{error::Error, fs::File, path::Path};

use regex::Regex;

lazy_static! {
    static ref BAG_RULE_REGEX: Regex = Regex::new(r#"(.*) bags contain (.*)\."#).unwrap();
    static ref BAG_DETAIL_REGEX: Regex = Regex::new(r#"(\d*)\s([a-z ]*)\sbags?"#).unwrap();
}

pub fn read_into_collection(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = contents.lines().map(|s| s.trim().to_string()).collect();

    Ok(parsed)
}

pub fn parse_bag_rules(bag_rules: Vec<String>) -> Vec<(String, Vec<String>)> {
    bag_rules
        .iter()
        .map(|s| {
            let captures = BAG_RULE_REGEX.captures(s).unwrap();
            let bags = &captures[2]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (captures[1].to_string(), bags.clone())
        })
        .collect::<Vec<(String, Vec<String>)>>()
}

pub fn parse_bag_details(bag_details: Vec<String>) -> Vec<(u32, String)> {
    bag_details
        .iter()
        .map(|s| {
            let details = BAG_DETAIL_REGEX.captures(s).unwrap();

            let number = details[1].parse::<u32>().unwrap_or(0);

            (number, details[2].to_string())
        })
        .collect::<Vec<(u32, String)>>()
}

pub fn write_string_to_file(s: String, path: String) {
    let path = Path::new(&path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
