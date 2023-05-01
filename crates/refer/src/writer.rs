// a refer format writer

use crate::Result;
use std::fs::File;
use std::io::{self, BufWriter, IntoInnerError, Write};
use std::path::Path;

use crate::reader::{parse_input_line, Author, Record};

use std::fmt::Display;

pub struct Writer<W: io::Write> {
    pub wtr: io::BufWriter<W>,
}

impl<W: io::Write + std::marker::Send + std::fmt::Debug> Writer<W> {
    pub fn new(wtr: W) -> Writer<W> {
        Writer {
            wtr: io::BufWriter::new(wtr),
        }
    }
    //
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Writer<File>> {
        Ok(Writer::new(File::create(path)?))
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
        T: AsRef<[u8]> + std::convert::Into<std::vec::Vec<u8>>,
    {
        let mut record_holder = Record::default();
        for field in record {
            let field_string = String::from_utf8(field.into())?;
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
        // %A line
        match self.author.is_empty() {
            true => write!(f, "")?,
            false => {
                let mut author_list = String::new();
                for el in &self.author {
                    author_list += &el.to_string();
                    author_list += "\n";
                }
                write!(f, "{}", author_list)?
            }
        };

        // %B line
        match &self.book {
            Some(b) => writeln!(f, "%B {}", b)?,
            None => write!(f, "")?,
        };
        // %C line
        match &self.place {
            Some(p) => writeln!(f, "%C {}", p)?,
            None => write!(f, "")?,
        };
        // %D line
        match &self.date {
            Some(d) => writeln!(f, "%D {}", d)?,
            None => write!(f, "")?,
        };
        // %D line
        match &self.editor {
            Some(e) => writeln!(f, "%E {}", e)?,
            None => write!(f, "")?,
        };
        // %G line
        match &self.government {
            Some(g) => writeln!(f, "%G {}", g)?,
            None => write!(f, "")?,
        };
        // %I line
        match &self.issuer {
            Some(i) => writeln!(f, "%I {}", i)?,
            None => write!(f, "")?,
        };
        // %J line
        match &self.journal {
            Some(j) => writeln!(f, "%J {}", j)?,
            None => write!(f, "")?,
        };
        // %K line, all formatted with a space in between
        match &self.keywords {
            Some(k) => {
                let mut keywords = String::new();
                for el in k {
                    keywords += el;
                    keywords += " ";
                }
                keywords.pop();
                writeln!(f, "%K {}", keywords)?
            }
            None => write!(f, "")?,
        };
        // %L line
        match &self.label {
            Some(l) => writeln!(f, "%L {}", l)?,
            None => write!(f, "")?,
        };
        // %N line
        match &self.issue_number {
            Some(i) => writeln!(f, "%N {}", i)?,
            None => write!(f, "")?,
        };
        // %O line
        match &self.other {
            Some(o) => writeln!(f, "%O {}", o)?,
            None => write!(f, "")?,
        };
        // %P line
        match &self.page_number {
            Some(p) => writeln!(f, "%P {}", p)?,
            None => write!(f, "")?,
        };
        // %Q line
        match &self.author_np {
            Some(q) => writeln!(f, "%Q {}", q)?,
            None => write!(f, "")?,
        };
        // %R line
        match &self.report {
            Some(r) => writeln!(f, "%R {}", r)?,
            None => write!(f, "")?,
        };
        // %S line
        match &self.series {
            Some(s) => writeln!(f, "%S {}", s)?,
            None => write!(f, "")?,
        };
        // %T line
        match &self.title {
            Some(t) => writeln!(f, "%T {}", t)?,
            None => write!(f, "")?,
        };
        // %V line
        match &self.volume {
            Some(v) => writeln!(f, "%V {}", v)?,
            None => write!(f, "")?,
        };
        // %X line
        match &self.annotation {
            Some(x) => writeln!(f, "%X {}", x)?,
            None => write!(f, "")?,
        };

        Ok(())
    }
}
