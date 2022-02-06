use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Operation<T> {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
    NumberValue(T),
    Wire(String),
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

    fn parse_command(s: &str) -> IResult<&str, (&str, &str)> {
        let (i, lhs) = take_until(" ")(s)?;
        let (i, _) = tag(" -> ")(i)?;
        let (i, rhs) = rest(i)?;
        Ok((i, (lhs, rhs)))
    }

    fn computation(s: &str) -> IResult<&str, (&str, &str, &str)> {
        let (i, lhs) = take_until(" ")(s)?;
        let (i, operation) = alt((
            tag("AND"),
            tag("OR"),
            tag("NOT"),
            tag("LSHIFT"),
            tag("RSHIFT"),
        ))(i)?;
        let (i, rhs) = rest(i)?;

        Ok((i, (lhs, operation, rhs)))
    }
}

impl FromStr for LogicNode<u16> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse_command(s).finish() {
            Ok((_, (lhs, rhs))) => {
                // let output_wire = rhs.to_string();
                // let (i, (op, left, right)) = Self::computation(lhs)?;
                // Ok(Self::new(output_wire, op, left, right))
                print!("lhs: {} | rhs: {}", lhs, rhs);
            }
            Err(_) => {}
        };
        Ok(Self::new(
            "a".to_string(),
            Operation::Identity(0),
            None,
            None,
        ))
    }
}
