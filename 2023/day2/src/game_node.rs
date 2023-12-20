use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeenIntValue {
    pub value: u64,
    pub seen_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CubeSet {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl CubeSet {
    pub fn new() -> CubeSet {
        CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct GameNode {
    pub id: String,
    pub number_of_reveals: u64,
    pub highest_cubeset: CubeSet,
    pub reveals: Vec<CubeSet>,
}

impl GameNode {
    pub fn blank() -> Self {
        Self {
            id: "".to_owned(),
            number_of_reveals: 0,
            highest_cubeset: CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            },
            reveals: [].to_vec(),
        }
    }

    pub fn new(id: String, number_of_reveals: u64) -> Self {
        Self {
            id,
            number_of_reveals,
            highest_cubeset: CubeSet {
                red: 0,
                green: 0,
                blue: 0,
            },
            reveals: [].to_vec(),
        }
    }
}

impl FromStr for GameNode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            pub static ref GAME_DETECTOR: Regex = Regex::new(r"(\d+)$").unwrap();
            pub static ref CUBE_DETECTOR: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
        }

        let parts: Vec<&str> = s.split(": ").collect();
        println!("{:#?}", parts);

        let game_matches = GAME_DETECTOR.captures(parts[0]).unwrap();
        let game_number = game_matches.get(1).unwrap().as_str();
        println!("id = {:#?}", game_number);

        let reveal_strings = parts[1].split("; ").collect::<Vec<&str>>();
        println!("{:#?}", reveal_strings);

        let reveals = reveal_strings
            .iter()
            .map(|rs| {
                return rs.split(", ").fold(CubeSet::new(), |mut acc, p| {
                    let cube_matches = CUBE_DETECTOR.captures(p).unwrap();
                    let cube_number = cube_matches.get(1).unwrap().as_str();
                    let cube_color = cube_matches.get(2).unwrap().as_str();
                    println!("{:#?}", cube_number);
                    println!("{:#?}", cube_color);

                    match cube_color {
                        "red" => acc.red += cube_number.parse::<u64>().unwrap(),
                        "green" => acc.green += cube_number.parse::<u64>().unwrap(),
                        "blue" => acc.blue += cube_number.parse::<u64>().unwrap(),
                        _ => panic!("Invalid cube color"),
                    }

                    return acc;
                });
            })
            .collect::<Vec<CubeSet>>();

        println!("{:#?}", reveals);

        Ok(Self::blank())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_string() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse::<GameNode>()
            .unwrap();
        assert_eq!(game.number_of_reveals, 0)
    }
}
