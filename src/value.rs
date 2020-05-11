use std::fmt;

#[derive(Clone)]
pub enum Value {
    First,
    Second,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Value::First => '0',
            Value::Second => '1',
        };
        write!(f, "{}", printable)
    }
}
