//! # Grid
//!
//! `grid` represents the grid of the game in the GUI

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use fltk::{enums::{Color, Event}, prelude::{InputExt, WidgetExt}, input::Input};
use crate::engine::Binero;
use crate::size::Size;
use crate::value::Value;
use enum_iterator::IntoEnumIterator;

/// All the possible grids
pub struct GuiGrids {
    pub grids: HashMap<Size, Rc<RefCell<Vec<Vec<Input>>>>>,
}

impl GuiGrids {
    /// Returns all the possible grids
    ///
    /// # Arguments
    ///
    /// * `starting_y` - the starting point for the height of the grid in the GUI
    pub fn new(starting_y: i32) -> GuiGrids {
        let mut grids = HashMap::new();
        for size in Size::into_enum_iter() {
            grids.insert(size, GuiGrids::init_grid(size.as_u8(), starting_y));
        }
        GuiGrids {
            grids,
        }
    }

    /// Fills the grid of the game with a binero
    ///
    /// # Arguments
    ///
    /// * `grids` - the grids
    /// * `binero` - a binero
    pub fn fill(grids: &Rc<RefCell<GuiGrids>>, binero: Rc<RefCell<Binero>>) {
        let size = binero.borrow().size();
        for (a_size, boxes) in &grids.borrow().grids {
            if *a_size == size {
                GuiGrids::fill_selected_grid(&boxes, size.as_u8(), &binero);
            } else {
                GuiGrids::hide_selected_grid(&boxes, size.as_u8());
            }
        }
    }

    /// Initialise a grid
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `starting_y` - the starting point for the height of the grid in the GUI
    fn init_grid(size: u8, starting_y: i32) -> Rc<RefCell<Vec<Vec<Input>>>> {
        let mut boxes = Vec::new();
        for i in 0..size {
            boxes.push(Vec::new());
            for j in 0..size {
                let mut input = Input::new(j as i32 * GuiGrids::INPUT_SIZE,
                                           starting_y + i as i32 * GuiGrids::INPUT_SIZE,
                                           GuiGrids::INPUT_SIZE, GuiGrids::INPUT_SIZE, "");
                input.set_text_size(20);
                input.hide();
                boxes[i as usize].push(input);
            }
        }
        Rc::new(RefCell::new(boxes))
    }

    /// Hides the selected grid
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    /// * `size` - an unsigned 8-bit integer that gives the size
    fn hide_selected_grid(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, size: u8) {
        for i in 0..size {
            for j in 0..size {
                let input = &mut boxes.borrow_mut()[i as usize][j as usize];
                input.hide();
            }
        }
    }

    /// Fills the selected grid
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    /// * `size` - an unsigned 8-bit integer that gives the size
    /// * `binero` - a binero
    fn fill_selected_grid(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, size: u8, binero: &Rc<RefCell<Binero>>) {
        for i in 0..size {
            for j in 0..size {
                let input = &mut boxes.borrow_mut()[i as usize][j as usize];
                GuiGrids::fill_box(input, binero, i, j);
                GuiGrids::add_event_handler(boxes, input, binero, i, j);
            }
        }
    }

    /// Fills a box
    ///
    /// # Arguments
    ///
    /// * `input` - a box
    /// * `binero` - a binero
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    fn fill_box(input: &mut Input, binero: &Rc<RefCell<Binero>>, x_axis: u8, y_axis: u8) {
        if let Some(val) = binero.borrow().get(x_axis, y_axis) {
            input.set_value(&format!(" {}", val));
            input.set_readonly(true);
            input.set_text_color(Color::Inactive);
            input.set_selection_color(Color::Dark1);
        } else {
            input.set_value(" ");
            input.set_readonly(false);
            input.set_text_color(Color::Black);
            input.set_selection_color(Color::Dark3);
        }
        input.show();
    }

    /// Adds the event handler to the box
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    /// * `input` - a box
    /// * `binero` - a binero
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    fn add_event_handler(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, input: &mut Input, binero: &Rc<RefCell<Binero>>, x_axis: u8, y_axis: u8) {
        let cloned_boxes = Rc::clone(boxes);
        let cloned_binero = Rc::clone(&binero);
        input.handle(Box::new(move |ev: Event| {
            match ev {
                Event::KeyUp => {
                    let value = cloned_boxes.borrow()[x_axis as usize][y_axis as usize].value();
                    if let Ok(val) = value.trim().parse() {
                        if val != 0 && val != 1 {
                            cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize].undo();
                        } else {
                            let old_value = cloned_binero.borrow().get(x_axis, y_axis);
                            if old_value != Value::from_u8(val) {
                                if cloned_binero.borrow_mut().try_to_put(x_axis, y_axis, Value::from_u8(val)) {
                                    cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize].set_value(&format!(" {}", value.trim()));
                                } else {
                                    cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize].undo();
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

    const INPUT_SIZE: i32 = 32;
}
