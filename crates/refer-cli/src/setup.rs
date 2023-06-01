// set up the file system and the database file

use crate::{ReferError, ReferErrorKind, ReferResult};
use std::{env, fs, io::Write};

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

    // create the config file
    let mut config_default = refer_directory.clone();
    config_default.push("rc.toml");
    match fs::File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(config_default)
    {
        Ok(mut f) => {
            // add defaults to file here
            f.write_all(b"# rc configuration file")?;
            // nano default editor
            f.write_all(b"editor = \"nano\"")?;
            // TODO: what else we want to add to the rc.toml?
        }
        Err(e) => eprintln!("Warning: for ~/.refer/rc.toml - {}", e),
    }

    // now create the database file
    let mut database_default = refer_directory.clone();
    database_default.push("bib.refer");
    match fs::File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(database_default)
    {
        Ok(_) => (),
        Err(e) => eprintln!("Warning: for ~/.refer/bib.refer - {}", e),
    }

    // and set the environmental variable to the database location
    let key = "REFER_DATABASE";
    env::set_var(key, refer_directory);

    Ok(())
}
