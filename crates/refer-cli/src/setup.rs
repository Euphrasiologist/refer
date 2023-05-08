// set up the file system and the database file

use crate::{ReferError, ReferErrorKind, ReferResult};
use std::{env, fs};

pub fn setup_rc() -> ReferResult<()> {
    let mut refer_directory = match home::home_dir() {
        Some(h) => h,
        None => {
            return Err(ReferError::new(ReferErrorKind::Cli(
                "could not find home directory on this system".into(),
            )))
        }
    };

    // create a hidden directory
    refer_directory.push(".refer");
    match fs::create_dir(refer_directory.clone()) {
        // all good
        Ok(_) => (),
        Err(e) => eprintln!("Warning: for ~/.refer/ - {}", e),
    };

    // now create the database file
    refer_directory.push("bib.refer");
    match fs::File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(refer_directory.clone())
    {
        Ok(_) => (),
        Err(e) => eprintln!("Warning: for ~/.refer/bib.refer - {}", e),
    }

    // and set the environmental variable to the database location
    let key = "REFER_DATABASE";
    env::set_var(key, refer_directory);

    Ok(())
}
