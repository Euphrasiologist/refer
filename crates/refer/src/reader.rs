use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_alphabetic,
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

use std::io::{self, Read};
use std::path::Path;
use std::{fs::File, io::BufRead};

use crate::{
    error::{Error, ErrorKind, Result},
    record::{Author, Record},
    str_from_utf8,
};

/// A refer reader.
///
/// # Example
///
/// The refer reader has a convenient constructor method `from_path`
/// to read a file from a path. To create a refer reader from a generic
/// reader, use `Reader::new()`. Below, we just use some bytes.
///
/// ```
/// use std::error::Error;
/// use refer::{Reader, record::{Author, Record}};
///
/// # fn main() { example().unwrap(); }
/// fn example() -> Result<(), Box<dyn Error>> {
///     let data = "%A Brown, M.\n%J PNAS\n".as_bytes();
///
///     let mut rdr = Reader::new(data);
///
///     if let Some(result) = rdr.records().next() {
///         let record = result?;
///         assert_eq!(record, Record {
///             author: vec![Author { last: "Brown".into(), rest: "M.".into() }],
///             journal: Some("PNAS".into()),
///             ..Default::default()
///         });
///         Ok(())
///     } else {
///         Err(From::from("expected at least one record, but got none"))
///     }
/// }
///
/// ```
///
/// # Error handling
///
/// Errors can arise in parsing, as records must conform to the refer specification.
/// However, the specification is not exactly followed, and is a bit more relaxed. Most
/// fields in a record will return `Option<String>`. For exact details on errors, please
/// see [`Error`](enum.Error.html).
///
pub struct Reader<R> {
    /// The underlying reader.
    rdr: io::BufReader<R>,
    /// The line number to keep track of errors if we find any.
    line: u64,
}

impl Reader<File> {
    /// Create a reader from a given file path.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::error::Error;
    /// use refer::Reader;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn Error>> {
    ///     let mut rdr = Reader::from_path("foo.refer")?;
    ///     for result in rdr.records() {
    ///         let record = result?;
    ///         println!("{:?}", record);
    ///     }
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<File>> {
        Ok(Reader::new(File::open(path)?))
    }
}

impl<R: io::Read> Reader<R> {
    pub fn new(rdr: R) -> Reader<R> {
        Reader {
            rdr: io::BufReader::new(rdr),
            line: 0,
        }
    }

    /// A borrowed iterator over the records of a refer file.
    pub fn records(&mut self) -> RecordsIter<R> {
        RecordsIter::new(self)
    }

    /// An owned iterator over the records of a refer file.
    pub fn into_records(self) -> RecordsIntoIter<R> {
        RecordsIntoIter::new(self)
    }

    /// Read a single record from an input reader.
    fn read_record(&mut self) -> Result<Option<Record>> {
        // read the line
        // if line is empty/only contains spaces, end read
        // if line contains something,
        let mut record = Record::default();
        let reader = self.rdr.by_ref();

        // might have to read each line separately
        // using read_line()
        let mut temp_buf = String::new();

        loop {
            self.line += 1;
            temp_buf.clear();
            let bytes = reader.read_line(&mut temp_buf)?;
            if bytes == 0 {
                // this is the EOF
                if record == Record::default() {
                    return Ok(None);
                } else {
                    return Ok(Some(record));
                }
            }
            // we've cleared the buffer and read the line
            if temp_buf.trim().is_empty() {
                // if the record is empty AND we hit a newline,
                // just keep going, otherwise we break becasue
                // we already processed a record :)
                if record == Record::default() {
                    continue;
                } else {
                    break;
                }
            }

            let parsed = parse_input_line(temp_buf.clone(), &mut record, self.line);

            match parsed {
                Ok(e) => match e {
                    Some(_) => continue,
                    None => break,
                },
                Err(e) => return Err(e),
            }
        }

        Ok(Some(record))
    }
}

/// A borrowed iterator over the records of a refer file.
pub struct RecordsIter<'r, R: 'r> {
    /// The underlying reader
    rdr: &'r mut Reader<R>,
}

impl<'r, R: io::Read> RecordsIter<'r, R> {
    fn new(rdr: &'r mut Reader<R>) -> RecordsIter<'r, R> {
        RecordsIter { rdr }
    }
    /// Return a reference to the underlying reader.
    pub fn reader(&self) -> &Reader<R> {
        self.rdr
    }

    /// Return a mutable reference to the underlying reader.
    pub fn reader_mut(&mut self) -> &mut Reader<R> {
        self.rdr
    }
}

impl<'r, R: io::Read> Iterator for RecordsIter<'r, R> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Result<Record>> {
        match self.rdr.read_record() {
            Ok(Some(r)) => {
                self.rdr.line += 1;
                Some(Ok(r))
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// An owned iterator over the records of a refer file.
pub struct RecordsIntoIter<R> {
    /// The underlying reader.
    rdr: Reader<R>,
}

impl<R: io::Read> RecordsIntoIter<R> {
    fn new(rdr: Reader<R>) -> RecordsIntoIter<R> {
        RecordsIntoIter { rdr }
    }
    /// Return a reference to the underlying reader.
    pub fn reader(&self) -> &Reader<R> {
        &self.rdr
    }

    /// Return a mutable reference to the underlying reader.
    pub fn reader_mut(&mut self) -> &mut Reader<R> {
        &mut self.rdr
    }

    /// Drop this iterator and return the underlying reader.
    pub fn into_reader(self) -> Reader<R> {
        self.rdr
    }
}

impl<R: io::Read> Iterator for RecordsIntoIter<R> {
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Result<Record>> {
        match self.rdr.read_record() {
            Ok(Some(r)) => {
                self.rdr.line += 1;
                Some(Ok(r))
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// Parse a single line from an input string into a mutable `Record` instance.
// does not need this return type, can simplify
pub(crate) fn parse_input_line(
    input: String,
    record: &mut Record,
    line_no: u64,
) -> Result<Option<()>> {
    let bytes = input.as_bytes();
    // parse the authors
    if let Err(e) = parse_author_line(bytes, line_no) {
        if let ErrorKind::Author(_) = e.kind() {
            return Err(e);
        }
    }

    if let Ok(author_list) = parse_author_line(bytes, line_no) {
        record.author.push(author_list);
        return Ok(Some(()));
    }

    // TODO: handle this error separately, somehow..?
    if let Ok(keywords) = parse_keywords_line(bytes) {
        let keywords_str: Result<Vec<String>> = keywords
            .iter()
            .map(|e| str_from_utf8(e).map(|e| e.to_owned()))
            .collect();

        record.keywords = Some(keywords_str?);
        return Ok(Some(()));
    }

    if let Ok((editor, _)) = parse_editor_line(bytes) {
        record.editor.push(str_from_utf8(editor)?.trim().into());
        return Ok(Some(()));
    }

    // deal with the rest in one big alt
    let (parsed, line_tag) = match alt((
        parse_book_line,
        parse_place_line,
        parse_date_line,
        parse_government_line,
        parse_issuer_line,
        parse_journal_line,
        parse_label_line,
        parse_issue_number_line,
        parse_other_line,
        parse_page_number_line,
        parse_author_np_line,
        parse_report_line,
        parse_series_line,
        parse_title_line,
        parse_volume_line,
        parse_annotation_line,
    ))(bytes)
    {
        Ok(e) => e,
        Err(e) => match e {
            nom::Err::Incomplete(_) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. In parsing the fields, an incomplete error was raised.",
                    line_no
                ))))
            }
            nom::Err::Error(e) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. Parsing error with code ({}): {}",
                    line_no,
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
            nom::Err::Failure(e) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. Parsing failure with code ({}): {}",
                    line_no,
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
        },
    };

    let tag = str_from_utf8(line_tag)?;

    let parsed = str_from_utf8(parsed)?.trim().to_string();
    // skip empty fields
    if parsed.is_empty() {
        return Ok(Some(()));
    }

    match tag {
        "%B " => record.book = Some(parsed),
        "%C " => record.place = Some(parsed),
        "%D " => record.date = Some(parsed),
        "%G " => record.government = Some(parsed),
        "%I " => record.issuer = Some(parsed),
        "%J " => record.journal = Some(parsed),
        "%L " => record.label = Some(parsed),
        "%N " => record.issue_number = Some(parsed),
        "%O " => record.other = Some(parsed),
        "%P " => record.page_number = Some(parsed),
        "%Q " => record.author_np = Some(parsed),
        "%R " => record.report = Some(parsed),
        "%S " => record.series = Some(parsed),
        "%T " => record.title = Some(parsed),
        "%V " => record.volume = Some(parsed),
        "%X " => record.annotation = Some(parsed),
        // should never get here
        t => return Err(Error::new(ErrorKind::TagNotFound(t.to_string()))),
    }
    Ok(Some(()))
}

/// Parse author tag.
fn parse_author_tag(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%A ")(i)
}

// Parse the author name.
fn parse_author_name(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    let sep1 = tag(", ");
    let sep2 = tag(" ");
    separated_list0(
        alt((sep1, sep2)),
        // TODO: there are probably other edge cases here
        take_while(|e| is_alphabetic(e) || e == b'-' || e == b'.'),
    )(i)
}

/// Parse the author line.
fn parse_author_line(line: &[u8], line_no: u64) -> Result<Author> {
    let (_, parsed) = match preceded(parse_author_tag, parse_author_name)(line) {
        Ok(p) => p,
        Err(e) => match e {
            nom::Err::Incomplete(_) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. In parsing the fields, an incomplete error was raised.",
                    line_no
                ))))
            }
            nom::Err::Error(e) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. Parsing error with code ({}): {}",
                    line_no,
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
            nom::Err::Failure(e) => {
                return Err(Error::new(ErrorKind::NomError(format!(
                    "At line: {}. Parsing failure with code ({}): {}",
                    line_no,
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
        },
    };

    match parsed.len() {
        0..=1 => {
            return Err(Error::new(ErrorKind::Author(format!(
                "Input error: {}. `Number of names should be of length 2 or more, found 0 or 1",
                str_from_utf8(line)?
            ))))
        }
        2 => Ok(Author {
            last: str_from_utf8(parsed[0])?.to_owned(),
            rest: str_from_utf8(parsed[1])?.to_owned(),
        }),
        n @ 3.. => {
            let mut rest = String::new();
            for el in parsed.iter().take(n).skip(1) {
                rest += str_from_utf8(el)?;
                rest += " ";
            }
            rest.pop();

            Ok(Author {
                last: str_from_utf8(parsed[0])?.to_owned(),
                rest,
            })
        }
        e => {
            return Err(Error::new(ErrorKind::Author(format!(
                "Input error: {}. `Number of names should be of length 2 or more, found {}",
                str_from_utf8(line)?,
                e
            ))))
        }
    }
}

/// Parse the book title.
fn parse_book_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%B ")(i)
}
/// Parse place line.
fn parse_place_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%C ")(i)
}

/// Parse the date line.
fn parse_date_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%D ")(i)
}

// %E:  For an article that is part of a book, the name of an editor of the book.
// Where the work has editors and no authors, the names of the editors should be
// given as %A fields and , (ed) or , (eds) should be appended to the last author.
// possibly needs same treatment as Author.

/// Parse the editor line.
fn parse_editor_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%E ")(i)
}

/// Parse US Government ordering number line.
fn parse_government_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%G ")(i)
}

// Parse issuer line.
fn parse_issuer_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%I ")(i)
}

// Parse journal line.
fn parse_journal_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%J ")(i)
}

/// Parse keywords tag.
fn parse_keywords_tag(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%K ")(i)
}

/// Parse all keywords in a keyword line.
fn parse_all_keywords(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    separated_list0(tag(" "), take_while(is_alphabetic))(i)
}

/// Parse the full keywords line.
fn parse_keywords_line(i: &[u8]) -> Result<Vec<&[u8]>> {
    let (_, parsed) = match preceded(parse_keywords_tag, parse_all_keywords)(i) {
        Ok(p) => p,
        Err(e) => match e {
            nom::Err::Incomplete(_) => {
                return Err(Error::new(ErrorKind::Keyword(
                    "In parsing the fields, an incomplete error was raised.".to_string(),
                )))
            }
            nom::Err::Error(e) => {
                return Err(Error::new(ErrorKind::Keyword(format!(
                    "Keyword line parsing error with code ({}): {}",
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
            nom::Err::Failure(e) => {
                return Err(Error::new(ErrorKind::Keyword(format!(
                    "Line parsing failure with code ({}): {}",
                    e.code.description(),
                    str_from_utf8(e.input)?
                ))))
            }
        },
    };

    Ok(parsed)
}

/// Parse label line.
fn parse_label_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%L ")(i)
}

/// Parse issue number line.
fn parse_issue_number_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%N ")(i)
}

/// Parse other information line.
fn parse_other_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%O ")(i)
}

// Parse page number line.
fn parse_page_number_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%P ")(i)
}

/// Parse author line (not a person).
fn parse_author_np_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%Q ")(i)
}

/// Parse technical report number line.
fn parse_report_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%R ")(i)
}

/// Parse series name line.
fn parse_series_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%S ")(i)
}

/// Parse title line.
fn parse_title_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%T ")(i)
}

/// Parse volume line.
fn parse_volume_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%V ")(i)
}

/// Parse annotation line.
fn parse_annotation_line(i: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("%X ")(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_record() {}

    #[test]
    fn test_parse_author_tag() {
        let author_string = b"%A Carter-Brown, M.";
        let (parsed, _) = parse_author_tag(author_string).unwrap();
        assert_eq!(parsed, b"Carter-Brown, M.")
    }

    #[test]
    fn test_parse_author_name() {
        let author_string = b"Carter-Brown, M.";
        let (_, y) = parse_author_name(author_string).unwrap();
        let res: Vec<&str> = y.iter().map(|e| std::str::from_utf8(e).unwrap()).collect();

        assert_eq!(vec!["Carter-Brown", "M."], res)
    }

    #[test]
    fn test_parse_author_line() {
        let author_string = b"%A Carter-Brown, M.";
        let parsed = parse_author_line(author_string, 1).unwrap();

        assert_eq!(parsed.rest, "M.");
        assert_eq!(parsed.last, "Carter-Brown");
    }

    #[test]
    fn test_parse_author_line_one() {
        let author_string = b"%A Carter-Brown";
        let parsed = parse_author_line(author_string, 1);

        assert!(parsed.is_err())
    }

    #[test]
    fn test_keywords_line() {
        let keywords = b"%K keyword another word";
        let parsed = parse_keywords_line(keywords).unwrap();
        assert_eq!(
            parsed,
            &[
                "keyword".as_bytes(),
                "another".as_bytes(),
                "word".as_bytes()
            ]
        )
    }

    #[test]
    fn parse_input_line_1() {
        let input = String::from("%B a book");
        let mut record = Record::default();
        let _ = parse_input_line(input, &mut record, 0).unwrap();

        let expected_record = Record {
            book: Some("a book".into()),
            ..Default::default()
        };

        assert_eq!(expected_record, record);
    }

    #[test]
    fn test_reader_1() {
        let mut reader = Reader::new("%A Brown, M.\n%T a title\n".as_bytes());
        let record = reader.records().next().unwrap().unwrap();

        let expected_record = Record {
            author: vec![Author {
                last: "Brown".into(),
                rest: "M.".into(),
            }],
            title: Some("a title".into()),
            ..Default::default()
        };

        assert_eq!(expected_record, record);
    }
}
