use std::fmt;
use std::str::FromStr;

use regex::Regex;

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

pub struct CodeLine {
    pub instruction: String,
    pub offset: i32,
    pub executed: bool,
}

impl FromStr for CodeLine {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        Ok(CodeLine {
            instruction: parts[0].clone(),
            offset: parts[1].parse::<i32>().unwrap(),
            executed: false,
        })
    }
}
