use std::fmt;

use crate::code_line::CodeLine;

pub struct CodeProcessor {
    acc: i32,
    pc: u32,
}

impl CodeProcessor {
    pub fn new() -> CodeProcessor {
        CodeProcessor { acc: 0, pc: 0 }
    }

    pub fn execute(&self, code: Vec<CodeLine>) {
        for line in code {
            println!("{:?}", line);
        }
    }
}

impl fmt::Display for CodeProcessor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Accumulator = {} | Program Counter = {}",
            self.acc, self.pc
        )
    }
}
