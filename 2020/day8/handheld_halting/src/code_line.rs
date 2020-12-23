use core::panic;
use std::fmt;
use std::str::FromStr;

use regex::Regex;

use crate::enums::Instructions;

lazy_static! {
    static ref CODE_LINE_REGEX: Regex = Regex::new(r#"^(nop|jmp|acc) ((\+|-)([0-9]*))$"#).unwrap();
}

#[derive(Debug, Clone)]
pub struct CodeParseError {
    failed_string: String,
}

impl fmt::Display for CodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not identify \"{}\" as something that can be a line of code.",
            self.failed_string
        )
    }
}

#[derive(Debug)]
pub struct CodeLine {
    pub instruction: Instructions,
    pub offset: i32,
}

impl FromStr for CodeLine {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let instruction = match &parts[0][..] {
            "nop" => Instructions::NOP,
            "acc" => Instructions::ACC,
            "jmp" => Instructions::JMP,
            "" | &_ => {
                panic!("Unrecognized instruction found!{}", parts[0])
            }
        };

        Ok(CodeLine {
            instruction,
            offset: parts[1].parse::<i32>().unwrap(),
        })
    }
}

impl fmt::Display for CodeLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.instruction, self.offset)
    }
}
