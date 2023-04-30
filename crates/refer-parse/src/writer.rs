// a refer format writer

use crate::Result;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::reader::{parse_input_line, Author, Record};

use std::fmt::Display;

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "%A {}{} {}",
            self.first,
            match &self.middle {
                Some(e) => format!(" {}", e),
                None => "".into(),
            },
            self.last
        )
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.author.is_empty() {
            let mut author_list = String::new();
            for el in &self.author {
                author_list += &el.to_string();
                author_list += "\n";
            }
            write!(f, "{}", author_list)
        } else {
            // shouldn't get here?
            write!(f, "")
        }
    }
}

pub struct Writer<W: io::Write> {
    pub wtr: io::BufWriter<W>,
}

impl<W: io::Write> Writer<W> {
    pub fn new(wtr: W) -> Writer<W> {
        Writer {
            wtr: io::BufWriter::new(wtr),
        }
    }
    //
    pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<Writer<File>> {
        Ok(Writer::new(File::create(path)?))
    }
    //
    fn check_field(&self, field: String, record: &mut Record) -> Result<()> {
        parse_input_line(field, record)?;
        Ok(())
    }
    //
    pub fn write_record<'a, I>(&mut self, record: I) -> Result<()>
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let mut record_holder = Record::default();
        for field in record {
            let field_string = String::from_utf8(field.to_vec())?;
            self.check_field(field_string, &mut record_holder)?;
        }
        self.wtr.write_all(record_holder.to_string().as_bytes())?;
        // must be newline at end of record
        self.wtr.write_all(b"\n")?;
        Ok(())
    }
}
