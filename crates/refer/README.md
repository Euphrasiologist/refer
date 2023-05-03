# `refer`

This crate provides the basics for parsing and writing <a href="https://manpages.ubuntu.com/manpages/focal/en/man1/refer.1.html">`refer`</a> files.

It intends to follow the specification, but may be narrower in scope eventually.

## Usage

Add this to your `Cargo.toml` file: 

```toml
[dependencies]
refer = "0.1.1"
```

## Example

Read a `refer` file from stdin, and print to stdout.

```rust
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

There's also a `refer::Writer` struct for writing `refer` files.