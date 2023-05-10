use std::fs::File;
use std::io::{self, BufWriter, IntoInnerError, Write};
use std::path::Path;
use std::result::Result as StdResult;

use crate::{error::Result, reader::parse_input_line, record::Record};

/// A writer for a refer file.
///
/// It's a simple wrapper of a [io::BufWriter], along
/// with a line number tracker to help track errors when
/// parsing the input.
pub struct Writer<W: io::Write> {
    pub wtr: io::BufWriter<W>,
    line_no: u64,
}

impl Writer<File> {
    /// Build a refer writer that writes data to the given file path.
    /// *The file is truncated if it already exists*.
    ///
    /// If there was a problem opening the file at the given path, then this
    /// returns the corresponding error.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::error::Error;
    /// use refer::Writer;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn Error>> {
    ///     let mut wtr = Writer::from_path("foo.refer")?;
    ///     wtr.write_record(vec!["%A Brown, M", "%J PNAS"])?;
    ///     wtr.write_record(vec!["%A Twyford, A. D.", "%J PNAS"])?;
    ///     wtr.flush()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<File>> {
        Ok(Writer::new(File::create(path)?))
    }
}

impl<W: io::Write> Writer<W> {
    /// Create a new refer writer.
    ///
    /// # Example
    ///
    /// ```
    /// use std::error::Error;
    /// use refer::Writer;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn Error>> {
    ///     let mut wtr = Writer::new(vec![]);
    ///     wtr.write_record(vec!["%A Brown, M", "%J PNAS"])?;
    ///     wtr.write_record(vec!["%A Twyford, A. D.", "%J PNAS"])?;
    ///
    ///     let data = String::from_utf8(wtr.into_inner()?)?;
    ///     assert_eq!(data, "%A Brown, M\n%J PNAS\n\n%A Twyford, A. D.\n%J PNAS\n\n");
    ///     Ok(())
    /// }
    /// ```
    pub fn new(wtr: W) -> Writer<W> {
        Writer {
            wtr: io::BufWriter::new(wtr),
            line_no: 0,
        }
    }

    /// For each input field, we parse it and check it's correct.
    fn check_field(&self, field: String, record: &mut Record) -> Result<()> {
        parse_input_line(field, record, self.line_no)?;
        Ok(())
    }

    /// The main function used to write a record to an output buffer.
    pub fn write_record<I, T>(&mut self, record: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]> + std::convert::Into<Vec<u8>>,
    {
        let mut record_holder = Record::default();
        for field in record {
            self.line_no += 1;
            let field_string = String::from_utf8(field.into()).unwrap();
            self.check_field(field_string, &mut record_holder)?;
        }
        self.wtr.write_all(record_holder.to_string().as_bytes())?;
        // must be newline at end of record
        self.wtr.write_all(b"\n")?;
        Ok(())
    }
    /// Flush the contents of the current buffer and return the underlying writer.
    pub fn into_inner(self) -> StdResult<W, IntoInnerError<BufWriter<W>>> {
        self.wtr.into_inner()
    }

    /// Flush the writer at the end of usage.
    pub fn flush(&mut self) -> StdResult<(), io::Error> {
        self.wtr.flush()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn wtr_as_string(wtr: Writer<Vec<u8>>) -> String {
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }

    #[test]
    fn one_record() {
        let mut wtr = Writer::new(vec![]);
        wtr.write_record(vec!["%A Brown, M", "%T refer crate"])
            .unwrap();

        assert_eq!(wtr_as_string(wtr), "%A Brown, M\n%T refer crate\n\n");
    }
}
