// OPTIONS:
// - ask for options on the cli (e.g. authors, title, etc)

// commands:
// add
// edit
// delete

// set up $HOME/.refer/ directory

use refer_parse::reader::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = Reader::<&[u8]>::from_path("./assets/test.refer")?;
    // let reader = Reader::new("%A Max Brown\n%B hi\n".as_bytes());

    for record in reader.into_records() {
        eprintln!("{:#?}", record);
    }

    Ok(())
}
