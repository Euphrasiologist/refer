use std::fmt::Display;

/// A refer record.
#[derive(Default, Debug, PartialEq)]
pub struct Record {
    // TODO: this should probably be option<Vec<..>>
    /// The author list
    pub author: Vec<Author>,
    /// The name of the book
    pub book: Option<String>,
    /// The place
    pub place: Option<String>,
    /// Date of publication
    pub date: Option<String>,
    /// The editor
    pub editor: Option<String>,
    /// US Government ordering number.
    pub government: Option<String>,
    /// The publisher (issuer)
    pub issuer: Option<String>,
    /// For an article in a journal, the name of the journal.
    pub journal: Option<String>,
    /// Keywords to be used for searching.
    pub keywords: Option<Vec<String>>,
    /// Label.
    pub label: Option<String>,
    // Journal issue number
    pub issue_number: Option<String>,
    /// Page number. A range of pages can be specified as m-n.
    // probably needs to be parsed fully e.g. 1-100
    pub page_number: Option<String>,
    /// Other information. This is usually printed at the end of the reference.
    pub other: Option<String>,
    /// The name of the author, if the author is not a person. This will only be used if there are no %A fields. There can only be one %Q field.
    pub author_np: Option<String>,
    /// Technical report number.
    pub report: Option<String>,
    /// Series name.
    pub series: Option<String>,
    /// Title. For an article in a book or journal, this should be the title of the article.
    pub title: Option<String>,
    /// Volume number of the journal or book.
    pub volume: Option<String>,
    /// Annotation.
    pub annotation: Option<String>,
}

/// The author field needs to be parsed specially as there can be
/// multiple fields, and they are in a specific format
#[derive(Debug, PartialEq)]
pub struct Author {
    /// First name
    pub first: String,
    /// Middle name
    pub middle: Option<String>,
    /// Last name
    pub last: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "%A {}{} {}",
            self.first,
            match &self.middle {
                Some(e) => format!(" {}", e),
                None => "".into(),
            },
            self.last
        )
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // %A line
        match self.author.is_empty() {
            true => write!(f, "")?,
            false => {
                let mut author_list = String::new();
                for el in &self.author {
                    author_list += &el.to_string();
                    author_list += "\n";
                }
                write!(f, "{}", author_list)?
            }
        };

        // %B line
        match &self.book {
            Some(b) => writeln!(f, "%B {}", b)?,
            None => write!(f, "")?,
        };
        // %C line
        match &self.place {
            Some(p) => writeln!(f, "%C {}", p)?,
            None => write!(f, "")?,
        };
        // %D line
        match &self.date {
            Some(d) => writeln!(f, "%D {}", d)?,
            None => write!(f, "")?,
        };
        // %D line
        match &self.editor {
            Some(e) => writeln!(f, "%E {}", e)?,
            None => write!(f, "")?,
        };
        // %G line
        match &self.government {
            Some(g) => writeln!(f, "%G {}", g)?,
            None => write!(f, "")?,
        };
        // %I line
        match &self.issuer {
            Some(i) => writeln!(f, "%I {}", i)?,
            None => write!(f, "")?,
        };
        // %J line
        match &self.journal {
            Some(j) => writeln!(f, "%J {}", j)?,
            None => write!(f, "")?,
        };
        // %K line, all formatted with a space in between
        match &self.keywords {
            Some(k) => {
                let mut keywords = String::new();
                for el in k {
                    keywords += el;
                    keywords += " ";
                }
                keywords.pop();
                writeln!(f, "%K {}", keywords)?
            }
            None => write!(f, "")?,
        };
        // %L line
        match &self.label {
            Some(l) => writeln!(f, "%L {}", l)?,
            None => write!(f, "")?,
        };
        // %N line
        match &self.issue_number {
            Some(i) => writeln!(f, "%N {}", i)?,
            None => write!(f, "")?,
        };
        // %O line
        match &self.other {
            Some(o) => writeln!(f, "%O {}", o)?,
            None => write!(f, "")?,
        };
        // %P line
        match &self.page_number {
            Some(p) => writeln!(f, "%P {}", p)?,
            None => write!(f, "")?,
        };
        // %Q line
        match &self.author_np {
            Some(q) => writeln!(f, "%Q {}", q)?,
            None => write!(f, "")?,
        };
        // %R line
        match &self.report {
            Some(r) => writeln!(f, "%R {}", r)?,
            None => write!(f, "")?,
        };
        // %S line
        match &self.series {
            Some(s) => writeln!(f, "%S {}", s)?,
            None => write!(f, "")?,
        };
        // %T line
        match &self.title {
            Some(t) => writeln!(f, "%T {}", t)?,
            None => write!(f, "")?,
        };
        // %V line
        match &self.volume {
            Some(v) => writeln!(f, "%V {}", v)?,
            None => write!(f, "")?,
        };
        // %X line
        match &self.annotation {
            Some(x) => writeln!(f, "%X {}", x)?,
            None => write!(f, "")?,
        };

        Ok(())
    }
}
