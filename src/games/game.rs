use serde::{Serialize, Deserialize};
use crate::games::{
    sequence::{
        SequenceGame,
    },
    link::{
        LinkGame,
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="game-type", rename_all="kebab-case")]
pub enum Game {
    Sequence(SequenceGame),
    Link(LinkGame),
}
