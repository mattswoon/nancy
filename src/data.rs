use serenity::{
    prelude::TypeMapKey,
    client::Context,
    framework::standard::CommandResult,
    model::{
        user::User,
        channel::{
            GuildChannel,
            Message,
            PrivateChannel,
        }
    }
};
use async_trait::async_trait;
use crate::{
    games::{
        game::{
            Game,
            PlayingGame,
        },
    },
    error::Error,
};

impl TypeMapKey for State {
    type Value = State;
}

#[derive(Debug)]
pub struct State {
    pub host: Option<User>,
    pub main_channel: Option<GuildChannel>,
    pub games: Vec<Game>,
    pub playing: Option<PlayingGame>
}

impl State {
    pub fn new() -> State {
        State {
            host: None,
            main_channel: None,
            games: vec![],
            playing: None,
        }
    }
    
    pub fn with_games(self, games: Vec<Game>) -> State {
        State {
            games,
            ..self
        }
    }

    pub fn set_host(&mut self, user: &User) {
        self.host = Some(user.clone());
    }

    pub fn add_game(&mut self, game: Game) {
        self.games.push(game);
    }

    pub fn queue_game(&mut self) -> Result<(), Error> {
        let game = self.games.pop()
            .ok_or(Error::NoGamesLeft)?;
        self.playing = Some(PlayingGame::new(game));
        Ok(())
    }

    pub fn next_clue(&mut self) -> Result<String, Error> {
        let (clue, playing) = self.playing
            .as_ref()
            .ok_or(Error::NoGamePlaying)
            .and_then(|p| {
                let (clue, state) = p.clone().next_clue();
                Ok((clue, p.clone().with_state(state)))
            })?;
        self.playing = Some(playing);
        Ok(clue.unwrap_or("".to_string()))
    }
}

pub type RespondableResult<'a> = Result<ResponseOk<'a>, ResponseErr<'a>>;
pub type DynRespondable = Box<dyn Respondable>;

#[async_trait]
pub trait Respondable {
    async fn send(self) -> CommandResult;
}

#[derive(Clone)]
pub struct Executor<'a> {
    context: &'a Context,
    message: &'a Message,
}

impl<'a> Executor<'a> {
    pub fn new(context: &'a Context, message: &'a Message) -> Executor<'a> {
        Executor { context, message }
    }

    pub async fn write<F, R>(self, action: F) -> Result<R, ResponseErr<'a>> 
    where
        F: Send + FnOnce(&mut State) -> R,
        R: 'a + Respondable
    {
        self.context
            .data
            .write()
            .await
            .get_mut::<State>()
            .ok_or(ResponseErr::new(self.context, self.message, Error::NoState))
            .map(action)
    }
    
    pub async fn write_and_get<F, T>(self, action: F) -> Result<T, ResponseErr<'a>> 
    where
        F: Send + FnOnce(&mut State) -> T,
    {
        self.context
            .data
            .write()
            .await
            .get_mut::<State>()
            .ok_or(ResponseErr::new(self.context, self.message, Error::NoState))
            .map(action)
    }

    pub async fn try_write<F, R>(self, action: F) -> Result<R, ResponseErr<'a>>
    where
        F: Send + FnOnce(&mut State) -> Result<R, Error>,
        R: 'a + Respondable
    {
        self.context
            .data
            .write()
            .await
            .get_mut::<State>()
            .ok_or(Error::NoState)
            .and_then(action)
            .map_err(|e| ResponseErr::new(self.context, self.message, e))
    }
    
    pub async fn try_write_and_get<F, T>(self, action: F) -> Result<T, ResponseErr<'a>>
    where
        F: Send + FnOnce(&mut State) -> Result<T, Error>,
    {
        self.context
            .data
            .write()
            .await
            .get_mut::<State>()
            .ok_or(Error::NoState)
            .and_then(action)
            .map_err(|e| ResponseErr::new(self.context, self.message, e))
    }
    
    pub async fn read<F, R>(&self, action: F) -> Result<R, ResponseErr<'a>> 
    where
        F: Send + Fn(&State) -> R,
        R: 'a + Respondable,
    {
        self.context
            .data
            .read()
            .await
            .get::<State>()
            .ok_or(ResponseErr::new(self.context, self.message, Error::NoState))
            .map(action)
    }

    pub async fn get<T, F>(&self, action: F) -> Result<T, ResponseErr<'a>> 
    where
        F: Send + Fn(&State) -> T,
    {
        self.context
            .data
            .read()
            .await
            .get::<State>()
            .ok_or(ResponseErr::new(self.context, self.message, Error::NoState))
            .map(action)
    }
    
    pub async fn try_get<T, F>(&self, action: F) -> Result<T, ResponseErr<'a>> 
    where
        F: Send + Fn(&State) -> Result<T, Error>,
    {
        self.context
            .data
            .read()
            .await
            .get::<State>()
            .ok_or(Error::NoState)
            .and_then(action)
            .map_err(|e| ResponseErr::new(self.context, self.message, e))
    }
}

/// If something returns `Result<(), Error>` that means we don't respond
/// in discord, even if it errors
#[async_trait]
impl Respondable for () {
    async fn send(self) -> CommandResult {
        Ok(())
    }
}

#[derive(Clone)]
pub struct ResponseErr<'a> {
    pub context: &'a Context,
    pub message: &'a Message,
    pub error: Error
}

impl<'a> ResponseErr<'a> {
    pub fn new(context: &'a Context, message: &'a Message, error : Error) -> ResponseErr<'a> {
        ResponseErr {
            context,
            message,
            error
        }
    }
}

pub struct ResponseOk<'a> {
    pub context: &'a Context,
    pub message: &'a Message,
    pub channel: Option<&'a GuildChannel>,
    pub dm_channel: Option<&'a PrivateChannel>,
    pub react: Option<char>,
    pub content: Option<String>,
}

impl<'a> ResponseOk<'a> {
    pub fn new(context: &'a Context, message: &'a Message) -> ResponseOk<'a> {
        ResponseOk {
            context, 
            message, 
            channel: None,
            dm_channel: None,
            react: None,
            content: None,
        }
    }

    pub fn with_channel(self, channel: &'a GuildChannel) -> ResponseOk<'a> {
        ResponseOk{
            channel: Some(channel),
            ..self
        }
    }

    pub fn with_dm_channel(self, dm_channel: &'a PrivateChannel) -> ResponseOk<'a> {
        ResponseOk {
            dm_channel: Some(dm_channel),
            ..self
        }
    }

    pub fn with_react(self, react: char) -> ResponseOk<'a> {
        ResponseOk{
            react: Some(react),
            ..self
        }
    }
    
    pub fn with_content(self, content: String) -> ResponseOk<'a> {
        ResponseOk{
            content: Some(content),
            ..self
        }
    }
}

#[async_trait]
impl<'a> Respondable for ResponseOk<'a> {
    async fn send(self) -> CommandResult {
        if let Some(r) = self.react {
            self.message.react(self.context, r).await?;
        }
        if let Some(text) = self.content {
            if let Some(chan) = self.channel {
                chan.send_message(self.context, |m| m.content(&text))
                    .await?;
            }
            if let Some(dm_chan) = self.dm_channel {
                dm_chan.send_message(self.context, |m| m.content(&text))
                    .await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<'a> Respondable for ResponseErr<'a> {
    async fn send(self) -> CommandResult {
        log::warn!("{}", self.error);
        self.message.reply(self.context, self.error)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<T: Respondable + Send, E: Respondable + Send> Respondable for Result<T, E> {
    async fn send(self) -> CommandResult {
        match self {
            Ok(o) => o.send(),
            Err(e) => e.send(),
        }.await
    }
}

#[async_trait]
pub trait OrSend {
    type OkType;

    async fn or_send(self) -> CommandResult<Self::OkType>;
}

#[async_trait]
impl<T> OrSend for Result<T, ResponseErr<'_>> 
where 
    T: Send
{
    type OkType = T;

    async fn or_send(self) -> CommandResult<T> {
        match self {
            Ok(o) => Ok(o),
            Err(e) => {
                let err = e.error.clone();
                e.send().await?;
                Err(Box::new(err))
            }
        }
    }
}
