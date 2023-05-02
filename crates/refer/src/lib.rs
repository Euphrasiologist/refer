mod error;
pub mod reader;
mod record;
pub use reader::Reader;

pub mod writer;
pub use writer::Writer;

fn str_from_utf8(input: &[u8]) -> Result<&str, error::Error> {
    match std::str::from_utf8(input) {
        Ok(s) => Ok(s),
        Err(e) => Err(error::Error::new(error::ErrorKind::Utf8(e))),
    }
}