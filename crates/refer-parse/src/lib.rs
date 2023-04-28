use std::{error::Error, result};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub mod reader;