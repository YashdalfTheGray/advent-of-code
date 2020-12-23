use std::fmt;

#[derive(Debug)]
pub enum Instructions {
    NOP,
    ACC,
    JMP,
    UNRECOGNIZED,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Instructions::ACC => "acc",
            Instructions::NOP => "nop",
            Instructions::JMP => "jmp",
            Instructions::UNRECOGNIZED => "<!unrecognized!>",
        };
        write!(f, "{}", output)
    }
}
