#![feature(lazy_cell)]

pub mod camelot;

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use schemars::JsonSchema;
use serde_with::{DeserializeFromStr, SerializeDisplay};
