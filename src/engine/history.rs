//! # History
//!
//! `history` manages the history of the game

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

    pub fn latest_choice(&mut self) -> Option<usize> {
        self.choices.pop()
    }

    pub fn current_item(&self) -> Option<usize> {
        self.current_item
    }

    pub fn is_undo_possible(&self) -> bool {
        self.current_item.is_some()
    }

    pub fn is_redo_possible(&self) -> bool {
        self.next_item().is_some()
    }

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

    /// Panics if no possible redo
    pub fn redo(&mut self) -> &Item {
        let next = self.next_item().unwrap();
        self.current_item = Some(next);
        self.items.get(next).unwrap()
    }

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

    fn pop_all_after_current_item(&mut self) {
        if let Some(next) = self.next_item() {
            let size = self.items.len();
            if next < size {
                for i in next..size {
                    self.items.pop();
                    self.choices.retain(|&item| item != i);
                }
            }
        }
    }
}

pub struct Item {
    x_axis: u8,
    y_axis: u8,
    old_value: Option<Value>,
    new_value: Option<Value>,
}

impl Item {
    fn new(x_axis: u8, y_axis: u8, old_value: Option<Value>, new_value: Option<Value>) -> Item {
        Item {
            x_axis,
            y_axis,
            old_value,
            new_value,
        }
    }

    pub fn x_axis(&self) -> u8 {
        self.x_axis
    }

    pub fn y_axis(&self) -> u8 {
        self.y_axis
    }

    pub fn old_value(&self) -> Option<Value> {
        self.old_value
    }

    pub fn new_value(&self) -> Option<Value> {
        self.new_value
    }
}
