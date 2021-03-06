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

impl Display for TextLink {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
r#"Solve for the link

    1. {}
    2. {}
    3. {}
    4. {}

Answer: ||{}||
"#, self.clue1.replace("\n", "\n\t\t"), 
self.clue2.replace("\n", "\n\t\t"), 
self.clue3.replace("\n", "\n\t\t"), 
self.clue4.replace("\n", "\n\t\t"), 
self.answer.replace("\n", "\n\t\t"))
    }
}
