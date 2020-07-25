//! # Timer
//!
//! `timer` handles the timer

use std::{sync::{Arc, Mutex, mpsc::{self, Sender}}, thread, time::{Duration, Instant}};
use fltk::{prelude::{WidgetExt}, frame::Frame};
use crate::gui::set_svg;

pub struct Timer {
    timer: Arc<Mutex<Frame>>,
    old_duration: Arc<Mutex<u64>>,
    curr_start: Arc<Mutex<Instant>>,
}

impl Timer {
    /// Returns a timer
    pub fn new(x: i32, y: i32, width: i32) -> Timer {
        let timer = Timer::init_timer(x, y, width);
        let old_duration = Arc::new(Mutex::new(0));
        let curr_start = Arc::new(Mutex::new(Instant::now()));
        Timer {
            timer,
            old_duration,
            curr_start,
        }
    }

    /// Starts the timer
    pub fn start(&mut self) -> Sender<bool> {
        self.reset();
        let cloned_timer = Arc::clone(&self.timer);
        let cloned_old_duration = Arc::clone(&self.old_duration);
        let cloned_curr_start = Arc::clone(&self.curr_start);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut pause = false;
            loop {
                thread::sleep(Timer::WAITING_DURATION);
                if let Ok(p) = rx.try_recv() {
                    pause = p;
                    if pause {
                        let mut old_duration = cloned_old_duration.lock().unwrap();
                        *old_duration += cloned_curr_start.lock().unwrap().elapsed().as_secs();
                    } else {
                        let mut curr_start = cloned_curr_start.lock().unwrap();
                        *curr_start = Instant::now();
                    }
                }
                if !pause {
                    let duration = cloned_curr_start.lock().unwrap().elapsed().as_secs() + *cloned_old_duration.lock().unwrap();
                    Timer::display_duration(&cloned_timer, duration);
                }
            }
        });
        tx
    }

    /// Returns the current duration
    pub fn duration(&self) -> u64 {
        *self.old_duration.lock().unwrap()
    }

    /// Formats a duration
    ///
    /// # Arguments
    ///
    /// * `duration` - a duration
    pub fn format(duration: u64) -> String {
        format!("{:02}:{:02}", duration / 60, duration % 60)
    }

    /// Refreshes the duration in the GUI
    pub fn refresh_duration(&self) {
        Timer::display_duration(&self.timer, self.duration());
    }

    /// Displays a duration in the GUI
    ///
    /// # Arguments
    ///
    /// * `timer` - the timer in the GUI
    /// * `duration` - a duration
    fn display_duration(timer: &Arc<Mutex<Frame>>, duration: u64) {
        timer.lock().unwrap().set_label(&Timer::format(duration));
    }

    /// Returns the `Frame` displaying the timer
    ///
    /// # Arguments
    ///
    /// * `x` - the value in x-axis
    /// * `y` - the value in y-axis
    /// * `width` - the width
    fn init_timer(x: i32, y: i32, width: i32) -> Arc<Mutex<Frame>> {
        let mut timer = Frame::new(x, y, width, 120, "00:00");
        set_svg(&mut timer, "chrono.svg");
        timer.hide();
        Arc::new(Mutex::new(timer))
    }

    /// Resets the timer
    fn reset(&mut self) {
        let mut curr_start = self.curr_start.lock().unwrap();
        *curr_start = Instant::now();
        let mut old_duration = self.old_duration.lock().unwrap();
        *old_duration = 0;
        self.timer.lock().unwrap().show();
    }

    pub const WAITING: u64 = 100;

    const WAITING_DURATION: Duration = Duration::from_millis(Timer::WAITING);
}
