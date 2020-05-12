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

    /// Returns the size of the grid
    pub fn size(&self) -> u8 {
        self.size
    }

    /// Returns wether or not a value can be put in the grid
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
    /// let cp = grid.can_put(2, 2, Value::Second);
    /// ```
    pub fn can_put(&self, x_axis: u8, y_axis: u8, value: Value) -> bool {
        let (i, j) = self.indexes(x_axis, y_axis);
        self.can_accept(Axis::X, i, value) && self.can_accept(Axis::Y, j, value)
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

    /// Returns wether or not the grid can accept a value in the nth row or column
    ///
    /// Arguments
    ///
    /// * `axis` - an x or y-axis
    /// * `index` - the index of the row or column
    /// * `value` - a `Value`
    ///
    /// # Example
    ///
    /// ```
    /// let ca = grid.can_accept(Axis::X, 1, Value::First);
    /// ```
    fn can_accept(&self, axis: Axis, index: usize, value: Value) -> bool {
        let mut number: u8 = 1;

        let mut incr_number = |i: usize, j| {
            if let Some(val) = self.matrix[i][j] {
                if val == value {
                    number += 1;
                }
            }
        };

        match axis {
            Axis::X => {
                for i in 0..self.size {
                    incr_number(index, i as usize);
                }
            },
            Axis::Y => {
                for i in 0..self.size {
                    incr_number(i as usize, index);
                }
            },
        }
        number <= self.size / 2
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

enum Axis {
    X,
    Y,
}
