use add::add_rc;
use edit::edit_rc;
use error::{ReferError, ReferErrorKind, ReferResult};
use inquire::{formatter::OptionFormatter, Select};
use refer::{Error as InnerReferError, Reader, StyleBuilder};
use setup::setup_rc;
use status::status_rc;
use std::{ffi::OsString, fmt::Display, fs::File, str::FromStr};
use toml::Table;

mod add;
mod edit;
mod error;
mod setup;
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
    // edit an entry based on keywords/title match
    Edit {
        keywords: Vec<String>,
        all: bool,
    },
    // this just counts records currently
    Status,
    // sets up a database
    Setup,
}

impl AppArgs {
    fn execute(&self) -> ReferResult<()> {
        // evaluate editor used
        // possibly evaluate elsewhere...
        let editor_exec = read_editor()?;
        match self {
            AppArgs::Global { help } => {
                if *help {
                    let print_help = generate_help_rc(VERSION);
                    eprintln!("{}", print_help);
                    Ok(())
                } else {
                    Err(ReferError::new(ReferErrorKind::Cli(
                        "Unknown argument to rc, pass -h or --help to view help".into(),
                    )))
                }
            }
            AppArgs::Add {
                journal,
                book,
                string,
                editor,
            } => {
                let opts = vec![journal, book, &string.is_some(), editor]
                    .into_iter()
                    .count();
                if opts > 1 {
                    return Err(ReferError::new(ReferErrorKind::Cli(
                        "for `rc add`, only one of journal, book, string, or editor may be specifed on the cli"
                            .into(),
                    )));
                }
                add_rc(*journal, *book, string.to_owned(), *editor, editor_exec)
            }
            AppArgs::Edit { keywords, all } => {
                // check cli args here
                if keywords.is_empty() && !all {
                    return Err(ReferError::new(ReferErrorKind::Cli(
                        "`rc edit` must have at least one keyword, or pass the -a flag".into(),
                    )));
                }
                edit_rc(keywords, all, editor_exec)
            }
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
                              - [-j] flag. is a journal
                              - [-b] flag. is a book
                              - [-e] flag. use an editor to add an entry
                              - [-s] option. provide a string as an arg
    rc edit [-a <keywords>]   - edit/remove an entry in the database
                              - [-a] flag. select from all entries
    rc status                 - some stats on the database. Mainly for
                                debugging.
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
        Some("edit") => {
            let kr: Result<Vec<String>, OsString> = args
                .clone()
                .finish()
                .into_iter()
                .map(|e| e.into_string())
                .collect();
            let all = args.contains(["-a", "--all"]);

            match kr {
                Ok(keywords) => {
                    let pargs = AppArgs::Edit { keywords, all };
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

pub enum ReferEditor {
    Nano,
    Helix,
}

impl FromStr for ReferEditor {
    type Err = ReferError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nano" => Ok(Self::Nano),
            "hx" => Ok(Self::Helix),
            _ => Err(ReferError::new(ReferErrorKind::CatchAll(
                "the refer editor can currently only be either nano or helix".into(),
            ))),
        }
    }
}

impl Display for ReferEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferEditor::Nano => write!(f, "nano"),
            ReferEditor::Helix => write!(f, "hx"),
        }
    }
}

fn read_editor() -> ReferResult<ReferEditor> {
    let mut config_path = match home::home_dir() {
        Some(h) => h,
        None => {
            return Err(ReferError::new(ReferErrorKind::Cli(
                "could not find the home directory on this system".into(),
            )))
        }
    };
    config_path.push(".refer");
    config_path.push("rc.toml");

    // open the config
    match std::fs::read_to_string(config_path) {
        Ok(c) => {
            let config = c.parse::<Table>()?;
            // assuming here that the config is written correctly.
            ReferEditor::from_str(config["editor"].as_str().unwrap_or("nano"))
        }
        Err(_) => ReferEditor::from_str("nano"),
    }
}

pub struct CheckedRecord {
    title: String,
    styled: Result<String, InnerReferError>,
}

impl Display for CheckedRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.styled {
            Ok(st) => st.clone(),
            Err(_) => "Could not format this reference".into(),
        };
        write!(f, "{}", s)
    }
}

// from a database of records, return a choice matching the keywords
// return the title of the record as a string
pub fn matches_from_keywords(
    mut reader: Reader<File>,
    keywords: &[String],
) -> ReferResult<CheckedRecord> {
    let choices: Result<Vec<_>, ReferError> = reader
        .records()
        .filter_map(|e| {
            let x = e.as_ref().map(|f| {
                let record = f.to_string().to_ascii_uppercase();
                let mut record_contains = false;
                for kw in keywords {
                    let up_kw = kw.to_ascii_uppercase();
                    if record.contains(&up_kw) {
                        record_contains = true;
                        break;
                    } else {
                        record_contains = false;
                    }
                }

                record_contains
            });
            match x {
                Ok(bool_res) => match bool_res {
                    true => match e {
                        Ok(ele) => Some(Ok(ele)),
                        Err(err) => Some(Err(ReferError::new(ReferErrorKind::ReferParse(err)))),
                    },
                    false => None,
                },
                Err(err) => Some(Err(ReferError::new(ReferErrorKind::CatchAll(format!(
                    "error in parsing a record, {}",
                    err
                ))))),
            }
        })
        .collect();

    let checked_titles: Vec<CheckedRecord> = choices?
        .iter()
        .map(|e| {
            let formatted_record = StyleBuilder::new(e.clone()).format();
            let title = e.title.clone().unwrap_or_else(|| "".into());
            CheckedRecord {
                title,
                styled: formatted_record,
            }
        })
        .filter(|CheckedRecord { title, styled: _ }| !title.is_empty())
        .collect();

    let formatter: OptionFormatter<CheckedRecord> = &|i| {
        //
        let CheckedRecord { title, styled } = i.value;

        match styled {
            Ok(s) => s.clone(),
            // otherwise fall back on the title
            Err(_) => title.clone(),
        }
    };

    let r_selection = Select::new("Title: ", checked_titles)
        .with_formatter(formatter)
        .prompt();

    r_selection.map_err(|err| err.into())
}
