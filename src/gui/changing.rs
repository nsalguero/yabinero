//! # Changing
//!
//! `changing` represents the changing part of the GUI, used during a game

use std::{cell::RefCell, collections::HashMap, fmt, path::Path, rc::Rc, sync::mpsc::Sender, thread, time::Duration};
use tr::tr;
use fltk::{button::Button, enums::{Color, Event}, prelude::{ImageExt, InputExt, WidgetBase, WidgetExt}, frame::Frame, image::SvgImage, input::Input};
use enum_iterator::{all, last};
use crate::engine::{Binero, history::Item};
use crate::enums::{Difficulty, Size, Value};
use crate::gui::{BG_COLOR, SELECT_COLOR, RO_SELECT_COLOR, display_alert, display_message, sound::Sound, timer::Timer, user_data::{UserPrefs, BestScores}};

/// The changing part of the GUI, used during a game
pub struct ChangingPart {
    grids: HashMap<Size, Rc<RefCell<Vec<Vec<Input>>>>>,
    pause: Frame,
    timer: Rc<RefCell<Timer>>,
    but_pause: Button,
    but_resume: Button,
    but_undo: Button,
    but_redo: Button,
    but_retry: Button,
    but_solve: Button,
    success: bool,
    paused: bool,
    binero: Option<Rc<RefCell<Binero>>>,
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
        let max_size = last::<Size>().unwrap().as_u8() as i32;
        for size in all::<Size>() {
            let delta_to_center = (max_size - size.as_u8() as i32) / 2 * ChangingPart::INPUT_SIZE;
            grids.insert(size, ChangingPart::init_grid(size.as_u8(), starting_y, delta_to_center));
        }
        let starting_x = max_size * ChangingPart::INPUT_SIZE + ChangingPart::MARGIN_X;
        let width = ending_x - starting_x - ChangingPart::MARGIN_X;
        let timer = Rc::new(RefCell::new(Timer::new(starting_x, starting_y + ChangingPart::MARGIN_Y, width)));
        let but_pause = ChangingPart::init_button(starting_x, ending_y - ChangingPart::HEIGHT - ChangingPart::MARGIN_Y, width, PlayButton::Pause);
        let but_resume = ChangingPart::init_button(starting_x, ending_y - ChangingPart::HEIGHT - ChangingPart::MARGIN_Y, width, PlayButton::Resume);
        let but_undo = ChangingPart::init_button(starting_x, ending_y - 5 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Undo);
        let but_redo = ChangingPart::init_button(starting_x, ending_y - 4 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Redo);
        let but_retry = ChangingPart::init_button(starting_x, ending_y - 3 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Retry);
        let but_solve = ChangingPart::init_button(starting_x, ending_y - 2 * (ChangingPart::HEIGHT + ChangingPart::MARGIN_Y), width, PlayButton::Solve);
        let pause = ChangingPart::init_pause(starting_x, ending_y);
        ChangingPart {
            grids,
            pause,
            timer,
            but_pause,
            but_resume,
            but_undo,
            but_redo,
            but_retry,
            but_solve,
            success: false,
            paused: false,
            binero: None,
        }
    }

    /// Creates a new game and returns the `Sender` to pause the game
    ///
    /// # Arguments
    ///
    /// * `user_prefs` - the user's preferences
    /// * `changing` - the changing part of the GUI
    pub fn new_game(user_prefs: &Rc<RefCell<UserPrefs>>, changing: &Rc<RefCell<ChangingPart>>) -> Sender<bool> {
        let binero = Rc::new(RefCell::new(Binero::new(user_prefs.borrow().size(), user_prefs.borrow().difficulty())));
        changing.borrow_mut().binero = Some(binero);
        let tx_result = changing.borrow_mut().timer.borrow_mut().start();
        ChangingPart::fill(changing, user_prefs, &tx_result, user_prefs.borrow().difficulty(), &changing.borrow().timer);
        ChangingPart::add_pause_handler(changing, user_prefs.borrow().size(), Sender::clone(&tx_result));
        ChangingPart::add_resume_handler(changing, user_prefs.borrow().size(), Sender::clone(&tx_result));
        ChangingPart::add_undo_handler(changing);
        ChangingPart::add_redo_handler(changing);
        ChangingPart::add_retry_handler(changing);
        ChangingPart::add_solve_handler(changing, user_prefs);
        changing.borrow_mut().pause.hide();
        changing.borrow_mut().paused = false;
        changing.borrow_mut().success = false;
        tx_result
    }

    /// Pauses the game
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    pub fn pause_game(changing: &Rc<RefCell<ChangingPart>>) {
        changing.borrow_mut().paused = true;
        changing.borrow_mut().but_resume.show();
        changing.borrow_mut().but_pause.hide();
        for (_, boxes) in &changing.borrow().grids {
            ChangingPart::hide_selected_grid(&boxes);
        }
    }

    /// Cancels the current action
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    pub fn undo(changing: &Rc<RefCell<ChangingPart>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        if !changing.borrow().success && !changing.borrow().paused {
            let size = binero.borrow().size();
            if let Some(item) = binero.borrow_mut().try_to_undo() {
                ChangingPart::set_value(&changing, size, item, true);
            }
        }
    }

    /// Replays the next action that was previously undone
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    pub fn redo(changing: &Rc<RefCell<ChangingPart>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        if !changing.borrow().success && !changing.borrow().paused {
            let size = binero.borrow().size();
            if let Some(item) = binero.borrow_mut().try_to_redo() {
                ChangingPart::set_value(&changing, size, item, false);
            }
        }
    }

    /// Fills the grid of the game with a binero
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `user_prefs` - the user's preferences
    /// * `tx` - a `Sender`
    /// * `difficulty` - a difficulty
    /// * `timer` - a timer
    fn fill(changing: &Rc<RefCell<ChangingPart>>, user_prefs: &Rc<RefCell<UserPrefs>>, tx: &Sender<bool>, difficulty: Difficulty, timer: &Rc<RefCell<Timer>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        let size = binero.borrow().size();
        for (a_size, boxes) in &changing.borrow().grids {
            if *a_size == size {
                ChangingPart::fill_selected_grid(&boxes, user_prefs, tx, difficulty, timer, changing);
            } else {
                ChangingPart::hide_selected_grid(&boxes);
            }
        }
    }

    /// Returns a grid
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `starting_y` - the starting point for the height of the grid in the GUI
    /// * `delta_to_center` - the delta to center the grid
    fn init_grid(size: u8, starting_y: i32, delta_to_center: i32) -> Rc<RefCell<Vec<Vec<Input>>>> {
        let mut boxes = Vec::new();
        for i in 0..size {
            boxes.push(Vec::new());
            for j in 0..size {
                let mut input = Input::new(j as i32 * ChangingPart::INPUT_SIZE + delta_to_center,
                                           starting_y + i as i32 * ChangingPart::INPUT_SIZE + delta_to_center,
                                           ChangingPart::INPUT_SIZE, ChangingPart::INPUT_SIZE, "");
                input.set_text_size(20);
                input.hide();
                boxes[i as usize].push(input);
            }
        }
        Rc::new(RefCell::new(boxes))
    }

    /// Returns the `Frame` displayed when the game is paused
    ///
    /// # Arguments
    ///
    /// * `ending_x` - the ending point for the width of the part of the GUI used during a game
    /// * `ending_y` - the ending point for the height of the part of the GUI used during a game
    fn init_pause(ending_x: i32, ending_y: i32) -> Frame {
        let mut pause = Frame::new(0, 0, ending_x, ending_y, "");
        if let Ok(mut img) = SvgImage::load(&Path::new("icons").join("pause.svg")) {
            img.scale(200, 200, true, true);
            pause.set_image(Some(img));
        }
        pause.hide();
        pause
    }

    /// Returns a button
    ///
    /// # Arguments
    ///
    /// * `x` - the value in x-axis
    /// * `y` - the value in y-axis
    /// * `width` - the width
    /// * `play_button` - a `PlayButton`
    fn init_button(x: i32, y: i32, width: i32, play_button: PlayButton) -> Button {
        let mut button = Button::new(x, y, width, ChangingPart::HEIGHT, "");
        button.set_label(&format!("{}", play_button));
        button.set_color(BG_COLOR);
        button.hide();
        button
    }

    /// Hides the selected grid
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    fn hide_selected_grid(boxes: &Rc<RefCell<Vec<Vec<Input>>>>) {
        let size = boxes.borrow().len();
        for i in 0..size {
            for j in 0..size {
                let input = &mut boxes.borrow_mut()[i][j];
                input.hide();
            }
        }
    }

    /// Shows the selected grid
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    fn show_selected_grid(boxes: &Rc<RefCell<Vec<Vec<Input>>>>) {
        let size = boxes.borrow().len();
        for i in 0..size {
            for j in 0..size {
                let input = &mut boxes.borrow_mut()[i][j];
                input.show();
            }
        }
    }

    /// Fills the selected grid
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    /// * `user_prefs` - the user's preferences
    /// * `tx` - a `Sender`
    /// * `difficulty` - a difficulty
    /// * `timer` - a timer
    /// * `changing` - the changing part of the GUI
    fn fill_selected_grid(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, user_prefs: &Rc<RefCell<UserPrefs>>, tx: &Sender<bool>, difficulty: Difficulty, timer: &Rc<RefCell<Timer>>, changing: &Rc<RefCell<ChangingPart>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        let size = boxes.borrow().len();
        for i in 0..size {
            for j in 0..size {
                let input = &mut boxes.borrow_mut()[i][j];
                ChangingPart::fill_box(input, &binero, i as u8, j as u8, user_prefs.borrow().color(), user_prefs.borrow().ro_color());
                ChangingPart::add_event_handler(boxes, input, i as u8, j as u8, user_prefs, tx, difficulty, timer, changing);
            }
        }
    }

    /// Fills a box
    ///
    /// # Arguments
    ///
    /// * `input` - a box
    /// * `value` - a value
    fn fill_box_with_value(input: &mut Input, value: &Option<Value>) {
        let val = if let Some(v) = value {
            format!(" {}", v)
        } else {
            String::from(" ")
        };
        input.set_value(&val);
    }

    /// Fills a box
    ///
    /// # Arguments
    ///
    /// * `input` - a box
    /// * `binero` - a binero
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `color` - the color of the writable boxes
    /// * `ro_ color` - the color of the read-only boxes
    fn fill_box(input: &mut Input, binero: &Rc<RefCell<Binero>>, x_axis: u8, y_axis: u8, color: Color, ro_color: Color) {
        let value = binero.borrow().get(x_axis, y_axis);
        ChangingPart::fill_box_with_value(input, &value);
        if value.is_some() {
            input.set_readonly(true);
            input.set_text_color(ro_color);
            input.set_selection_color(RO_SELECT_COLOR);
        } else {
            input.set_readonly(false);
            input.set_text_color(color);
            input.set_selection_color(SELECT_COLOR);
        }
        input.show();
    }

    /// Adds the event handler to the box
    ///
    /// # Arguments
    ///
    /// * `boxes` - a grid
    /// * `input` - a box
    /// * `x_axis` - an unsigned 8-bit integer that gives the x-axis
    /// * `y_axis` - an unsigned 8-bit integer that gives the y-axis
    /// * `user_prefs` - the user's preferences
    /// * `tx` - a `Sender`
    /// * `difficulty` - a difficulty
    /// * `timer` - a timer
    /// * `changing` - the changing part of the GUI
    fn add_event_handler(boxes: &Rc<RefCell<Vec<Vec<Input>>>>, input: &mut Input, x_axis: u8, y_axis: u8, user_prefs: &Rc<RefCell<UserPrefs>>, tx: &Sender<bool>, difficulty: Difficulty, timer: &Rc<RefCell<Timer>>, changing: &Rc<RefCell<ChangingPart>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        let cloned_boxes = Rc::clone(boxes);
        let cloned_binero = Rc::clone(&binero);
        let cloned_prefs = Rc::clone(user_prefs);
        let cloned_tx = Sender::clone(tx);
        let cloned_timer = Rc::clone(timer);
        let cloned_changing = Rc::clone(changing);
        input.handle(Box::new(move |_: &mut Input, ev: Event| {
            match ev {
                Event::KeyUp | Event::Unfocus => {
                    let old_value = cloned_binero.borrow().get(x_axis, y_axis);
                    let value = cloned_boxes.borrow()[x_axis as usize][y_axis as usize].value();
                    let val = value.trim();
                    if val == "0" || val == "1" {
                        let val = Value::from_u8(val.parse().unwrap());
                        if old_value != val {
                            if cloned_binero.borrow_mut().try_to_put(x_axis, y_axis, val) {
                                ChangingPart::fill_box_with_value(&mut cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize], &val);
                                if cloned_binero.borrow().is_full() {
                                    cloned_tx.send(true).unwrap();
                                    ChangingPart::display_success(cloned_prefs.borrow().sounds(), cloned_binero.borrow().size(), difficulty, &cloned_timer, &cloned_changing);
                                }
                            } else {
                                ChangingPart::fill_box_with_value(&mut cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize], &old_value);
                                ChangingPart::display_error(&tr!("Bad value!"), cloned_prefs.borrow().sounds());
                            }
                        }
                    } else if val == "" {
                        if old_value.is_some() {
                            ChangingPart::fill_box_with_value(&mut cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize], &None);
                            cloned_binero.borrow_mut().try_to_put(x_axis, y_axis, None);
                        }
                    } else {
                        ChangingPart::fill_box_with_value(&mut cloned_boxes.borrow_mut()[x_axis as usize][y_axis as usize], &old_value);
                        ChangingPart::display_error(&tr!("Bad value!"), cloned_prefs.borrow().sounds());
                    }
                    true
                },
                _ => false,
            }
        }));
    }

    /// Displays a popup with an error message and play the error sound if sounds are activated
    ///
    /// # Arguments
    ///
    /// * `msg` - the error message
    /// * `sounds` - whether or not the sounds must be played
    fn display_error(msg: &str, sounds: bool) {
        if sounds {
            Sound::Error.play();
        }
        display_alert(msg);
    }

    /// Displays a popup with a success message and play the success sound if sounds are activated
    ///
    /// # Arguments
    ///
    /// * `sounds` - whether or not the sounds must be played
    /// * `size` - a size
    /// * `difficulty` - a difficulty
    /// * `timer` - a timer
    /// * `changing` - the changing part of the GUI
    fn display_success(sounds: bool, size: Size, difficulty: Difficulty, timer: &Rc<RefCell<Timer>>, changing: &Rc<RefCell<ChangingPart>>) {
        changing.borrow_mut().success = true;
        const WAITING_DURATION: Duration = Duration::from_millis(Timer::WAITING * 2);
        thread::sleep(WAITING_DURATION);
        if sounds {
            Sound::Success.play();
        }
        let mut best_scores = BestScores::new();
        best_scores.add_best_score(size, difficulty, timer);
        timer.borrow().refresh_duration();
        display_message(&tr!("Congratulations, you won!"));
    }

    /// Adds the handler to the Pause button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `size` - a size
    /// * `tx` - a `Sender`
    fn add_pause_handler(changing: &Rc<RefCell<ChangingPart>>, size: Size, tx: Sender<bool>) {
        let cloned_changing = Rc::clone(changing);
        cloned_changing.borrow_mut().but_pause.show();
        changing.borrow_mut().but_pause.set_callback(Box::new(move |_: &mut Button| {
            if !cloned_changing.borrow().success {
                tx.send(true).unwrap();
                cloned_changing.borrow_mut().paused = true;
                cloned_changing.borrow_mut().but_resume.show();
                cloned_changing.borrow_mut().but_pause.hide();
                if let Some(boxes) = cloned_changing.borrow().grids.get(&size) {
                    ChangingPart::hide_selected_grid(&boxes);
                }
                cloned_changing.borrow_mut().pause.show();
            }
        }));
    }

    /// Adds the handler to the Resume button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `size` - a size
    /// * `tx` - a `Sender`
    fn add_resume_handler(changing: &Rc<RefCell<ChangingPart>>, size: Size, tx: Sender<bool>) {
        let cloned_changing = Rc::clone(changing);
        cloned_changing.borrow_mut().but_resume.hide();
        changing.borrow_mut().but_resume.set_callback(Box::new(move |_: &mut Button| {
            tx.send(false).unwrap();
            cloned_changing.borrow_mut().paused = false;
            cloned_changing.borrow_mut().but_pause.show();
            cloned_changing.borrow_mut().but_resume.hide();
            if let Some(boxes) = cloned_changing.borrow().grids.get(&size) {
                ChangingPart::show_selected_grid(&boxes);
            }
            cloned_changing.borrow_mut().pause.hide();
        }));
    }

    /// Sets a value in the grid
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `size` - a size
    /// * `item` - an item of the history
    /// * `undo` - whether or not the operation is undo
    fn set_value(changing: &Rc<RefCell<ChangingPart>>, size: Size, item: &Item, undo: bool) {
        let mut select_boxes: Option<&Rc<RefCell<Vec<Vec<Input>>>>> = None;
        let grids = &changing.borrow().grids;
        for (a_size, boxes) in grids {
            if *a_size == size {
                select_boxes = Some(boxes);
            }
        }
        let boxes = select_boxes.unwrap();
        let value = if undo {
            item.old_value()
        } else {
            item.new_value()
        };
        ChangingPart::fill_box_with_value(&mut boxes.borrow_mut()[item.x_axis() as usize][item.y_axis() as usize], &value);
    }

    /// Adds the handler to the Undo button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    fn add_undo_handler(changing: &Rc<RefCell<ChangingPart>>) {
        changing.borrow_mut().but_undo.show();
        let cloned_changing = Rc::clone(changing);
        changing.borrow_mut().but_undo.set_callback(Box::new(move |_: &mut Button| {
            ChangingPart::undo(&cloned_changing);
        }));
    }

    /// Adds the handler to the Redo button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    fn add_redo_handler(changing: &Rc<RefCell<ChangingPart>>) {
        changing.borrow_mut().but_redo.show();
        let cloned_changing = Rc::clone(changing);
        changing.borrow_mut().but_redo.set_callback(Box::new(move |_: &mut Button| {
            ChangingPart::redo(&cloned_changing);
        }));
    }

    /// Adds the handler to the Retry button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    fn add_retry_handler(changing: &Rc<RefCell<ChangingPart>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        changing.borrow_mut().but_retry.show();
        let cloned_changing = Rc::clone(changing);
        changing.borrow_mut().but_retry.set_callback(Box::new(move |_: &mut Button| {
            if !cloned_changing.borrow().success && !cloned_changing.borrow().paused {
                let size = binero.borrow().size();
                while let Some(item) = binero.borrow_mut().try_to_undo() {
                    ChangingPart::set_value(&cloned_changing, size, item, true);
                }
                binero.borrow_mut().clear_history();
            }
        }));
    }

    /// Adds the handler to the Solve button
    ///
    /// # Arguments
    ///
    /// * `changing` - the changing part of the GUI
    /// * `binero` - a binero
    /// * `user_prefs` - the user's preferences
    fn add_solve_handler(changing: &Rc<RefCell<ChangingPart>>, user_prefs: &Rc<RefCell<UserPrefs>>) {
        let binero = changing.borrow().binero.clone().unwrap();
        changing.borrow_mut().but_solve.show();
        let cloned_changing = Rc::clone(changing);
        let cloned_prefs = Rc::clone(user_prefs);
        changing.borrow_mut().but_solve.set_callback(Box::new(move |_: &mut Button| {
            if !cloned_changing.borrow().success && !cloned_changing.borrow().paused {
                let size = binero.borrow().size();
                let result = binero.borrow_mut().try_to_solve();
                while let Some(item) = binero.borrow_mut().try_to_undo() {
                    ChangingPart::set_value(&cloned_changing, size, item, true);
                }
                while let Some(item) = binero.borrow_mut().try_to_redo() {
                    ChangingPart::set_value(&cloned_changing, size, item, false);
                }
                if !result {
                    ChangingPart::display_error(&tr!("No solution!"), cloned_prefs.borrow().sounds());
                }
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
    Solve,
}

impl fmt::Display for PlayButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            PlayButton::Pause => tr!("Pause"),
            PlayButton::Resume => tr!("Resume"),
            PlayButton::Undo => tr!("Undo"),
            PlayButton::Redo => tr!("Redo"),
            PlayButton::Retry => tr!("Retry"),
            PlayButton::Solve => tr!("Solve"),
        };
        write!(f, "{}", printable)
    }
}
