use std::fmt;

#[derive(Debug)]
pub enum Instructions {
    NOP,
    ACC,
    JMP,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Instructions::ACC => "acc",
            Instructions::NOP => "nop",
            Instructions::JMP => "jmp",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum ExitResult {
    Exited,
    InfiniteLoop,
}

impl fmt::Display for ExitResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            ExitResult::Exited => "exited",
            ExitResult::InfiniteLoop => "InfiniteLoop",
        };
        write!(f, "{}", output)
    }
}
