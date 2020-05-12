//! # Grid
//!
//! `grid` represents the grid of the game

use std::fmt;
use crate::value::Value;

/// A binero grid is represented here
pub struct Grid {
    size: u8,
    matrix: Vec<Vec<Option<Value>>>,
}

impl Grid {
    /// Returns a binero grid with the given size
    ///
    /// # Arguments
    ///
    /// * `size` - an unsigned 8-bit integer that gives the size
    ///
    /// # Example
    ///
    /// ```
    /// use engine::grid::Grid;
    /// let mut grid = Grid::new(6);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `size` is an odd number
    pub fn new(size: u8) -> Grid {
        assert_eq!(size % 2, 0);
        Grid {
            size,
            matrix: vec![vec![None; size as usize]; size as usize],
        }
    }

    /// Puts a value in the grid and returns the previous one
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Value`
    ///
    /// # Example
    ///
    /// ```
    /// grid.put(4, 2, Value::First);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn put(&mut self, x_axis: u8, y_axis: u8, value: Value) -> Option<Value> {
        let result = self.get(x_axis, y_axis);
        let (i, j) = self.indexes(x_axis, y_axis);
        self.matrix[i][j] = Some(value);
        result
    }

    /// Returns a value from the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    ///
    /// # Example
    ///
    /// ```
    /// let value = grid.get(4, 2);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn get(&self, x_axis: u8, y_axis: u8) -> Option<Value> {
        let (i, j) = self.indexes(x_axis, y_axis);
        self.matrix[i][j]
    }

    /// Returns the indexes corresponding to the given x and y-axis
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    ///
    /// # Example
    ///
    /// ```
    /// let (i, j) = grid.get(4, 2);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    fn indexes(&self, x_axis: u8, y_axis: u8) -> (usize, usize) {
        let i = x_axis - 1;
        assert!(i < self.size);
        let j = y_axis - 1;
        assert!(j < self.size);
        (i as usize, j as usize)
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
