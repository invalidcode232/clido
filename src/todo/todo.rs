use chrono::{DateTime, Local};

pub struct Todo<'a> {
    todo: &'a str,
    date_added: DateTime<Local>,
}
