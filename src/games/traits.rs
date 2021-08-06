pub trait Playable: std::fmt::Debug + Send + Sync {
    type State;
    type ClueType;
    type AnswerType;

    fn next(&self, state: Self::State) -> (Option<Self::ClueType>, Self::State);

    fn reveal(&self) -> (Option<Self::AnswerType>, Self::State);
}

pub trait PlayState: std::fmt::Debug + Send + Sync {}

pub trait NextClue {
    type State;
    type ClueType;

    fn next_clue(&self, state: Self::State) -> (Option<Self::ClueType>, Self::State);
}

pub trait RevealAnswer {
    type State;
    type AnswerType;

    fn reveal_answer(&self, state: Self::State) -> (Option<Self::AnswerType>, Self::State);
}
