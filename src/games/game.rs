use std::fmt::{Formatter, Display, self};
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
#[serde(rename_all="kebab-case")]
pub struct Game {
    pub submitted_by: String,
    #[serde(flatten)]
    pub game: GameType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag="game-type", rename_all="kebab-case")]
pub enum GameType {
    Sequence(SequenceGame),
    Link(LinkGame),
}

#[derive(Clone, Debug)]
pub enum GameState {
    Ready,
    Clue(i64),
    NoCluesLeft,
    Answered,
}


#[derive(Debug, Clone)]
pub struct PlayingGame {
    pub game: Game,
    pub state: GameState,
}

impl PlayingGame {
    pub fn new(game: Game) -> Self {
        PlayingGame {
            game,
            state: GameState::Ready,
        }
    }

    pub fn with_state(self, state: GameState) -> Self {
        PlayingGame {
            state,
            ..self
        }
    }

    pub fn next_clue(self) -> (Option<String>, GameState) {
        match &self.state {
            GameState::Ready => {
                match self.game.game {
                    GameType::Link(LinkGame::Text(g)) => (Some(format!("1.\t{}\n", g.clue1.replace("\n", "\n\t\t"))), GameState::Clue(1)),
                    GameType::Sequence(SequenceGame::Text(g)) => (Some(format!("1.\t{}\n", g.clue1.replace("\n", "\n\t\t"))), GameState::Clue(1)),
                }
            },
            GameState::Clue(i) => {
                match self.game.game {
                    GameType::Link(LinkGame::Text(g)) => {
                        match &i {
                            1 => (Some(format!("1.\t{}\n2.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"))), GameState::Clue(2)),
                            2 => (Some(format!("1.\t{}\n2.\t{}\n3.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"), g.clue3.replace("\n", "\n\t\t"))), GameState::Clue(3)),
                            3 => (Some(format!("1.\t{}\n2.\t{}\n3.\t{}\n4.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"), g.clue3.replace("\n", "\n\t\t"), g.clue4.replace("\n", "\n\t\t"))), GameState::NoCluesLeft),
                            _ => (None, GameState::NoCluesLeft),
                        }
                    },
                    GameType::Sequence(SequenceGame::Text(g)) => {
                        match &i {
                            1 => (Some(format!("1.\t{}\n2.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"))), GameState::Clue(2)),
                            2 => (Some(format!("1.\t{}\n2.\t{}\n3.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"), g.clue3.replace("\n", "\n\t\t"))), GameState::Clue(3)),
                            3 => (Some(format!("1.\t{}\n2.\t{}\n3.\t{}\n4.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"), g.clue3.replace("\n", "\n\t\t"), g.clue4.replace("\n", "\n\t\t"))), GameState::NoCluesLeft),
                            4 => (Some(format!("1.\t{}\n2.\t{}\n3.\t{}\n4.\t{}\n5.\t{}\n", g.clue1.replace("\n", "\n\t\t"), g.clue2.replace("\n", "\n\t\t"), g.clue3.replace("\n", "\n\t\t"), g.clue4.replace("\n", "\n\t\t"), g.clue5.replace("\n", "\n\t\t"))), GameState::NoCluesLeft),
                            _ => (None, GameState::NoCluesLeft),
                        }
                    },
                }
            },
            _ => (None, self.state)
        }
    }

    pub fn reveal(self) -> (String, GameState) {
        let submitted_by = self.game.submitted_by.clone();
        match self.game.game {
            GameType::Link(LinkGame::Text(g)) => (format!("Submitted by: {}\n\n{}", submitted_by, g), GameState::Answered),
            GameType::Sequence(SequenceGame::Text(g)) => (format!("Submitted by: {}\n\n{}", submitted_by, g), GameState::Answered),
        }
    }
}

impl Display for GameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameType::Link(LinkGame::Text(g)) => 
                write!(f, "{}", g),
            GameType::Sequence(SequenceGame::Text(g)) =>
                write!(f, "{}", g),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Submitted by: {}\n\n{}", self.submitted_by, self.game)
    }
}
