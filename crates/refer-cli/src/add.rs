use crate::{default_refer_location, ReferResult};
use inquire::Text;
use refer::Writer;
use std::fs::OpenOptions;

pub fn add_rc(journal: bool, book: bool, string: Option<String>, editor: bool) -> ReferResult<()> {
    let default_location = default_refer_location()?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(default_location)?;
    let mut writer = Writer::new(file);

    eprintln!("Add a journal article to the database:");
    if journal {
        let mut record = vec![];
        loop {
            let mut author = Text::new("Author").prompt()?;
            if !author.is_empty() {
                author.insert_str(0, "%A ");
                record.push(author);
            }
            let skip = inquire::Confirm::new("Finished authors?").prompt()?;
            if skip {
                break;
            }
        }

        let mut title = Text::new("Title").prompt()?;
        if !title.is_empty() {
            title.insert_str(0, "%T ");
            record.push(title);
        }

        let mut journal = Text::new("Journal").prompt()?;
        if !journal.is_empty() {
            journal.insert_str(0, "%J ");
            record.push(journal);
        }

        let mut volume = Text::new("Volume").prompt()?;
        if !volume.is_empty() {
            volume.insert_str(0, "%V ");
            record.push(volume);
        }

        let mut number = Text::new("Number").prompt()?;
        if !number.is_empty() {
            number.insert_str(0, "%N ");
            record.push(number);
        }

        let mut date = Text::new("Date").prompt()?;
        if !date.is_empty() {
            date.insert_str(0, "%D ");
            record.push(date);
        }

        let mut pages = Text::new("Pages").prompt()?;
        if !pages.is_empty() {
            pages.insert_str(0, "%P ");
            record.push(pages);
        }

        loop {
            let mut keyword = Text::new("Keywords").prompt()?;
            if !keyword.is_empty() {
                keyword.insert_str(0, "%K ");
                record.push(keyword);
            }
            let skip = inquire::Confirm::new("Finished keywords?").prompt()?;
            if skip {
                break;
            }
        }

        eprintln!("Written the record {:?}", record);
        writer.write_record(record)?;
    }

    Ok(())
}
