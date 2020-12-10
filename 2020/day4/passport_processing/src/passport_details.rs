use std::fmt;

#[derive(Debug, Clone)]
pub struct PassportParseError {
  failedString: String,
}

impl fmt::Display for PassportParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Could not parse {} into a PassportDetails struct.",
      self.failedString
    )
  }
}

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
