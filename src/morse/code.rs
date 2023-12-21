use crate::gpio::Led;
use crate::morse::{LARGE, SMALL};
use std::time::Duration;

pub enum MorseCode {
    DOT,
    HYPHEN,
    SLASH,
}

impl MorseCode {
    pub fn new(char: char) -> Option<MorseCode> {
        Some(match char {
            '.' => MorseCode::DOT,
            '-' => MorseCode::HYPHEN,
            '/' => MorseCode::SLASH,
            _ => return None,
        })
    }

    pub fn led(&self) -> Led {
        match self {
            MorseCode::SLASH => Led::Green,
            _ => Led::Red,
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            MorseCode::DOT => SMALL,
            MorseCode::HYPHEN | MorseCode::SLASH => LARGE,
        }
    }
}
