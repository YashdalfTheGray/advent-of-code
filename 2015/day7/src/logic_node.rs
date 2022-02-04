use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Operation<T> {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
    Identity(T),
}

pub type ChildLogicNode<T> = Option<Box<LogicNode<T>>>;

#[derive(Debug)]
pub struct LogicNode<T> {
    pub output_wire: String,
    pub op: Operation<T>,
    left: ChildLogicNode<T>,
    right: ChildLogicNode<T>,
}

impl<T> LogicNode<T> {
    pub fn new(
        output_wire: String,
        op: Operation<T>,
        left: ChildLogicNode<T>,
        right: ChildLogicNode<T>,
    ) -> Self {
        Self {
            output_wire,
            op,
            left,
            right,
        }
    }
}

impl FromStr for LogicNode<u16> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<String> = s.split("->").map(|s| s.to_string()).collect();
        let mut _lhs = parts.pop().unwrap().trim().to_string();
        let output_wire = parts.pop().unwrap().trim().to_string();

        Ok(Self::new(output_wire, Operation::Identity(0), None, None))
    }
}
