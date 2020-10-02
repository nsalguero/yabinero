//! # Sound
//!
//! `sound` contains the functions to play the sounds of the game

use std::{fmt, fs::File, path::Path, io::BufReader, thread};
use rodio::Source;

/// The two possible types of sound
#[derive(Debug)]
pub enum Sound {
    Success,
    Error,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut printable = format!("{:?}", self);
        printable.make_ascii_lowercase();
        write!(f, "{}.ogg", printable)
    }
}

impl Sound {
    /// Plays a sound
    pub fn play(&self) {
        let snd_file = format!("{}", *self);
        thread::spawn(move || {
            let device = rodio::default_output_device().unwrap();
            let file = File::open(Path::new("sounds").join(snd_file)).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            rodio::play_raw(&device, source.convert_samples());
        });
    }
}
