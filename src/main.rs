// OPTIONS:
// - ask for options on the cli (e.g. authors, title, etc)

// commands:
// add
// edit
// delete

// set up $HOME/.refer/ directory

use refer_parse::Reader;
use refer_parse::Writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = Reader::<&[u8]>::from_path("./assets/test.refer")?;
    let mut reader =
        Reader::new("\n\n\n\n%A Molly Carter-Brown\n%B An amazing book\n\n".as_bytes());

    for record in reader.records() {
        eprintln!("{:#?}", record?);
    }

    let mut writer = Writer::new(Vec::new());
    // entry 1
    writer.write_record(vec![
        "%A Author three".as_bytes(),
        "%A Author four".as_bytes(),
        "%B Time and tide".as_bytes(),
        "%K keyone keytwo keythree".as_bytes(),
        "%V 123".as_bytes(),
    ])?;
    // entry 2
    writer.write_record(vec!["%A Author one".as_bytes(), "%A Author two".as_bytes()])?;

    println!(
        "{}",
        std::str::from_utf8(&writer.wtr.into_inner()?).unwrap()
    );

    Ok(())
}
