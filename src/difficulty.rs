use std::fmt;
use tr::tr;

/// The four possible levels of difficulty
#[derive(Clone, Copy)]
pub enum Difficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Difficulty::Beginner => tr!("Beginner"),
            Difficulty::Easy => tr!("Easy"),
            Difficulty::Medium => tr!("Medium"),
            Difficulty::Hard => tr!("Hard"),
        };
        write!(f, "{}", printable)
    }
}
