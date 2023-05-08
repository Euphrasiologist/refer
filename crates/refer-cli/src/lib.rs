use std::ffi::OsString;

mod add;
mod error;
mod setup;
use add::add_rc;
use error::{ReferError, ReferErrorKind, ReferResult};
use setup::setup_rc;
use status::status_rc;
mod status;

#[derive(Debug)]
pub enum AppArgs {
    // global help
    Global {
        help: bool,
    },
    // subcommands
    // add an entry to the refer database
    Add {
        // indicate that the reference to add is a journal
        journal: bool,
        // indicate that the reference to add is a book
        book: bool,
        // parse a string on the cli and enter into db
        string: Option<String>,
        // open an editor (nano, vim, helix...) to add to the db
        editor: bool,
    },
    // remove a
    Remove {
        keywords: Vec<String>,
    },
    Edit {
        keywords: Vec<String>,
    },
    Status,
    Setup,
}

impl AppArgs {
    fn execute(&self) -> ReferResult<()> {
        match self {
            AppArgs::Global { help } => {
                if *help {
                    let print_help = generate_help_rc(VERSION);
                    eprintln!("{}", print_help);
                    Ok(())
                } else {
                    Err(ReferError::new(ReferErrorKind::Cli(
                        "Unknown argument to rc, pass -h or --help to view help.".into(),
                    )))
                }
            }
            AppArgs::Add {
                journal,
                book,
                string,
                editor,
            } => add_rc(*journal, *book, string.to_owned(), *editor),
            AppArgs::Remove { keywords } => todo!(),
            AppArgs::Edit { keywords } => todo!(),
            AppArgs::Status => status_rc(),
            AppArgs::Setup => setup_rc(),
        }
    }
}

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn generate_help_rc(version: &str) -> String {
    format!(
        "\
    rc {}
    Max Brown <euphrasiamax@gmail.com>
    https://github.com/euphrasiologist/refer

    USAGE:
        rc [-h] [subcommand] [options]

        rc add [-jbe -s <string>] - add an entry to the database
                                  - [-j] is a journal
                                  - [-b] is a book
                                  - [-e] use an editor to add an entry
                                  - [-s] provide a string as an arg
        rc remove <keywords>      - remove an entry from the database
        rc edit <keywords>        - edit an entry in the database
        rc status                 - some stats on the database
        rc setup                  - initialise an empty database. Should 
                                    only be run once upon installing.
",
        version
    )
}

pub fn cli() -> ReferResult<()> {
    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()?.as_deref() {
        Some("add") => {
            let pargs = AppArgs::Add {
                journal: args.contains(["-j", "--journal"]),
                book: args.contains(["-b", "--book"]),
                string: args.opt_value_from_str(["-s", "--string"])?,
                editor: args.contains(["-e", "--editor"]),
            };

            pargs.execute()?;
            Ok(())
        }
        Some("remove") => {
            let kr: Result<Vec<String>, OsString> =
                args.finish().into_iter().map(|e| e.into_string()).collect();

            match kr {
                Ok(keywords) => {
                    let pargs = AppArgs::Remove { keywords };
                    pargs.execute()?;
                }
                Err(e) => {
                    eprintln!("Could not convert {:?} into string", e);
                    std::process::exit(1);
                }
            }

            Ok(())
        }
        Some("edit") => {
            let kr: Result<Vec<String>, OsString> =
                args.finish().into_iter().map(|e| e.into_string()).collect();

            match kr {
                Ok(keywords) => {
                    let pargs = AppArgs::Remove { keywords };
                    pargs.execute()?;
                }
                Err(e) => {
                    eprintln!("Could not convert {:?} into string", e);
                    std::process::exit(1);
                }
            }
            Ok(())
        }
        Some("status") => {
            let pargs = AppArgs::Status;

            pargs.execute()?;
            Ok(())
        }
        Some("setup") => {
            let pargs = AppArgs::Setup;
            pargs.execute()?;
            Ok(())
        }
        Some(e) => Err(ReferError::new(error::ReferErrorKind::Cli(format!(
            "\"{}\" is an unknown subcommand",
            e
        )))),
        None => {
            let pargs = AppArgs::Global {
                help: args.contains(["-h", "--help"]),
            };
            pargs.execute()?;
            Ok(())
        }
    }
}

pub fn default_refer_location() -> ReferResult<String> {
    let mut home = match home::home_dir() {
        Some(h) => h,
        None => {
            return Err(ReferError::new(ReferErrorKind::Cli(
                "could not find the home directory on this system".into(),
            )))
        }
    };
    home.push(".refer");
    home.push("bib.refer");
    Ok(home.to_string_lossy().to_string())
}
