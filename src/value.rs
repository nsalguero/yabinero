//! # Value
//!
//! `value` describes the values that can be put in the grid of the game

use std::fmt;

/// The two possible values that can be put in a binero grid
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Value {
    First,
    Second,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Value::First => FIRST_CHAR,
            Value::Second => SECOND_CHAR,
        };
        write!(f, "{}", printable)
    }
}

/// Returns a `Value` if the given character is correct and `None` otherwise
///
/// # Arguments
///
/// * `c` - a character
pub fn value(c: char) -> Option<Value> {
    match c {
        FIRST_CHAR => Some(Value::First),
        SECOND_CHAR => Some(Value::Second),
        _ => None,
    }
}

const FIRST_CHAR: char = '0';
const SECOND_CHAR: char = '1';
