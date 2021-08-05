

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
