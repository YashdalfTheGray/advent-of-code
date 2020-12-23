use std::{collections::HashSet, fmt};

use crate::code_line::CodeLine;
use crate::code_line::Instructions;

pub struct CodeProcessor {
    acc: i32,
    pc: u32,
}

impl CodeProcessor {
    pub fn new() -> CodeProcessor {
        CodeProcessor { acc: 0, pc: 0 }
    }

    pub fn execute(&mut self, code: Vec<CodeLine>) {
        let mut visited_set: HashSet<u32> = HashSet::new();

        loop {
            let current_line = &code[self.pc as usize];

            if (self.pc as usize) > (code.len() - 1) {
                println!("{}", self);
                break;
            }

            if visited_set.contains(&self.pc) {
                println!(
                    "Loop found in the code. The accumulator is {} and the program counter is at {}.",
                    self.acc, self.pc
                );
                break;
            } else {
                visited_set.insert(self.pc);
                match current_line.instruction {
                    Instructions::ACC => {
                        self.acc += current_line.offset;
                        self.pc += 1;
                    }
                    Instructions::JMP => {
                        if current_line.offset < 0 {
                            self.pc -= current_line.offset.abs() as u32;
                        } else {
                            self.pc += current_line.offset as u32;
                        }
                    }
                    Instructions::NOP => {
                        self.pc += 1;
                    }
                    Instructions::UNRECOGNIZED => {
                        panic!(
                            "An unrecognized instruction was encountered\nState {}\n{:?}",
                            self, current_line
                        );
                    }
                }
            }
        }

        println!(
            "The program terminated normally. Accumulator value is at {}",
            self.acc
        );
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
