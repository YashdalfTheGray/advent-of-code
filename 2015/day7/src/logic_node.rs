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

                if lhs_parts.len() == 2 {
                    let op = lhs_parts.remove(0);

                    let right_wire_or_value = lhs_parts.remove(0);
                    let right_value = match right_wire_or_value.parse() {
                        Ok(value) => Operation::NumberValue(value),
                        Err(_) => Operation::Wire(right_wire_or_value.to_string()),
                    };

                    Ok(Self::new(
                        output_wire.to_string(),
                        op.parse()?,
                        None,
                        Some(Box::new(Self::new(
                            right_wire_or_value.to_string(),
                            right_value,
                            None,
                            None,
                        ))),
                    ))
                } else {
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
                        op.parse()?,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_to_wire() {
        let input = "123 -> x";
        let node = input.parse::<LogicNode<u16>>().unwrap();

        println!("{:#?}", node);
        assert_eq!(node.output_wire, "x");
        assert_eq!(node.op, Operation::NumberValue(123));
    }

    #[test]
    fn test_parse_binary_operation_to_wire() {
        let input = "x AND y -> z";
        let node = input.parse::<LogicNode<u16>>().unwrap();

        println!("{:#?}", node);
        assert_eq!(node.output_wire, "z");
        assert_eq!(node.op, Operation::And);
        assert_eq!(node.left.unwrap().output_wire, "x");
        assert_eq!(node.right.unwrap().output_wire, "y");
    }
}
