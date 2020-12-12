use std::fmt;
use std::str::FromStr;

use regex::Regex;

lazy_static! {
    static ref HEIGHT_MATCHER: Regex = Regex::new("([0-9]*)(cm|in)").unwrap();
    static ref HAIR_COLOR_MATCHER: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
    static ref PASSPORT_NUMBER_MATCHER: Regex = Regex::new("[0-9]{9}").unwrap();
    static ref VALID_EYE_COLORS: Vec<&'static str> =
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

#[derive(Debug, Clone)]
pub struct PassportParseError {
    failed_string: String,
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
    pub unparsed: Vec<String>,
}

impl PassportDetails {
    pub fn is_valid(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        let parsed_year = self.byr.parse::<u16>().unwrap_or(0);

        !self.byr.is_empty() && parsed_year >= 1920 && parsed_year <= 2002
    }

    fn is_iyr_valid(&self) -> bool {
        let parsed_year = self.iyr.parse::<u16>().unwrap_or(0);

        !self.iyr.is_empty() && parsed_year >= 2010 && parsed_year <= 2020
    }

    fn is_eyr_valid(&self) -> bool {
        let parsed_year = self.iyr.parse::<u16>().unwrap_or(0);

        !self.eyr.is_empty() && parsed_year >= 2020 && parsed_year <= 2030
    }

    fn is_hgt_valid(&self) -> bool {
        let captures = HEIGHT_MATCHER.captures(&(self.hgt)).unwrap();
        let height = (&captures[1]).parse::<u16>().unwrap_or(0);
        let unit = &captures[2];

        !self.hgt.is_empty()
            && match unit {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                "" => false,
                &_ => false,
            }
    }

    fn is_hcl_valid(&self) -> bool {
        !self.hcl.is_empty() && HAIR_COLOR_MATCHER.is_match(&(self.hcl))
    }

    fn is_ecl_valid(&self) -> bool {
        !self.ecl.is_empty() && VALID_EYE_COLORS.contains(&(&(self.ecl)[..]))
    }

    fn is_pid_valid(&self) -> bool {
        !self.pid.is_empty() && PASSPORT_NUMBER_MATCHER.is_match(&(self.pid))
    }
}

impl FromStr for PassportDetails {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        let mut byr = "".to_string();
        let mut cid = "".to_string();
        let mut ecl = "".to_string();
        let mut eyr = "".to_string();
        let mut hcl = "".to_string();
        let mut hgt = "".to_string();
        let mut iyr = "".to_string();
        let mut pid = "".to_string();
        let mut unparsed = vec![];

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
                [&_, _, ..] | [_] | [] => unparsed.push(part),
            }
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
            unparsed,
        })
    }
}
