use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;
use schemars::JsonSchema;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use thiserror::Error;

mod search;
mod transition;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
)]
pub enum Mode {
    Minor = 0,
    Major = 1,
}

impl Mode {
    fn swap(self) -> Self {
        match self {
            Mode::Minor => Mode::Major,
            Mode::Major => Mode::Minor,
        }
    }
}

#[derive(Error, Debug)]
pub enum ScaleError {
    #[error("invalid scale string provided")]
    InvalidScaleString,
}

impl FromStr for Mode {
    type Err = ScaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Minor,
            "B" => Self::Major,
            _ => return Err(ScaleError::InvalidScaleString),
        })
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minor => write!(f, "A"),
            Self::Major => write!(f, "B"),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Debug,
    Hash,
    DeserializeFromStr,
    SerializeDisplay,
    JsonSchema,
)]
pub struct Key {
    pub tonic: usize,
    pub mode: Mode,
}

fn mod_cyclic(num: isize, modulus: usize) -> isize {
    let modulus = modulus as isize;
    ((num % modulus) + modulus) % modulus
}

impl Key {
    pub fn swap_kind(self) -> Self {
        Self {
            mode: self.mode.swap(),
            ..self
        }
    }

    pub fn change_index(self, amount: isize) -> Self {
        let index = mod_cyclic((self.tonic as isize) + amount, 12);
        Self {
            tonic: index as usize,
            ..self
        }
    }
}

impl FromStr for Key {
    type Err = ScaleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(1|2|3|4|5|6|7|8|9|10|11|12)([AB])$").unwrap();

        let captures = re.captures(s).unwrap();

        let number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let letter = captures.get(2).unwrap().as_str();

        Ok(Key {
            tonic: number - 1,
            mode: Mode::from_str(letter).unwrap(),
        })
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.tonic + 1, self.mode)
    }
}

pub const fn scale(tonic: usize, mode: Mode) -> Key {
    Key { tonic, mode }
}

pub fn make_standard_scale() -> Vec<Key> {
    (0..=11)
        .flat_map(|i| [scale(i, Mode::Minor), scale(i, Mode::Major)])
        .collect::<Vec<_>>()
}
