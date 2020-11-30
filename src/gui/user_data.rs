//! # User Data
//!
//! `user_data` contains the functions that handles the user's preferences and best scores

use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};
use preferences::{AppInfo, PreferencesMap, Preferences};
use fltk::{app::AppScheme, enums::Color, utils::hex2rgb};
use tr::tr;
use enum_iterator::IntoEnumIterator;
use chrono::Local;
use crate::gui::{FG_COLOR, RO_FG_COLOR, display_alert, timer::Timer};
use crate::enums::{Difficulty, Size};

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
            faves.insert("color".to_owned(), UserPrefs::color_as_string(&FG_COLOR));
            faves.insert("ro_color".to_owned(), UserPrefs::color_as_string(&RO_FG_COLOR));
            let result = UserPrefs {
                faves,
            };
            result.save(false);
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
        self.save(true);
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
        self.save(true);
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
        self.save(true);
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
        self.save(true);
    }

    /// Returns the current color of writable boxes
    pub fn color(&self) -> Color {
        if let Some(color_str) = self.faves.get("color") {
            if let Some(color) = UserPrefs::color_from_str(color_str) {
                color
            } else {
                *FG_COLOR
            }
        } else {
            *FG_COLOR
        }
    }

    /// Sets the color of writable boxes
    ///
    /// # Arguments
    ///
    /// * `color` - a RGB value
    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        self.faves.insert("color".to_owned(), UserPrefs::rgb_as_string(color));
        self.save(true);
    }

    /// Returns the current color of read-only boxes
    pub fn ro_color(&self) -> Color {
        if let Some(color_str) = self.faves.get("ro_color") {
            if let Some(color) = UserPrefs::color_from_str(color_str) {
                color
            } else {
                *RO_FG_COLOR
            }
        } else {
            *RO_FG_COLOR
        }
    }

    /// Sets the color of read-only boxes
    ///
    /// # Arguments
    ///
    /// * `color` - a RGB value
    pub fn set_ro_color(&mut self, color: (u8, u8, u8)) {
        self.faves.insert("ro_color".to_owned(), UserPrefs::rgb_as_string(color));
        self.save(true);
    }

    /// Returns a RGB value as a string
    ///
    /// # Arguments
    ///
    /// * `color` - a RGB value
    fn rgb_as_string(color: (u8, u8, u8)) -> String {
        format!("{:?}", color)
    }

    /// Returns a color as a string
    ///
    /// # Arguments
    ///
    /// * `color` - a color
    fn color_as_string(color: &Color) -> String {
        let hexa_val = u32::from_str_radix(&format!("{:?}", color)[0..6], 16).unwrap();
        UserPrefs::rgb_as_string(hex2rgb(hexa_val))
    }

    /// Saves the user's preferences
    ///
    /// # Arguments
    ///
    /// * `show_error` - Whether or not display a popup when an error has occurred
    fn save(&self, show_error: bool) {
        let save_result = self.faves.save(&APP_INFO, UserPrefs::PREFS_KEY);
        if !save_result.is_ok() && show_error {
            display_alert(&tr!("User preferences cannot be saved!"));
        }
    }

    /// Returns the color extracted from a string or `None` if there is a problem
    ///
    /// * `color_str` - a string that contains a color
    fn color_from_str(color_str: &str) -> Option<Color> {
        let color: Vec<&str> = color_str.split(",").collect();
        if color.len() == 3 {
            if let Ok(red) = color[0].replace("(", "").trim().parse() {
                if let Ok(green) = color[1].trim().parse() {
                    if let Ok(blue) = color[2].replace(")", "").trim().parse() {
                        return Some(Color::from_rgb(red, green, blue));
                    }
                }
            }
        }
        None
    }

    /// Returns the default size when the size cannot be read from the user's preferences
    fn bad_size() -> Size {
        display_alert(&tr!("Bad size!"));
        Size::Side6
    }

    /// Returns the default difficulty when the difficulty cannot be read from the user's preferences
    fn bad_difficulty() -> Difficulty {
        display_alert(&tr!("Bad difficulty!"));
        Difficulty::Beginner
    }

    /// Returns `true` when the choice about playing the sounds cannot be read from the user's preferences
    fn bad_sounds() -> bool {
        display_alert(&tr!("Unable to know whether or not the sounds must be played!"));
        true
    }

    /// Returns the default theme when the theme cannot be read from the user's preferences
    fn bad_theme() -> AppScheme {
        display_alert(&tr!("Bad theme!"));
        AppScheme::Base
    }

    const PREFS_KEY: &'static str = "yabinero";
}

/// The best scores
pub struct BestScores {
    scores: HashMap<String, PreferencesMap<String>>,
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
                scores.insert(key, score);
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
        let mut ranking: u8 = 1;
        while ranking <= BestScores::MAX_BEST_SCORE {
            if let Some(score) = best_scores.get(&format!("{}", ranking)) {
                result.push_str(&format!("{:02}\t", ranking));
                result.push_str(score);
            }
            result.push_str("\n");
            ranking += 1;
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
        let key = BestScores::key(size, difficulty);
        let best_scores = self.scores.get_mut(&key).unwrap();
        let mut ranking = String::from("1");
        let mut rank: u8 = 1;
        while rank <= BestScores::MAX_BEST_SCORE {
            let rank_str = format!("{}", rank);
            ranking = rank_str.clone();
            if let Some(score) = best_scores.get(&rank_str) {
                let dur = BestScores::duration(&score);
                if dur > duration {
                    break;
                }
                rank += 1;
            } else {
                break;
            }
        }
        if rank <= BestScores::MAX_BEST_SCORE {
            let mut old_score = Some(BestScores::score(duration));
            while BestScores::ranking_as_u8(&ranking) <= BestScores::MAX_BEST_SCORE && old_score.is_some() {
                old_score = best_scores.insert(ranking.clone(), old_score.unwrap());
                ranking = format!("{}", BestScores::ranking_as_u8(&ranking) + 1);
            }
        }
        BestScores::save(best_scores, key);
    }

    /// Extracts the duration from a score and returns it
    ///
    /// # Arguments
    ///
    /// * `score` - a score
    fn duration(score: &str) -> u64 {
        let duration: Vec<&str> = score.split("\t\t").collect();
        let duration: Vec<&str> = duration[0].split(":").collect();
        BestScores::duration_as_u64(duration[0]) * 60 + BestScores::duration_as_u64(duration[1])
    }

    /// Returns a score created using a duration
    ///
    /// # Arguments
    ///
    /// * `duration` - a duration
    fn score(duration: u64) -> String {
        let mut score = Timer::format(duration);
        score.push_str("\t\t");
        score.push_str(&Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        score
    }

    /// Saves some best scores
    ///
    /// # Arguments
    ///
    /// * `best_scores` - some best scores
    /// * `key` - a key
    fn save(best_scores: &mut PreferencesMap<String>, key: String) {
        let save_result = best_scores.save(&APP_INFO, key);
        if !save_result.is_ok() {
            display_alert(&tr!("Best scores cannot be saved!"));
        }
    }

    /// Returns a ranking as an unsigned 8-bit integer
    ///
    /// # Arguments
    ///
    /// * `ranking` - a ranking
    fn ranking_as_u8(ranking: &str) -> u8 {
        ranking.parse().unwrap()
    }

    /// Returns a duration as an unsigned 64-bit integer
    ///
    /// # Arguments
    ///
    /// * `duration` - a duration
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
        format!("{}-{:?}", size, difficulty)
    }

    const MAX_BEST_SCORE: u8 = 10;
}

const APP_INFO: AppInfo = AppInfo{name: "yabinero", author: "Nicolas Salguero"};
