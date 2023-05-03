use crate::{
    error::{Error, ErrorKind, Result},
    record::{Author, Record, RecordType},
};

pub struct StyleBuilder {
    inner: Record,
    typ: Style,
}

impl StyleBuilder {
    pub fn new(rec: Record) -> Self {
        StyleBuilder {
            inner: rec,
            typ: Style::default(),
        }
    }

    pub fn set_style(&mut self, style: Style) {
        self.typ = style;
    }

    pub fn format(&self) -> Result<String> {
        match self.typ {
            Style::Apa => todo!(),
            Style::Chicago => todo!(),
            Style::Harvard => match self.inner.record_type()? {
                RecordType::None => Err(Error::new(ErrorKind::RecordType(
                    "Calling format on a record which does not have a record type.".into(),
                ))),
                RecordType::Book => {
                    let record = &self.inner;
                    // <authors> <(date)> <title> <place>: <publisher> <series> <volume number>
                    // add the authors
                    let authors =
                        record
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
                    let mut a = authors.strip_suffix("and ").unwrap().to_string();
                    // add the date if there is one
                    if let Some(d) = &record.date {
                        a.push('(');
                        a.push_str(d);
                        a.push_str(") ");
                    }
                    // add the title
                    if let Some(t) = &record.title {
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
                RecordType::Journal => todo!(),
            },
        }
    }
}

#[derive(Default)]
pub enum Style {
    Apa,
    Chicago,
    #[default]
    Harvard,
}
