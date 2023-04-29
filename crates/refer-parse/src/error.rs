use nom::error::{ErrorKind, ParseError};

#[derive(Debug, Clone)]
pub struct ReferParseError {
    message: String,
}

impl ParseError<&[u8]> for ReferParseError {
    // on one line, we show the error code and the input that caused it
    fn from_error_kind(input: &[u8], kind: ErrorKind) -> Self {
        let message = format!("{:?}: {:?}  ", kind, std::str::from_utf8(input).unwrap());
        // println!("{}", message);
        ReferParseError { message }
    }

    // if combining multiple errors, we show them one after the other
    fn append(input: &[u8], kind: ErrorKind, other: Self) -> Self {
        let message = format!(
            "{}{:?}: {:?}  ",
            other.message,
            kind,
            std::str::from_utf8(input).unwrap()
        );
        // println!("{}", message);
        ReferParseError { message }
    }

    fn from_char(input: &[u8], c: char) -> Self {
        let message = format!("'{}': {:?}  ", c, std::str::from_utf8(input).unwrap());
        // println!("{}", message);
        ReferParseError { message }
    }
}
