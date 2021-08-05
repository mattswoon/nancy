use std::fmt::{Formatter, Display, self};
use crate::data::ResponseErr;

#[derive(Debug, Clone)]
pub enum Error {
    NoState,
    NoMainChannel,
    Serde(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoState => 
                write!(f, "No state object in context"),
            Error::NoMainChannel => 
                write!(f, "No main channel set"),
            Error::Serde(e) =>
                write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl<'a> From<ResponseErr<'a>> for Error {
    fn from(e: ResponseErr<'a>) -> Error {
        e.error
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e.to_string())
    }
}
