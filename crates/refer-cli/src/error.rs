use std::{error::Error as StdError, fmt, io, result, str};

pub type ReferResult<T> = result::Result<T, ReferError>;

#[derive(Debug)]
pub struct ReferError(Box<ReferErrorKind>);

impl ReferError {
    /// A crate private constructor for `Error`.
    pub(crate) fn new(kind: ReferErrorKind) -> ReferError {
        ReferError(Box::new(kind))
    }

    /// Return the specific type of this error.
    pub fn kind(&self) -> &ReferErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ReferErrorKind {
        *self.0
    }
}

#[derive(Debug)]
pub enum ReferErrorKind {
    /// On any of the I/O things that can go wrong
    Io(io::Error),
    /// Any UTF-8 funny business
    Utf8(str::Utf8Error),
}

impl From<io::Error> for ReferError {
    fn from(err: io::Error) -> ReferError {
        ReferError::new(ReferErrorKind::Io(err))
    }
}

impl From<ReferError> for io::Error {
    fn from(err: ReferError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl StdError for ReferError {}

impl fmt::Display for ReferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ReferErrorKind::Io(ref err) => err.fmt(f),
            ReferErrorKind::Utf8(ref err) => {
                write!(f, "refer invalid UTF-8 error: {}", err)
            }
        }
    }
}
