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

use crate::error::ReferParseError;
use crate::{
    record::{Author, Record},
    Result,
};

/// A refer format reader which works on anything implementing [`io::Read`]
pub struct Reader<R> {
    /// The underlying reader.
    rdr: io::BufReader<R>,
}

impl Reader<File> {
    /// Create a reader from a path, or anything which can be converted into
    /// a path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Reader<File>> {
        Ok(Reader::new(File::open(path)?))
    }
}

impl<R: io::Read> Reader<R> {
    pub fn new(rdr: R) -> Reader<R> {
        Reader {
            rdr: io::BufReader::new(rdr),
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

            let parsed = parse_input_line(temp_buf.clone(), &mut record);

            match parsed {
                Ok(e) => match e {
                    Some(_) => continue,
                    None => break,
                },
                Err(e) => return Err(e),
            }
        }

        // eprintln!("We are returning a record!");

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
            Ok(Some(r)) => Some(Ok(r)),
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
            Ok(Some(r)) => Some(Ok(r)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

// want to check for duplication at some point.
pub fn parse_input_line(input: String, record: &mut Record) -> Result<Option<()>> {
    // we need something here to see if a record is: XXX\n
    // and if it is, we need to output this here so the parent function
    // can exit the loop, rather than going to the next iteration and not
    // giving us back the record!

    let bytes = input.as_bytes();
    // parse the authors
    if let Ok((_, author_list)) = parse_author_line(bytes) {
        record.author.push(author_list);
        return Ok(Some(()));
    }

    if let Ok((_, keywords)) = parse_keywords_line(bytes) {
        // eprintln!("keywords");
        let keywords_str: Vec<String> = keywords
            .iter()
            // TODO: fix this unwrap
            .map(|e| std::str::from_utf8(e).unwrap().to_owned())
            .collect();
        record.keywords = Some(keywords_str);
        return Ok(Some(()));
    }

    // deal with the rest in one big alt
    let (parsed, line_tag) = match alt((
        parse_book_line,
        parse_place_line,
        parse_date_line,
        parse_editor_line,
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
        Err(e) => return Err(Box::new(e)),
    };

    let tag = std::str::from_utf8(line_tag)?;
    // eprintln!("Line tag: {}", tag);
    let parsed = String::from_utf8(parsed.to_vec())?.trim().to_string();
    // eprintln!("Parsed bit: {}", parsed);

    match tag {
        "%B " => record.book = Some(parsed),
        "%C " => record.place = Some(parsed),
        "%D " => record.date = Some(parsed),
        "%E " => record.editor = Some(parsed),
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
        _ => panic!(""),
    }
    Ok(Some(()))
}

// parse %A ...
fn parse_author_tag(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%A ")(i)
}

// parse %A Author 1 (are there any other special chars apart from -)
fn parse_author_name(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>, ReferParseError> {
    separated_list0(
        tag(" "),
        // TODO: there are probably other edge cases here
        take_while(|e| is_alphabetic(e) || e == b'-' || e == b'.'),
    )(i)
}

fn parse_author_line(line: &[u8]) -> IResult<&[u8], Author, ReferParseError> {
    let (input, parsed) = preceded(parse_author_tag, parse_author_name)(line)?;
    match parsed.len() {
        // TODO: remove these unwraps! Assumes we are all utf-8 good!
        2 => Ok((
            input,
            Author {
                first: std::str::from_utf8(parsed[0]).unwrap().to_owned(),
                middle: None,
                last: std::str::from_utf8(parsed[1]).unwrap().to_owned(),
            },
        )),
        3 => Ok((
            input,
            Author {
                first: std::str::from_utf8(parsed[0]).unwrap().to_owned(),
                middle: Some(std::str::from_utf8(parsed[1]).unwrap().to_owned()),
                last: std::str::from_utf8(parsed[2]).unwrap().to_owned(),
            },
        )),
        e => Err(nom::Err::Error(crate::error::ReferParseError {
            // I realise this isn't totally inclusive
            message: format!(
                "Name is of length: {}, should be formatted <Author> <Middle> <Surname>",
                e
            ),
        })),
    }
}

// book title needs no further parsing
fn parse_book_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%B ")(i)
}
// needs no further parsing
fn parse_place_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%C ")(i)
}
// should be year (2023), and then month in letters
// or 'in press'/'unknown'
// maybe no further parsing required
fn parse_date_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%D ")(i)
}

// %E:  For an article that is part of a book, the name of an editor of the book.
// Where the work has editors and no authors, the names of the editors should be
// given as %A fields and , (ed) or , (eds) should be appended to the last author.
// possibly needs same treatment as Author.
fn parse_editor_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%E ")(i)
}

// %G:  US Government ordering number.
// not needed for my purposes, no further parsing
fn parse_government_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%G ")(i)
}

// %I:  The publisher (issuer).
fn parse_issuer_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%I ")(i)
}

// %J:  For an article in a journal, the name of the journal.
fn parse_journal_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%J ")(i)
}

// %K:  Keywords to be used for searching.
fn parse_keywords_tag(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%K ")(i)
}

fn parse_all_keywords(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>, ReferParseError> {
    separated_list0(tag(" "), take_while(is_alphabetic))(i)
}

fn parse_keywords_line(i: &[u8]) -> IResult<&[u8], Vec<&[u8]>, ReferParseError> {
    let (before, parsed) = preceded(parse_keywords_tag, parse_all_keywords)(i)?;
    Ok((before, parsed))
}

// %L:  Label.
fn parse_label_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%L ")(i)
}
// %N:  Journal issue number.
fn parse_issue_number_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%N ")(i)
}
// %O:  Other information. This is usually printed at the end of the reference.
fn parse_other_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%O ")(i)
}
// %P:  Page number. A range of pages can be specified as m-n.
fn parse_page_number_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%P ")(i)
}
// %Q:  The name of the author, if the author is not a person. This will only be used if there are no %A fields. There can only be one %Q field.
fn parse_author_np_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%Q ")(i)
}

// %R:  Technical report number.
fn parse_report_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%R ")(i)
}

// %S:  Series name.
fn parse_series_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%S ")(i)
}

// %T:  Title. For an article in a book or journal, this should be the title of the article.
fn parse_title_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%T ")(i)
}

// %V:  Volume number of the journal or book.
fn parse_volume_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%V ")(i)
}

// %X:  Annotation.
fn parse_annotation_line(i: &[u8]) -> IResult<&[u8], &[u8], ReferParseError> {
    tag("%X ")(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_author_tag() {
        let author_string = b"%A Max Carter-Brown";
        let (parsed, _) = parse_author_tag(author_string).unwrap();
        assert_eq!(parsed, b"Max Carter-Brown")
    }

    #[test]
    fn test_parse_author_name() {
        let author_string = b"Max Carter-Brown";
        let (_, y) = parse_author_name(author_string).unwrap();
        let res: Vec<&str> = y.iter().map(|e| std::str::from_utf8(e).unwrap()).collect();

        assert_eq!(vec!["Max", "Carter-Brown"], res)
    }

    #[test]
    fn test_parse_author_line() {
        let author_string = b"%A Max Carter-Brown";
        let (_, parsed) = parse_author_line(author_string).unwrap();

        assert_eq!(parsed.first, "Max");
        assert_eq!(parsed.last, "Carter-Brown");
    }

    #[test]
    fn test_keywords_line() {
        let keywords = b"%K keyword another word";
        let (_, parsed) = parse_keywords_line(keywords).unwrap();
        assert_eq!(
            parsed,
            &[
                "keyword".as_bytes(),
                "another".as_bytes(),
                "word".as_bytes()
            ]
        )
    }
}
