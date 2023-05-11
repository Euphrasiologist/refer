/*!
The `refer` crate provides a refer bibliographic format reader and writer. There is currently no Serde support.

The primary types in this crate are
[`Reader`](struct.Reader.html)
and
[`Writer`](struct.Writer.html)
for reading and writing refer data.

All records are parsed into the
[`Record`](struct.Record.html)
which contains an exhaustive list of refer format fields.

The errors which can occur when parsing and writing are
described in the
[`Error`](struct.Error.html)
type.

# Example

This example shows the simple API to read in and iterate over
refer records from stdin, and then print the records out.

```no_run
use refer::Reader;
use std::{io, error};

fn main() -> Result<(), Box<dyn error::Error>> {

    // construct a new reader
    let mut reader = Reader::new(io::stdin());

    // iterate over the records (borrowing them)
    for result in reader.records() {
        // records() returns a result
        let record = result?;
        // print the record line to stdout
        println!("{:?}", record);
    }

    Ok(())
}
```
 */

mod error;
mod reader;
mod record;
mod style;
mod writer;

pub use crate::{
    error::Error,
    reader::{Reader, RecordsIntoIter, RecordsIter},
    record::{Author, Record},
    style::{Style, StyleBuilder},
    writer::Writer,
};

fn str_from_utf8(input: &[u8]) -> Result<&str, Error> {
    match std::str::from_utf8(input) {
        Ok(s) => Ok(s),
        Err(e) => Err(error::Error::new(error::ErrorKind::Utf8(e))),
    }
}
