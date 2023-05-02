// a refer format writer

use std::fs::File;
use std::io::{self, BufWriter, IntoInnerError, Write};
use std::path::Path;

use crate::{error::Result, reader::parse_input_line, record::Record};

pub struct Writer<W: io::Write> {
    pub wtr: io::BufWriter<W>,
}

impl Writer<File> {
    //
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<File>> {
        Ok(Writer::new(File::create(path)?))
    }
}

impl<W: io::Write> Writer<W> {
    pub fn new(wtr: W) -> Writer<W> {
        Writer {
            wtr: io::BufWriter::new(wtr),
        }
    }
    //
    fn check_field(&self, field: String, record: &mut Record) -> Result<()> {
        parse_input_line(field, record)?;
        Ok(())
    }
    //
    pub fn write_record<I, T>(&mut self, record: I) -> Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]> + std::convert::Into<Vec<u8>>,
    {
        let mut record_holder = Record::default();
        for field in record {
            let field_string = String::from_utf8(field.into()).unwrap();
            self.check_field(field_string, &mut record_holder)?;
        }
        self.wtr.write_all(record_holder.to_string().as_bytes())?;
        // must be newline at end of record
        self.wtr.write_all(b"\n")?;
        Ok(())
    }
    /// Flush the contents of the current buffer and return the underlying writer
    pub fn into_inner(self) -> std::result::Result<W, IntoInnerError<BufWriter<W>>> {
        self.wtr.into_inner()
    }
    pub fn flush(&mut self) -> std::result::Result<(), io::Error> {
        self.wtr.flush()
    }
}
