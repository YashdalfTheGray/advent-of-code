use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CubeSet {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
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

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
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
