use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="link-game-type", rename_all="kebab-case")]
pub enum LinkGame {
    Text(TextLink)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TextLink {
    clue1: String,
    clue2: String,
    clue3: String,
    clue4: String,
    answer: String,
}

#[derive(Debug, Clone)]
pub enum LinkState {
    Text(TextLinkState)
}

#[derive(Debug, Clone)]
pub enum TextLinkState {
    Clue1,
    Clue2,
    Clue3,
    Clue4,
    NoCluesLeft,
    Answered,
}
