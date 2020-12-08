extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs;

pub struct Constraint {
    min: usize,
    max: usize,
    letter: char,
}

pub type Metrics = (usize, usize);

impl Constraint {
    pub fn new(min: usize, max: usize, letter: char) -> Constraint {
        Constraint { min, max, letter }
    }

    pub fn validate_range(&self, password: String) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let letter_only_count = chars.iter().filter(|&c| *c == self.letter).count();

        letter_only_count >= self.min && letter_only_count <= self.max
    }

    pub fn validate_location(&self, password: String) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let valid_bounds = (self.min - 1) < password.len() && (self.max - 1) < password.len();

        if valid_bounds {
            let first_match = chars[self.min - 1] == self.letter;
            let last_match = chars[self.max - 1] == self.letter;

            xor(first_match, last_match)
        } else {
            false
        }
    }
}

pub fn read_and_process(filename: &str) -> Result<Metrics, Box<dyn Error>> {
    let input_parser = Regex::new(r"^(\d{1,})-(\d{1,}) ([a-z]{1}): ([a-z]+)$").unwrap();

    let contents = fs::read_to_string(filename)?;

    let range_matched = process_and_count(&contents, |line| {
        let captures = input_parser.captures(line).unwrap();

        Constraint::new(
            parse_to_usize(&captures[1]),
            parse_to_usize(&captures[2]),
            parse_to_char(&captures[3]),
        )
        .validate_range((&captures[4]).to_string())
    });

    let location_matched = process_and_count(&contents, |line| {
        let captures = input_parser.captures(line).unwrap();

        Constraint::new(
            parse_to_usize(&captures[1]),
            parse_to_usize(&captures[2]),
            parse_to_char(&captures[3]),
        )
        .validate_location((&captures[4]).to_string())
    });

    Ok((range_matched, location_matched))
}

fn parse_to_usize(int_as_string: &str) -> usize {
    int_as_string.parse::<usize>().unwrap()
}

fn parse_to_char(char_as_string: &str) -> char {
    let char_vector: Vec<char> = char_as_string.chars().collect();
    char_vector[0]
}

fn process_and_count<F>(content: &str, matcher: F) -> usize
where
    F: FnMut(&str) -> bool,
{
    content
        .lines()
        .map(matcher)
        .collect::<Vec<bool>>()
        .iter()
        .filter(|&n| *n)
        .count()
}

fn xor(a: bool, b: bool) -> bool {
    (a || b) && !(a && b)
}
