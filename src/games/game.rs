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
#[serde(tag="game-type", rename_all="kebab-case")]
pub enum Game {
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
                match self.game {
                    Game::Link(LinkGame::Text(g)) => (Some(format!("1. {}\n", g.clue1)), GameState::Clue(1)),
                    Game::Sequence(SequenceGame::Text(g)) => (Some(format!("1. {}\n", g.clue1)), GameState::Clue(1)),
                }
            },
            GameState::Clue(i) => {
                match self.game {
                    Game::Link(LinkGame::Text(g)) => {
                        match &i {
                            1 => (Some(format!("1. {}\n2. {}\n", g.clue1, g.clue2)), GameState::Clue(2)),
                            2 => (Some(format!("1. {}\n2. {}\n3. {}\n", g.clue1, g.clue2, g.clue3)), GameState::Clue(3)),
                            3 => (Some(format!("1. {}\n2. {}\n3. {}\n4. {}\n", g.clue1, g.clue2, g.clue3, g.clue4)), GameState::NoCluesLeft),
                            _ => (None, GameState::NoCluesLeft),
                        }
                    },
                    Game::Sequence(SequenceGame::Text(g)) => {
                        match &i {
                            1 => (Some(format!("1. {}\n2. {}\n", g.clue1, g.clue2)), GameState::Clue(2)),
                            2 => (Some(format!("1. {}\n2. {}\n3. {}\n", g.clue1, g.clue2, g.clue3)), GameState::Clue(3)),
                            3 => (Some(format!("1. {}\n2. {}\n3. {}\n4. {}\n", g.clue1, g.clue2, g.clue3, g.clue4)), GameState::NoCluesLeft),
                            4 => (Some(format!("1. {}\n2. {}\n3. {}\n4. {}\n5. {}\n", g.clue1, g.clue2, g.clue3, g.clue4, g.clue5)), GameState::NoCluesLeft),
                            _ => (None, GameState::NoCluesLeft),
                        }
                    },
                }
            },
            _ => (None, self.state)
        }
    }

    pub fn reveal(self) -> (String, GameState) {
        match self.game {
            Game::Link(LinkGame::Text(g)) => (g.to_string(), GameState::Answered),
            Game::Sequence(SequenceGame::Text(g)) => (g.to_string(), GameState::Answered),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Game::Link(LinkGame::Text(g)) => 
                write!(f, "{}", g),
            Game::Sequence(SequenceGame::Text(g)) =>
                write!(f, "{}", g),
        }
    }
}
