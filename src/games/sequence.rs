use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="seq-game-type", rename_all="kebab-case")]
pub enum SequenceGame {
    Text(TextSequence)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TextSequence {
    clue1: String,
    clue2: String,
    clue3: String,
    clue4: String,
    clue5: String,
    answer: String,
}

#[derive(Debug, Clone)]
pub enum SequenceState {
    Text(TextSequenceState)
}

#[derive(Debug, Clone)]
pub enum TextSequenceState {
    Clue1,
    Clue2,
    Clue3,
    Clue4,
    Clue5,
    Answered,
}
