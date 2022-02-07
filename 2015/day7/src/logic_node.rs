use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum Operation<T> {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
    NumberValue(T),
    Wire(String),
}

impl FromStr for Operation<u16> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::And),
            "OR" => Ok(Operation::Or),
            "NOT" => Ok(Operation::Not),
            "LSHIFT" => Ok(Operation::Lshift),
            "RSHIFT" => Ok(Operation::Rshift),
            _ => panic!("Unknown operation: {}", s),
        }
    }
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
        let mut parts = s.split(" -> ");
        let lhs = parts.next().unwrap();
        let output_wire = parts.next().unwrap();

        match lhs.parse() {
            Ok(value) => Ok(Self::new(
                output_wire.to_string(),
                Operation::NumberValue(value),
                None,
                None,
            )),
            Err(_) => {
                let mut lhs_parts = lhs.split_whitespace().collect::<Vec<&str>>();
                let left_wire_or_value = lhs_parts.remove(0);
                let left_node = match left_wire_or_value.parse() {
                    Ok(value) => Operation::NumberValue(value),
                    Err(_) => Operation::Wire(left_wire_or_value.to_string()),
                };
                let op = lhs_parts.remove(0);
                let right_wire_or_value = lhs_parts.remove(0);
                let right_value = match right_wire_or_value.parse() {
                    Ok(value) => Operation::NumberValue(value),
                    Err(_) => Operation::Wire(right_wire_or_value.to_string()),
                };

                Ok(Self::new(
                    output_wire.to_string(),
                    match op {
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        "NOT" => Operation::Not,
                        "LSHIFT" => Operation::Lshift,
                        "RSHIFT" => Operation::Rshift,
                        _ => panic!("Unknown operation: {}", op),
                    },
                    Some(Box::new(Self::new(
                        left_wire_or_value.to_string(),
                        left_node,
                        None,
                        None,
                    ))),
                    Some(Box::new(Self::new(
                        right_wire_or_value.to_string(),
                        right_value,
                        None,
                        None,
                    ))),
                ))
            }
        }
    }
}
