use std::fmt::{Formatter, Display, self};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="link-game-type", rename_all="kebab-case")]
pub enum LinkGame {
    Text(TextLink)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub struct TextLink {
    pub clue1: String,
    pub clue2: String,
    pub clue3: String,
    pub clue4: String,
    pub answer: String,
}

impl TextLink {
    pub fn verbatim(&self) -> String {
        format!("```\nclue1:\t{}\nclue2:\t{}\nclue3:\t{}\nclue4:\t{}\nanswer:\t{}\n```",
                self.clue1, self.clue2, self.clue3, self.clue4, self.answer)
    }
}

impl Display for TextLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "clue1:\t{}\nclue2:\t{}\nclue3:\t{}\nclue4:\t{}\nanswer:\t{}\n",
               self.clue1, self.clue2, self.clue3, self.clue4, self.answer)
    }
}
