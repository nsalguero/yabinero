//! # User Data
//!
//! `user_data` contains the functions that handles the user's preferences and best scores

use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;
use std::cell::RefCell;
use preferences::{AppInfo, PreferencesMap, Preferences};
use fltk::{app::{AppScheme, screen_size}, dialog::alert};
use tr::tr;
use enum_iterator::IntoEnumIterator;
use chrono::Local;
use crate::gui::timer::Timer;
use crate::size::Size;
use crate::difficulty::Difficulty;

/// The user's preferences
pub struct UserPrefs {
    faves: PreferencesMap<String>,
}

impl UserPrefs {
    /// Returns the user's preferences
    pub fn new() -> UserPrefs {
        let load_result = PreferencesMap::load(&APP_INFO, UserPrefs::PREFS_KEY);
        if load_result.is_ok() {
            UserPrefs {
                faves: load_result.unwrap(),
            }
        } else {
            let mut faves = PreferencesMap::new();
            faves.insert("size".to_owned(), "6".to_owned());
            faves.insert("difficulty".to_owned(), "Beginner".to_owned());
            faves.insert("sounds".to_owned(), "true".to_owned());
            faves.insert("theme".to_owned(), "Gtk".to_owned());
            let result = UserPrefs {
                faves,
            };
            result.save();
            result
        }
    }

    /// Returns the current size
    pub fn size(&self) -> Size {
        if let Some(size) = self.faves.get("size") {
            if let Some(result) = Size::from_str(size) {
                result
            } else {
                UserPrefs::bad_size()
            }
        } else {
            UserPrefs::bad_size()
        }
    }

    /// Sets the size
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    pub fn set_size(&mut self, size: Size) {
        self.faves.insert("size".to_owned(), format!("{}", size.as_u8()));
        self.save();
    }

    /// Returns the current difficulty
    pub fn difficulty(&self) -> Difficulty {
        if let Some(difficulty) = self.faves.get("difficulty") {
            if let Some(result) = Difficulty::from_str(difficulty) {
                result
            } else {
                UserPrefs::bad_difficulty()
            }
        } else {
            UserPrefs::bad_difficulty()
        }
    }

    /// Sets the difficulty
    ///
    /// # Arguments
    ///
    /// * `difficulty` - a difficulty
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.faves.insert("difficulty".to_owned(), format!("{:?}", difficulty));
        self.save();
    }

    /// Returns whether or not the sounds must be played
    pub fn sounds(&self) -> bool {
        if let Some(sounds) = self.faves.get("sounds") {
            if let Ok(result) = bool::from_str(sounds) {
                result
            } else {
                UserPrefs::bad_sounds()
            }
        } else {
            UserPrefs::bad_sounds()
        }
    }

    /// Sets whether or not the sounds must be played
    ///
    /// # Arguments
    ///
    /// * `sounds` - whether or not the sounds must be played
    pub fn set_sounds(&mut self, sounds: bool) {
        self.faves.insert("sounds".to_owned(), format!("{}", sounds));
        self.save();
    }

    /// Returns the current theme
    pub fn theme(&self) -> AppScheme {
        if let Some(theme) = self.faves.get("theme") {
            match theme.as_str() {
                "Base" => AppScheme::Base,
                "Gtk" => AppScheme::Gtk,
                "Gleam" => AppScheme::Gleam,
                "Plastic" => AppScheme::Plastic,
                _ => {
                    UserPrefs::bad_theme()
                },
            }
        } else {
            UserPrefs::bad_theme()
        }
    }

    /// Sets the theme
    ///
    /// # Arguments
    ///
    /// * `theme` - a theme
    pub fn set_theme(&mut self, theme: AppScheme) {
        self.faves.insert("theme".to_owned(), format!("{:?}", theme));
        self.save();
    }

    /// Displays an error
    ///
    /// # Arguments
    ///
    /// * `msg` - an error message
    fn display_error(msg: &str) {
        let (width, height) = screen_size();
        alert(width as i32 / 2 - 302, height as i32 / 2 - 14, msg);
    }

    /// Saves the user's preferences
    fn save(&self) {
        let save_result = self.faves.save(&APP_INFO, UserPrefs::PREFS_KEY);
        if !save_result.is_ok() {
            UserPrefs::display_error(&tr!("User preferences cannot be saved!"));
        }
    }

    /// Returns the default size when the size cannot be read from the user's preferences
    fn bad_size() -> Size {
        UserPrefs::display_error(&tr!("Bad size!"));
        Size::Side6
    }

    /// Returns the default difficulty when the difficulty cannot be read from the user's preferences
    fn bad_difficulty() -> Difficulty {
        UserPrefs::display_error(&tr!("Bad difficulty!"));
        Difficulty::Beginner
    }

    /// Returns `true` when the choice about playing the sounds cannot be read from the user's preferences
    fn bad_sounds() -> bool {
        UserPrefs::display_error(&tr!("Unable to know whether or not the sounds must be played!"));
        true
    }

    /// Returns the default theme when the theme cannot be read from the user's preferences
    fn bad_theme() -> AppScheme {
        UserPrefs::display_error(&tr!("Bad theme!"));
        AppScheme::Base
    }

    const PREFS_KEY: &'static str = "yabinero";
}

/// The best scores
pub struct BestScores {
    scores: HashMap<String, Rc<RefCell<PreferencesMap<String>>>>,
}

impl BestScores {
    /// Returns the best scores
    pub fn new() -> BestScores {
        let mut scores = HashMap::new();
        for size in Size::into_enum_iter() {
            for difficulty in Difficulty::into_enum_iter() {
                let key = BestScores::key(size, difficulty);
                let load_result = PreferencesMap::<String>::load(&APP_INFO, &key);
                let score = if load_result.is_ok() {
                    load_result.unwrap()
                } else {
                    PreferencesMap::new()
                };
                scores.insert(key, Rc::new(RefCell::new(score)));
            }
        }
        BestScores {
            scores,
        }
    }

    /// Returns the best scores for a size and a difficulty
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `difficulty` - a difficulty
    pub fn best_scores(&self, size: Size, difficulty: Difficulty) -> String {
        let key = BestScores::key(size, difficulty);
        let best_scores = self.scores.get(&key).unwrap();
        let mut result = "".to_owned();
        for (ranking, score) in best_scores.borrow().iter() {
            result.push_str(&format!("{:>2}\t", ranking));
            result.push_str(score);
            result.push_str("\n");
        }
        result
    }

    /// Adds a score to the bests scores if that score is a best one
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `difficulty` - a difficulty
    /// * `timer` - a timer
    pub fn add_best_score(&mut self, size: Size, difficulty: Difficulty, timer: &Rc<RefCell<Timer>>) {
        let duration = timer.borrow().duration();
        let mut ranking = format!("{}", BestScores::MAX_BEST_SCORE + 1);
        let key = BestScores::key(size, difficulty);
        let best_scores = self.scores.get(&key).unwrap();
        for (rank, score) in best_scores.borrow().iter() {
            let dur: Vec<&str> = score.split(" - ").collect();
            let dur: Vec<&str> = dur[0].split(":").collect();
            let dur: u64 = BestScores::duration_as_u64(dur[0]) * 60 + BestScores::duration_as_u64(dur[1]);
            if dur > duration {
                ranking = rank.to_string();
                break;
            }
        }
        let mut score = Timer::format(duration);
        score.push_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut old_score = Some(score);
        while BestScores::ranking_as_u8(&ranking) <= BestScores::MAX_BEST_SCORE && old_score.is_some() {
            old_score = best_scores.borrow_mut().insert(ranking.clone(), old_score.unwrap());
            ranking = format!("{}", BestScores::ranking_as_u8(&ranking) + 1);
        }
    }

    fn ranking_as_u8(ranking: &str) -> u8 {
        ranking.parse().unwrap()
    }

    fn duration_as_u64(duration: &str) -> u64 {
        duration.parse().unwrap()
    }

    /// Returns the key for a size and a difficulty
    ///
    /// # Arguments
    ///
    /// * `size` - a size
    /// * `difficulty` - a difficulty
    fn key(size: Size, difficulty: Difficulty) -> String {
        format!("{}-{}", size, difficulty)
    }

    const MAX_BEST_SCORE: u8 = 10;
}

const APP_INFO: AppInfo = AppInfo{name: "yabinero", author: "Nicolas Salguero"};
