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
r#"What's the fifth element of the sequence?

    1.  {}
    2.  {}
    3.  {}
    4.  {}
    5.  ||{}||

Answer: ||{}||
"#, self.clue1.replace("\n", "\n\t\t"), 
self.clue2.replace("\n", "\n\t\t"), 
self.clue3.replace("\n", "\n\t\t"), 
self.clue4.replace("\n", "\n\t\t"), 
self.clue5.replace("\n", "\n\t\t"), 
self.answer.replace("\n", "\n\t\t"))
    }
}
