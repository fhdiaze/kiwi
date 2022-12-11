use serde::Serialize;

#[derive(Serialize)]
pub struct Problem {
    pub status: u16,
    #[serde(rename = "type")]
    pub typo: String,
    pub title: String,
    pub detail: String,
}

impl Problem {
    pub fn new(status: u16, typo: String, title: String, detail: String) -> Self {
        Problem {
            status,
            typo,
            title,
            detail,
        }
    }
}
