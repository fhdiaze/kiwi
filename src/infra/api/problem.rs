use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct Problem {
    pub status: u16,
    #[serde(rename="type")]
    pub kind: String,
    pub title: String,
    pub detail: String,
}

#[derive(Serialize)]
pub enum Kind {
    NotFound,
    BadRequest,
    InternalServerError,
}

impl Problem {
    pub fn new(status: u16, kind: String, title: String, detail: String) -> Self {
        Problem {
            status,
            kind,
            title,
            detail,
        }
    }

    pub fn from_type(kind: Kind, title: String, detail: String) -> Self {
        Self::new(kind.status(), kind.kind().to_owned(), title, detail)
    }
}

impl Kind {
    fn status(&self) -> u16 {
        match self {
            Kind::NotFound => StatusCode::NOT_FOUND.as_u16(),
            Kind::BadRequest => StatusCode::BAD_REQUEST.as_u16(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    fn title(&self) -> &str {
        match self {
            Kind::NotFound => StatusCode::NOT_FOUND.canonical_reason().unwrap(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.canonical_reason().unwrap(),
        }
    }

    fn kind(&self) -> &str {
        match self {
            Kind::NotFound => "https://tools.ietf.org/html/rfc7231#section-6.5.4",
            _ => "https://tools.ietf.org/html/rfc7231#section-6.6.1",
        }
    }
}
