//! # Grid
//!
//! `grid` represents the grid of the game in the GUI

use std::rc::Rc;
use std::cell::RefCell;
use fltk::{enums::{Color, Event}, prelude::{InputExt, WidgetExt}, input::Input};
use crate::engine::Binero;
use crate::value;

/// Returns the grid of the game in the GUI, ready to be played
///
/// # Arguments
///
/// * `game` - a binero
/// * `starting_y` - the starting point for the height of the grid in the GUI
pub fn create(game: &Rc<RefCell<Binero>>, starting_y: i32) -> Rc<RefCell<Vec<Vec<Input>>>> {
    let boxes = init(&game, starting_y);
    populate(&boxes, &game);
    boxes
}

/// Returns an empty grid of the game in the GUI
///
/// # Arguments
///
/// * `game` - a binero
/// * `starting_y` - the starting point for the height of the grid in the GUI
fn init(game: &Rc<RefCell<Binero>>, starting_y: i32) -> Rc<RefCell<Vec<Vec<Input>>>> {
    let mut boxes = Vec::new();
    for i in 0..game.borrow().size() {
        boxes.push(Vec::new());
        for j in 0..game.borrow().size() {
            let mut input = Input::new(j as i32 * INPUT_SIZE, starting_y + i as i32 * INPUT_SIZE, INPUT_SIZE, INPUT_SIZE, "");
            input.set_text_size(20);
            if let Some(val) = game.borrow().get(i as u8, j as u8) {
                input.set_value(&format!(" {}", val));
                input.set_readonly(true);
                input.set_text_color(Color::Inactive);
                input.set_selection_color(Color::Dark1);
            } else {
                input.set_value(" ");
                input.set_selection_color(Color::Dark3);
            }
            boxes[i as usize].push(input);
        }
    }
    Rc::new(RefCell::new(boxes))
}

/// Populates the grid of the game in the GUI
///
/// # Arguments
///
/// * `boxes` - an empty grid of the game in the GUI
/// * `game` - a binero
fn populate(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, game: &Rc<RefCell<Binero>>) {
    for i in 0..game.borrow().size() {
        for j in 0..game.borrow().size() {
            let cloned_boxes = Rc::clone(&boxes);
            let cloned_game = Rc::clone(&game);
            boxes.borrow_mut()[i as usize][j as usize].handle(Box::new(move |ev: Event| {
                match ev {
                    Event::KeyUp => {
                        let value = cloned_boxes.borrow()[i as usize][j as usize].value();
                        if let Ok(val) = value.trim().parse() {
                            if val != 0 && val != 1 {
                                cloned_boxes.borrow_mut()[i as usize][j as usize].undo();
                            } else {
                                let old_value = cloned_game.borrow().get(i, j);
                                if old_value != value::from_u8(val) {
                                    if cloned_game.borrow_mut().try_to_put(i, j, value::from_u8(val)) {
                                        cloned_boxes.borrow_mut()[i as usize][j as usize].set_value(&format!(" {}", value.trim()));
                                    } else {
                                        cloned_boxes.borrow_mut()[i as usize][j as usize].undo();
                                    }
                                }
                            }
                        }
                        true
                    },
                    _ => false,
                }
            }));
        }
    }
}

const INPUT_SIZE: i32 = 32;
