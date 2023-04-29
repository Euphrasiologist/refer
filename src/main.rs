// OPTIONS:
// - ask for options on the cli (e.g. authors, title, etc)

// commands:
// add
// edit
// delete

// set up $HOME/.refer/ directory

use refer_parse::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = Reader::<&[u8]>::from_path("./assets/test.refer")?;
    let mut reader = Reader::new("\n\n\n\n%A Max Brown\n%A Amos B\n\n".as_bytes());

    for record in reader.records() {
        eprintln!("{:#?}", record?);
    }

    Ok(())
}
