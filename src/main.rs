// OPTIONS:
// - ask for options on the cli (e.g. authors, title, etc)

// commands:
// add
// edit
// delete

// set up $HOME/.refer/ directory

use refer::Reader;
use refer::StyleBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::new(
        "%A Brown, M.\n%A Carter M.\n%B a book\n%D 1972\n%C Chicago\n%T Our greatest book\n%I Oxford University Press\n%V volume 10"
            .as_bytes(),
    );

    for record in reader.records() {
        let rec = record?;
        eprintln!("{}", &rec);
        eprintln!("{}", StyleBuilder::new(rec).format()?);
    }

    Ok(())
}
