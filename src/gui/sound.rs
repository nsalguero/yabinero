//! # Sound
//!
//! `sound` contains the functions to play the sounds of the game

use std::{fmt, fs::File, path::Path, io::BufReader, thread};

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
            let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let file = File::open(Path::new("sounds").join(snd_file)).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        });
    }
}
