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
        let mut iter = s.split("->");

        Ok(Self::new(
            "a".to_string(),
            Operation::Identity(0),
            None,
            None,
        ))
    }
}
