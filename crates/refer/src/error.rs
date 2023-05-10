use std::{error::Error as StdError, fmt, io, result, str};

// I'm aware the error handling is slightly mad
// first time I have done a lib like this.

/// A type alias for `Result<T, refer::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can happen when processing refer data.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// A crate private constructor for `Error`.
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of error that can occur.
#[derive(Debug)]
pub enum ErrorKind {
    /// On any of the I/O things that can go wrong
    Io(io::Error),
    /// Any UTF-8 funny business
    Utf8(str::Utf8Error),
    /// As a result of using nom
    NomError(String),
    /// If a tag is not found
    TagNotFound(String),
    /// If the author parsing goes awry
    Author(String),
    /// If the keyword parsing goes awry
    Keyword(String),
    /// For fetching the type of the record
    RecordType(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(ErrorKind::Io(err))
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Io(ref err) => err.fmt(f),
            ErrorKind::Utf8(ref err) => {
                write!(f, "refer invalid UTF-8 error: {}", err)
            }
            ErrorKind::NomError(ref err) => {
                write!(f, "refer parsing error using nom: {:?}", err)
            }
            ErrorKind::TagNotFound(ref err) => {
                write!(
                    f,
                    "the tag used ({}) is not in the refer specification.",
                    err
                )
            }
            ErrorKind::Author(ref err) => {
                write!(f, "Author field incorrectly specified: {}", err)
            }
            ErrorKind::Keyword(ref err) => {
                write!(f, "Keyword field incorrectly specified: {}", err)
            }
            ErrorKind::RecordType(ref err) => {
                write!(
                    f,
                    "Record type cannot be both book and journal, or neither: {}",
                    err
                )
            }
        }
    }
}
