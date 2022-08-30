use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Command {
    name: String,
    date: DateTime<Utc>,
}

pub fn handle(cmd: &Command) -> usize {
    println!("{:?}", cmd);
    3
}
