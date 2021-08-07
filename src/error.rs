use std::fmt::{Formatter, Display, self};
use crate::data::ResponseErr;

#[derive(Debug, Clone)]
pub enum Error {
    NoState,
    NoMainChannel,
    NoGamesLeft,
    NoGamePlaying,
    NotFinishedPlayingYet,
    NoCluesToShow,
    NothingToReveal,
    ArgError(String),
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
            Error::NoGamePlaying =>
                write!(f, "A game hasn't been queued"),
            Error::NoGamesLeft => 
                write!(f, "There are no games left, try adding some more"),
            Error::ArgError(s) =>
                write!(f, "Couldn't parse an argument: {}", s),
            Error::NotFinishedPlayingYet =>
                write!(f, "Haven't finished the current game yet"),
            Error::NoCluesToShow  =>
                write!(f, "No clues to show"),
            Error::NothingToReveal => 
                write!(f, "Nothing to reveal"),
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
