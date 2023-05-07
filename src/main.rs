// OPTIONS:
// - ask for options on the cli (e.g. authors, title, etc)

// commands:
// add
// edit
// delete

// set up $HOME/.refer/ directory

use refer_cli::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match cli() {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {:?}.", e),
    }

    Ok(())
}
