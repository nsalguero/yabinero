//! # Changing
//!
//! `changing` represents the changing part of the GUI, used during a game

use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use tr::tr;
use std::rc::Rc;
use std::cell::RefCell;
use fltk::{button::Button, enums::{Color, Event}, prelude::{ImageExt, InputExt, WidgetExt}, frame::Frame, image::SvgImage, input::Input};
use crate::engine::Binero;
use crate::size::Size;
use crate::value::Value;
use enum_iterator::IntoEnumIterator;

/// The changing part of the GUI, used during a game
pub struct ChangingPart {
    grids: HashMap<Size, Rc<RefCell<Vec<Vec<Input>>>>>,
    //waiting: Rc<RefCell<Frame>>,
    //pause: Rc<RefCell<Frame>>,
    timer: Rc<RefCell<Frame>>,
    but_pause: Rc<RefCell<Button>>,
    but_resume: Rc<RefCell<Button>>,
    but_undo: Rc<RefCell<Button>>,
    but_redo: Rc<RefCell<Button>>,
    but_retry: Rc<RefCell<Button>>,
}

impl ChangingPart {
    /// Returns the changing part of the GUI, used during a game
    ///
    /// # Arguments
    ///
    /// * `starting_y` - the starting point for the height of the grid in the GUI
    /// * `ending_x` - the ending point for the width of the part of the GUI used during a game
    /// * `ending_y` - the ending point for the height of the part of the GUI used during a game
    pub fn new(starting_y: i32, ending_x: i32, ending_y: i32) -> ChangingPart {
        let mut grids = HashMap::new();
        for size in Size::into_enum_iter() {
            grids.insert(size, ChangingPart::init_grid(size.as_u8(), starting_y));
        }
        let starting_x = Size::into_enum_iter().last().unwrap().as_u8() as i32 * ChangingPart::INPUT_SIZE + ChangingPart::MARGIN_X;
        let width = ending_x - starting_x - ChangingPart::MARGIN_X;
        let timer = ChangingPart::init_timer(starting_x, starting_y + ChangingPart::MARGIN_Y, width);
        let but_pause = ChangingPart::init_button(starting_x, ending_y - ChangingPart::HEIGHT - ChangingPart::MARGIN_Y, width, PlayButton::Pause);
        let but_resume = ChangingPart::init_button(starting_x, ending_y - ChangingPart::HEIGHT - ChangingPart::MARGIN_Y, width, PlayButton::Resume);
        let but_undo = ChangingPart::init_button(starting_x, ending_y - 2 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Undo);
        let but_redo = ChangingPart::init_button(starting_x, ending_y - 3 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Redo);
        let but_retry = ChangingPart::init_button(starting_x, ending_y - 4 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Retry);
        ChangingPart {
            grids,
            timer,
            but_pause,
            but_resume,
            but_undo,
            but_redo,
            but_retry,
        }
    }

    /// Fills the grid of the game with a binero
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `binero` - a binero
    pub fn fill(changing: &Rc<RefCell<ChangingPart>>, binero: Rc<RefCell<Binero>>) {
        let size = binero.borrow().size();
        for (a_size, boxes) in &changing.borrow().grids {
            if *a_size == size {
                ChangingPart::fill_selected_grid(&boxes, size.as_u8(), &binero);
            } else {
                ChangingPart::hide_selected_grid(&boxes, size.as_u8());
            }
        }
    }

    /// Returns a grid
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `starting_y` - the starting point for the height of the grid in the GUI
    fn init_grid(size: u8, starting_y: i32) -> Rc<RefCell<Vec<Vec<Input>>>> {
        // TODO center the grid
        let mut boxes = Vec::new();
        for i in 0..size {
            boxes.push(Vec::new());
            for j in 0..size {
                let mut input = Input::new(j as i32 * ChangingPart::INPUT_SIZE,
                                           starting_y + i as i32 * ChangingPart::INPUT_SIZE,
                                           ChangingPart::INPUT_SIZE, ChangingPart::INPUT_SIZE, "");
                input.set_text_size(20);
                input.hide();
                boxes[i as usize].push(input);
            }
        }
        Rc::new(RefCell::new(boxes))
    }

    /// Returns the timer
    ///
    /// # Arguments
    ///
    /// * `x` - the value in x-axis
    /// * `y` - the value in y-axis
    /// * `width` - the width
    fn init_timer(x: i32, y: i32, width: i32) -> Rc<RefCell<Frame>> {
        let mut timer = Frame::new(x, y, width, 120, "00:00");
        if let Ok(mut img) = SvgImage::load(&Path::new("icons").join("chrono.svg")) {
            img.scale(80, 80, true, true);
            timer.set_image(&img);
        }
        //timer.hide();
        Rc::new(RefCell::new(timer))
    }

    /// Returns a button
    ///
    /// # Arguments
    ///
    /// * `x` - the value in x-axis
    /// * `y` - the value in y-axis
    /// * `width` - the width
    /// * `play_button` - a `PlayButton`
    fn init_button(x: i32, y: i32, width: i32, play_button: PlayButton) -> Rc<RefCell<Button>> {
        let mut button = Button::new(x, y, width, ChangingPart::HEIGHT, &format!("{}", play_button));
        button.set_color(Color::Light2);
        //button.hide();
        Rc::new(RefCell::new(button))
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
                ChangingPart::fill_box(input, binero, i, j);
                ChangingPart::add_event_handler(boxes, input, binero, i, j);
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
                            // TODO Display a popup saying "only 0 or 1" + error sound if sounds are activated
                        } else {
                            let old_value = cloned_binero.borrow().get(x_axis, y_axis);
                            if old_value != Value::from_u8(val) {
                                if cloned_binero.borrow_mut().try_to_put(x_axis, y_axis, Value::from_u8(val)) {
                                    cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize].set_value(&format!(" {}", value.trim()));
                                } else {
                                    cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize].undo();
                                    // TODO Display a popup saying "bad value" + error sound if sounds are activated
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
    const HEIGHT: i32 = 60;
    const MARGIN_X: i32 = 20;
    const MARGIN_Y: i32 = 4;
}

enum PlayButton {
    Pause,
    Resume,
    Undo,
    Redo,
    Retry,
}

impl fmt::Display for PlayButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PlayButton::Pause => tr!("Pause"),
            PlayButton::Resume => tr!("Resume"),
            PlayButton::Undo => tr!("Undo"),
            PlayButton::Redo => tr!("Redo"),
            PlayButton::Retry => tr!("Retry"),
        };
        write!(f, "{}", printable)
    }
}
