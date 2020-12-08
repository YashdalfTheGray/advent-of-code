extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs;

pub struct Constraint {
    min: usize,
    max: usize,
    letter: char,
}

impl Constraint {
    pub fn new(min: usize, max: usize, letter: char) -> Constraint {
        Constraint { min, max, letter }
    }

    pub fn validate_range(&self, password: String) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let letter_only_count = chars.iter().filter(|&c| *c == self.letter).count();

        letter_only_count >= self.min && letter_only_count <= self.max
    }
}

pub fn read_and_process(filename: &str) -> Result<usize, Box<dyn Error>> {
    let input_parser = Regex::new(r"^(\d{1,})-(\d{1,}) ([a-z]{1}): ([a-z]+)$").unwrap();

    let contents = fs::read_to_string(filename)?;

    let matched: Vec<bool> = contents
        .lines()
        .map(|line| {
            let captures = input_parser.captures(line).unwrap();

            Constraint::new(
                parse_to_usize(&captures[1]),
                parse_to_usize(&captures[2]),
                parse_to_char(&captures[3]),
            )
            .validate_range((&captures[4]).to_string())
        })
        .collect();

    Ok(matched.iter().filter(|&n| *n).count())
}

pub fn parse_to_usize(int_as_string: &str) -> usize {
    int_as_string.parse::<usize>().unwrap()
}

pub fn parse_to_char(char_as_string: &str) -> char {
    let char_vector: Vec<char> = char_as_string.chars().collect();
    char_vector[0]
}
