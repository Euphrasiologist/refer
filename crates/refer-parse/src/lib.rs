use std::{error::Error, result};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

mod error;
pub mod reader;
pub use reader::Reader;
