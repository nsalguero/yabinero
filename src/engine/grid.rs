//! # Grid
//!
//! `grid` represents the grid of the game

use std::fmt;
use crate::enums::{Size, Value};

/// A binero grid is represented here
pub struct Grid {
    size: Size,
    matrix: Vec<Vec<Option<Value>>>,
    empty_values: u16,
}

impl Grid {
    /// Returns a binero grid with the given size
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    pub fn new(size: Size) -> Grid {
        let size_u8 = size.as_u8();
        Grid {
            size,
            matrix: vec![vec![None; size_u8 as usize]; size_u8 as usize],
            empty_values: (size_u8 as u16).pow(2),
        }
    }

    /// Returns the size of the grid
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns wheter or not the grid is full
    pub fn is_full(&self) -> bool {
        self.empty_values == 0
    }

    /// Returns the number of empty boxes in the grid
    pub fn empty_values(&self) -> u16 {
        self.empty_values
    }

    /// Returns whether or not a value must be put in the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Value`
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn must_put(&self, x_axis: u8, y_axis: u8, value: Value) -> bool {
        let size = self.size.as_u8();
        assert!(x_axis < size && y_axis < size);
        let the_other_value = value.the_other();
        !self.can_put(x_axis, y_axis, the_other_value)
    }

    /// Returns whether or not a value can be put in the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Value`
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn can_put(&self, x_axis: u8, y_axis: u8, value: Value) -> bool {
        let size = self.size.as_u8();
        assert!(x_axis < size && y_axis < size);
        self.can_accept(Axis::X, x_axis, y_axis, value) &&
            self.can_accept(Axis::Y, x_axis, y_axis, value)
    }

    /// Puts a value in the grid and returns the previous one
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Option<Value>`
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn put(&mut self, x_axis: u8, y_axis: u8, value: Option<Value>) -> Option<Value> {
        let size = self.size.as_u8();
        assert!(x_axis < size && y_axis < size);
        let result = self.get(x_axis, y_axis);
        self.matrix[x_axis as usize][y_axis as usize] = value;
        if value.is_some() && result.is_none() {
            self.empty_values -= 1;
        } else if value.is_none() && result.is_some() {
            self.empty_values += 1;
        }
        result
    }

    /// Returns a value from the grid
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    ///
    /// # Panics
    ///
    /// Panics if `x_axis` or `y_axis` are greater than the size of the grid
    pub fn get(&self, x_axis: u8, y_axis: u8) -> Option<Value> {
        let size = self.size.as_u8();
        assert!(x_axis < size && y_axis < size);
        self.matrix[x_axis as usize][y_axis as usize]
    }

    /// Returns whether or not the grid can accept a value in the nth row or column
    ///
    /// Arguments
    ///
    /// * `axis` - the axis we are working on
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `value` - a `Value`
    fn can_accept(&self, axis: Axis, x_axis: u8, y_axis: u8, value: Value) -> bool {
        let mut total_number: u8 = 0;
        let mut adjacent_number: u8 = 0;

        let index_in_changing_axis = match axis {
            Axis::X => y_axis,
            Axis::Y => x_axis,
        };

        for k in 0..self.size.as_u8() {
            if k == index_in_changing_axis {
                total_number += 1;
                adjacent_number += 1;
            } else {
                let mut manage_numbers = |i: u8, j: u8| {
                    let v = self.matrix[i as usize][j as usize];
                    match v {
                        Some(val) => {
                            if val == value {
                                total_number += 1;
                                adjacent_number += 1;
                            } else {
                                adjacent_number = 0;
                            }
                        },
                        None => adjacent_number = 0,
                    }
                };

                match axis {
                    Axis::X => manage_numbers(x_axis, k),
                    Axis::Y => manage_numbers(k, y_axis),
                };
            }
            if self.violate_constraint_max_per_row_or_column(total_number) ||
                self.violate_constraint_max_adjacent_in_row_or_column(adjacent_number) {
                return false;
            }
        }
        true
    }

    /// Returns whether or not the grid violates the constraint saying a row or a column must
    /// contain as much of a value as of the other
    ///
    /// # Arguments
    ///
    /// * `number` - the number of a value in a row or a column
    fn violate_constraint_max_per_row_or_column(&self, number: u8) -> bool {
        number > self.size.as_u8() / 2
    }

    /// Returns whether or not the grid violates the constraint saying a row or a column cannot
    /// contain more than twice the same value side by side
    ///
    /// # Arguments
    ///
    /// * `number` - the number of the same value side by side in a row or a column
    fn violate_constraint_max_adjacent_in_row_or_column(&self, number: u8) -> bool {
        number > 2
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_line = || {
            let mut result = "\n".to_owned();
            for _ in 0..self.size.as_u8() {
                result.push_str("----");
            }
            result.push_str("-");
            result
        };

        let display_cell = |i| {
            let mut result = "\n|".to_owned();
            for j in 0..self.size.as_u8() {
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
        for i in 0..self.size.as_u8() {
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
