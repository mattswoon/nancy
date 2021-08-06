use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="seq-game-type", rename_all="kebab-case")]
pub enum SequenceGame {
    Text(TextSequence)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TextSequence {
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub clue4: String,
    pub clue5: String,
    pub answer: String,
}
