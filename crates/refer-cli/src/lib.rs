// the command line interface and API for the main ../src/main.rs

// core functionality

// add a record
// all added records MUST have at least one keyword
// added records can check existing database for duplicates
// rc add -j (journal record) -b (book record) --- record input from stdin
// rc add -s "%A author1\n%A author2" --- add record from string
// rc add -e --- takes you to an editor (I'll set up helix for this) "/Users/mbrown/.cargo/bin/hx"

// remove a record
// rc remove (keywords)

// edit a record
// rc edit (keywords) --- this might bring back a selection

// count how many records there are in the db.
// rc count

// additional functionality to maybe add later.
// get records filtering on keywords and title
// rc filter

use std::ffi::OsString;

mod error;
mod setup;
use error::ReferResult;

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
    fn execute(&self) {
        match self {
            AppArgs::Global { help } => {
                if *help {
                    let print_help = generate_help(VERSION);
                    eprintln!("{}", print_help);
                } else {
                    eprintln!("Unknown argument to rc, pass -h or --help to view help.")
                }
            }
            AppArgs::Add {
                journal,
                book,
                string,
                editor,
            } => todo!(),
            AppArgs::Remove { keywords } => todo!(),
            AppArgs::Edit { keywords } => todo!(),
            AppArgs::Status => todo!(),
            AppArgs::Setup => todo!(),
        }
    }
}

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn generate_help(version: &str) -> String {
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
        rc setup                  - initialise an empty database
",
        version
    )
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()?.as_deref() {
        Some("add") => {
            let pargs = AppArgs::Add {
                journal: args.contains(["-j", "--journal"]),
                book: args.contains(["-b", "--book"]),
                string: args.opt_value_from_str(["-s", "--string"])?,
                editor: args.contains(["-e", "--editor"]),
            };

            pargs.execute();
            Ok(())
        }
        Some("remove") => {
            let kr: Result<Vec<String>, OsString> =
                args.finish().into_iter().map(|e| e.into_string()).collect();

            match kr {
                Ok(keywords) => {
                    let pargs = AppArgs::Remove { keywords };
                    pargs.execute();
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
                    pargs.execute();
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

            pargs.execute();
            Ok(())
        }
        Some("setup") => {
            let pargs = AppArgs::Setup;
            pargs.execute();
            Ok(())
        }
        Some(_) => Err("unknown subcommand".into()),
        None => {
            let pargs = AppArgs::Global {
                help: args.contains(["-h", "--help"]),
            };
            pargs.execute();
            Ok(())
        }
    }
}
