use crate::{default_refer_location, ReferEditor, ReferResult};
use inquire::{Editor, Text};
use refer::{Reader, Writer};
use std::{
    ffi::OsString,
    fs::{File, OpenOptions},
};

pub fn add_rc(
    journal: bool,
    book: bool,
    string: Option<String>,
    editor: bool,
    editor_exec: ReferEditor,
) -> ReferResult<()> {
    let default_location = default_refer_location()?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(default_location)?;
    let writer = Writer::new(file);

    if let Some(cli_string) = string {
        write_from_cli_or_editor_string(cli_string, writer)?;
    } else if journal {
        write_journal_record(writer)?;
    } else if book {
        // TODO: implement this.
        todo!()
    } else if editor {
        let ee = editor_exec.to_string();
        let edited_string = Editor::new("Database entry")
            .with_editor_command(&OsString::from(ee))
            .prompt()?;

        write_from_cli_or_editor_string(edited_string, writer)?;
    }

    Ok(())
}

fn write_from_cli_or_editor_string(string: String, mut writer: Writer<File>) -> ReferResult<()> {
    let mut reader = Reader::new(string.as_bytes());

    for result in reader.records() {
        let record = result?;
        let checked_record_string = record.to_string();
        writer.write_record(vec![checked_record_string])?;
    }

    writer.flush()?;
    eprintln!("Successfully added to database from cli/editor string.");

    Ok(())
}

fn write_journal_record(mut writer: Writer<File>) -> ReferResult<()> {
    eprintln!("Add a journal article to the database:");
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
    writer.flush()?;

    Ok(())
}

fn write_book_record(mut writer: Writer<File>) {
    todo!()
}
