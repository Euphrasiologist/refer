use crate::{default_refer_location, ReferResult};
use refer::Reader;
use std::env;

pub fn status_rc() -> ReferResult<()> {
    let default_location = default_refer_location()?;
    let database = env::var("REFER_DATABASE").unwrap_or(default_location);

    let mut reader = Reader::from_path(database)?;
    let mut record_number = 0;

    for _ in reader.records() {
        record_number += 1;
    }

    println!("Number of records: {}", record_number);

    Ok(())
}
