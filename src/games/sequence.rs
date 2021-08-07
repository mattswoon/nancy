use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display, self};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="sequence-game-type", rename_all="kebab-case")]
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

impl Display for TextSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "clue1:\t{}\nclue2:\t{}\nclue3:\t{}\nclue4:\t{}\nclue5:\t{}\nanswer:\t{}\n",
               self.clue1, self.clue2, self.clue3, self.clue4, self.clue5, self.answer)
    }
}
