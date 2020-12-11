use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PassportParseError {
    failed_string: String,
}

impl PassportParseError {
    fn new(failed_string: String) -> PassportParseError {
        PassportParseError { failed_string }
    }
}

impl fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not parse \"{}\" into a PassportDetails struct.",
            self.failed_string
        )
    }
}

#[derive(Debug)]
pub struct PassportDetails {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl FromStr for PassportDetails {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();

        let mut byr = "".to_string();
        let mut cid = "".to_string();
        let mut ecl = "".to_string();
        let mut eyr = "".to_string();
        let mut hcl = "".to_string();
        let mut hgt = "".to_string();
        let mut iyr = "".to_string();
        let mut pid = "".to_string();
        let mut parse_error = false;

        for part in parts {
            let key_value_pair = part.split(':').collect::<Vec<&str>>();
            match key_value_pair[..] {
                ["byr", value, ..] => byr = value.to_string(),
                ["cid", value, ..] => cid = value.to_string(),
                ["ecl", value, ..] => ecl = value.to_string(),
                ["eyr", value, ..] => eyr = value.to_string(),
                ["hcl", value, ..] => hcl = value.to_string(),
                ["hgt", value, ..] => hgt = value.to_string(),
                ["iyr", value, ..] => iyr = value.to_string(),
                ["pid", value, ..] => pid = value.to_string(),
                [&_, _, ..] | [_] | [] => parse_error = true,
            }
        }

        if parse_error {
            return Err(PassportParseError::new(s.to_string()));
        }

        Ok(PassportDetails {
            byr,
            cid,
            ecl,
            eyr,
            hcl,
            hgt,
            iyr,
            pid,
        })
    }
}
