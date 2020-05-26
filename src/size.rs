//! # Size
//!
//! `size` describes the possible sizes of a binero

use std::fmt;

/// The possible sizes of a binero
pub enum Size {
    Side6,
    Side8,
    Side10,
    Side12,
    Side14,
    Side16,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.to_u8();
        let printable = format!("{}x{}", size, size);
        write!(f, "{}", printable)
    }
}

impl Size {
    /// Returns the size as an unsigned 8-bit integer
    pub fn to_u8(&self) -> u8 {
        match *self {
            Size::Side6 => 6,
            Size::Side8 => 8,
            Size::Side10 => 10,
            Size::Side12 => 12,
            Size::Side14 => 14,
            Size::Side16 => 16,
        }
    }
}
