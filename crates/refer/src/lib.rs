use std::{error::Error, result};

pub type Result<T> = result::Result<T, Box<dyn Error>>;

mod error;
mod record;
pub mod reader;
pub use reader::Reader;

pub mod writer;
pub use writer::Writer;
