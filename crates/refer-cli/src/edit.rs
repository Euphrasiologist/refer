use std::ffi::{OsStr, OsString};

use crate::{
    default_refer_location, matches_from_keywords, CheckedRecord, ReferEditor, ReferResult,
};
use inquire::{Editor, Select};
use refer::{Error as InnerReferError, Reader};

pub fn edit_rc(keywords: &[String], all: &bool, editor_exec: ReferEditor) -> ReferResult<()> {
    // use the keywords to search the database
    let default_location = default_refer_location()?;
    let mut reader = Reader::from_path(default_location.clone())?;

    if *all {
        // probably should not collect here.
        let titles: Result<Vec<_>, InnerReferError> =
            reader.records().map(|e| e.map(|r| r.title)).collect();
        // choose a title
        let checked_titles = titles?
            .iter()
            .map(|e| e.clone().unwrap_or_else(|| "".into()))
            .filter(|e| !e.is_empty())
            .collect();
        let r_selection = Select::new("Title: ", checked_titles).prompt();
        let selection = r_selection?;

        // find the line number of the match
        let mut line_no = 0;

        let mut reader = Reader::from_path(default_location.clone())?;
        for result in reader.records() {
            let record = result?;
            let tmp_title = record.clone().title.unwrap_or_else(|| "".into());
            // if we hit the title
            if tmp_title == selection {
                let ee = editor_exec.to_string();

                let mut editor = Editor::new("Found reference");
                let selected_record = record.to_string();
                editor.predefined_text = Some(&selected_record);

                let editor_cli = match editor_exec {
                    // nano is like: nano test.tx +10 - i.e. line 10
                    ReferEditor::Nano => vec![
                        OsString::from(format!("+{line_no}")),
                        OsString::from(default_location),
                    ],
                    // helix needs file:line_no
                    ReferEditor::Helix => {
                        vec![OsString::from(&format!("{default_location}:{line_no}"))]
                    }
                };

                let convert: Vec<_> = editor_cli.iter().map(OsStr::new).collect();
                editor.editor_command_args = convert.as_slice();

                let _edited_record = editor.with_editor_command(&OsString::from(ee)).prompt()?;

                // quit the loop here.
                break;
            }
            // +2 because there is a line break between records.
            line_no += &record.to_string().lines().count() + 2;
        }
    } else {
        let CheckedRecord { title, styled: _ } = matches_from_keywords(reader, keywords)?;

        // find the line number of the match
        let mut line_no = 0;

        let mut reader = Reader::from_path(default_location.clone())?;
        for result in reader.records() {
            let record = result?;
            let tmp_title = record.clone().title.unwrap_or_else(|| "".into());
            // if we hit the title
            if tmp_title == title {
                let ee = editor_exec.to_string();

                let mut editor = Editor::new("Found reference");
                let selected_record = record.to_string();
                editor.predefined_text = Some(&selected_record);

                let editor_line_no = match editor_exec {
                    // nano is like: nano test.tx +10 - i.e. line 10
                    ReferEditor::Nano => format!("{default_location} +{line_no}"),
                    // helix needs file:line_no
                    ReferEditor::Helix => format!("{default_location}:{line_no}"),
                };

                let editor_cli = &[OsStr::new(&editor_line_no)];
                editor.editor_command_args = editor_cli;

                let _edited_record = editor.with_editor_command(&OsString::from(ee)).prompt()?;

                // quit the loop here.
                break;
            }
            // +2 because there is a line break between records.
            line_no += &record.to_string().lines().count() + 2;
        }
    }
    Ok(())
}
