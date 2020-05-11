use std::fmt;
use crate::value::Value;

pub struct Grid {
    size: u8,
    matrix: Vec<Vec<Option<Value>>>,
}

impl Grid {
    pub fn new(size: u8) -> Grid {
        Grid {
            size,
            matrix: vec![vec![None; size as usize]; size as usize],
        }
    }

    pub fn set(&mut self, abs: u8, ord: u8, value: Value) {
        let i = abs - 1;
        assert!(i < self.size);
        let j = ord - 1;
        assert!(j < self.size);
        self.matrix[i as usize][j as usize] = Some(value);
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_line = || {
            let mut result = "\n".to_owned();
            for _ in 0..self.size {
                result.push_str("----");
            }
            result.push_str("-");
            result
        };

        let display_cell = |i| {
            let mut result = "\n|".to_owned();
            for j in 0..self.size {
                result.push_str(" ");
                match &self.matrix[i as usize][j as usize] {
                    Some(n) => result.push_str(format!("{}", n).as_str()),
                    None => result.push_str(" "),
                }
                result.push_str(" |");
            }
            result
        };

        let mut grid = "".to_owned();
        for i in 0..self.size {
            grid.push_str(&display_line());
            grid.push_str(&display_cell(i));
        }
        grid.push_str(&display_line());
        write!(f, "{}", grid)
    }
}
