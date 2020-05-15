//! # History
//!
//! `history` manages the history of the game

use std::fmt;
use crate::value::Value;

/// The history of the game is represented here
pub struct History {
    items: Vec<Item>,
    choices: Vec<usize>,
    current_item: Option<usize>,
}

impl History {
    /// Returns an empty history of a game
    pub fn new() -> History {
        History {
            items: Vec::new(),
            choices: Vec::new(),
            current_item: None,
        }
    }

    /// Pushes an action in the history
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `old_value` - the previous value
    /// * `new value` - the new value
    /// * `is_a_choice` - whether or not the new value is a choice
    pub fn push(&mut self, x_axis: u8, y_axis: u8, old_value: Option<Value>, new_value: Option<Value>, is_a_choice: bool) {
        self.pop_all_after_current_item();
        let next_item = self.items.len();
        self.current_item = Some(next_item);
        if is_a_choice {
            self.choices.push(next_item);
        }
        let item = Item::new(x_axis, y_axis, old_value, new_value);
        self.items.push(item);
    }

    /// Returns the index of the latest choice in the history
    pub fn latest_choice(&mut self) -> Option<usize> {
        self.choices.pop()
    }

    /// Returns the index of the current item of the history
    pub fn current_item(&self) -> Option<usize> {
        self.current_item
    }

    /// Returns whether or not undoing the current action is possible
    pub fn is_undo_possible(&self) -> bool {
        self.current_item.is_some()
    }

    /// Returns whether or not redoing the next action is possible because it was undone before
    pub fn is_redo_possible(&self) -> bool {
        self.next_item().is_some()
    }

    /// Cancels the current action and returns it
    ///
    /// # Panics
    ///
    /// Panics if no possible undo
    pub fn undo(&mut self) -> &Item {
        let curr = self.current_item.unwrap();
        if curr == 0 {
            self.current_item = None;
        } else {
            self.current_item = Some(curr - 1);
        }
        self.items.get(curr).unwrap()
    }

    /// Replays the next action that was previously undone and returns it
    ///
    /// # Panics
    ///
    /// Panics if no possible redo
    pub fn redo(&mut self) -> &Item {
        let next = self.next_item().unwrap();
        self.current_item = Some(next);
        self.items.get(next).unwrap()
    }

    /// Clears the history
    pub fn clear(&mut self) {
        self.current_item = None;
        self.pop_all_after_current_item();
    }

    /// Returns the next item of the history if it exists
    fn next_item(&self) -> Option<usize> {
        let current = self.current_item;
        let next = match current {
            Some(curr) => curr + 1,
            None => 0,
        };
        if self.items.get(next).is_none() {
            None
        } else {
            Some(next)
        }
    }

    /// Clears the history after the current item
    fn pop_all_after_current_item(&mut self) {
        if let Some(next) = self.next_item() {
            let size = self.items.len();
            for i in next..size {
                self.items.pop();
                self.choices.retain(|&item| item != i);
            }
        }
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut history = "\n".to_owned();
        for i in 0..self.items.len() {
            history.push_str(&format!("{}", self.items.get(i).unwrap()));
            if self.choices.contains(&i) {
                history.push_str(" *");
            }
            if let Some(current) = self.current_item {
                if current == i {
                    history.push_str(" <-");
                }
            }
            history.push_str("\n");
        }
        write!(f, "{}", history)
    }
}

/// An item of the history of the game is represented here
pub struct Item {
    x_axis: u8,
    y_axis: u8,
    old_value: Option<Value>,
    new_value: Option<Value>,
}

impl Item {
    /// Returns a new item of the history
    ///
    /// # Arguments
    ///
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `old_value` - the previous value
    /// * `new value` - the new value
    fn new(x_axis: u8, y_axis: u8, old_value: Option<Value>, new_value: Option<Value>) -> Item {
        Item {
            x_axis,
            y_axis,
            old_value,
            new_value,
        }
    }

    /// Returns the x-axis of an item of the history
    pub fn x_axis(&self) -> u8 {
        self.x_axis
    }

    /// Returns the y-axis of an item of the history
    pub fn y_axis(&self) -> u8 {
        self.y_axis
    }

    /// Returns the previous value of an item of the history
    pub fn old_value(&self) -> Option<Value> {
        self.old_value
    }

    /// Returns the new value of an item of the history
    pub fn new_value(&self) -> Option<Value> {
        self.new_value
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value_as_str = |value: Option<Value>| -> String {
            match value {
                Some(val) => format!("{}", val),
                None => "''".to_owned(),
            }
        };

        write!(f, "x-axis: {}, y-axis: {}, old value: {}, new value: {}", self.x_axis, self.y_axis,
               value_as_str(self.old_value), value_as_str(self.new_value))
    }
}
