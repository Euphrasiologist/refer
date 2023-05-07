use crate::{
    error::{Error, ErrorKind, Result},
    record::{Author, Record, RecordType},
};

#[derive(Debug)]
pub struct StyleBuilder {
    /// A record as defined in [Record]
    inner: Record,
    /// The styling to use, default is Harvard.
    typ: Style,
    /// Whether to use the other (%O) field. If true, the other field
    /// is used in journal formatting for 'Available at: DOI/URL'
    use_other_field: bool,
}

impl StyleBuilder {
    pub fn new(rec: Record) -> Self {
        StyleBuilder {
            inner: rec,
            typ: Style::default(),
            use_other_field: true,
        }
    }

    /// Set the style of the [StyleBuilder]. See [Style] for
    /// styles.
    pub fn set_style(&mut self, style: Style) {
        self.typ = style;
    }

    /// Use the '%O' (other) field when printing references
    /// in a specified format.
    pub fn use_other(&mut self) {
        self.use_other_field = true;
    }

    /// Formats a [Record] into a string which can fail if the record type
    /// is not defined on the underlying record.
    pub fn format(&self) -> Result<String> {
        match self.typ {
            Style::Apa => todo!(),
            // very rough implementations
            Style::Harvard => match self.inner.record_type()? {
                RecordType::None => Err(Error::new(ErrorKind::RecordType(
                    "Calling format on a record which does not have a record type.".into(),
                ))),
                // for harvard - how does the book title and normal title work?
                // see https://libguides.ucd.ie/harvardstyle/harvardchapterineditedbook#:~:text=Reference%3A%20Chapter%20Author(s),publication%3A%20Publisher%2C%20page%20range.
                RecordType::Book => {
                    let record = &self.inner;
                    // <authors> <(date)> <title> <place>: <publisher> <series> <volume number>
                    // add the authors
                    let mut a = harvard_author_string(record);
                    // add the date if there is one
                    harvard_date_string(record, &mut a);

                    // add the name of the book
                    if let Some(t) = &record.book {
                        a.push_str(t);
                        a.push_str(". ");
                    }
                    // add place
                    if let Some(p) = &record.place {
                        a.push_str(p);
                    }
                    // add publisher
                    if let Some(p) = &record.issuer {
                        if record.place.is_some() {
                            a.push_str(": ");
                        }
                        a.push_str(p);
                        a.push_str(". ");
                    }
                    // add series
                    if let Some(s) = &record.series {
                        a.push_str(s);
                        if record.volume.is_some() {
                            a.push_str(", ");
                        } else {
                            a.push('.');
                        }
                    }
                    // volume
                    if let Some(v) = &record.volume {
                        a.push_str(v);
                        a.push('.');
                    }

                    Ok(a)
                }
                RecordType::Journal => {
                    let record = &self.inner;
                    let mut a = harvard_author_string(record);
                    harvard_date_string(record, &mut a);

                    // title
                    if let Some(t) = &record.title {
                        a.push_str(t);
                        a.push_str(". ");
                    }
                    // journal
                    if let Some(j) = &record.journal {
                        a.push_str(j);
                        a.push_str(", ");
                    }
                    // volume
                    if let Some(v) = &record.volume {
                        a.push_str(v);
                        // a.push(' ');
                    }
                    // issue
                    if let Some(i) = &record.issue_number {
                        a.push('(');
                        a.push_str(i);
                        a.push_str(") ");
                    }
                    // page
                    if let Some(p) = &record.page_number {
                        a.push_str(p);
                        a.push('.');
                    }

                    Ok(a)
                }
            },
        }
    }
}

#[derive(Default, Debug)]
pub enum Style {
    Apa,
    #[default]
    Harvard,
}

fn harvard_author_string(record: &Record) -> String {
    if record.author.len() > 4 {
        // we are guaranteed to have a first element here.
        let first = record.author.first().unwrap();
        format!("{} {} et al., ", first.last, first.rest)
    } else {
        let authors = record
            .author
            .iter()
            .fold(String::new(), |mut a, Author { last, rest }| {
                a.push_str(last);
                a.push(' ');
                a.push_str(rest);
                a.push_str(" and ");
                a
            });
        // as we always add "and "
        // I guess we allocate here again which is annoying
        authors.strip_suffix("and ").unwrap().to_string()
    }
}

fn harvard_date_string(record: &Record, a: &mut String) {
    if let Some(d) = &record.date {
        a.push('(');
        a.push_str(d);
        a.push_str(") ");
    }
}
