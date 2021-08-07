use std::collections::HashSet;
use serenity::{
    prelude::TypeMap,
    client::{
        Client, 
        Context,
        EventHandler,
    },
    model::{
        channel::Message,
        id::UserId,
    },
    framework::{
        standard::{
            StandardFramework,
            CommandResult,
            Args,
            CommandGroup,
            HelpOptions,
            macros::{
                command,
                group,
                help,
            },
            help_commands::plain,
        }
    }
};
use nancy::{
    data::{
        State,
        Executor,
        ResponseOk,
        ResponseErr,
        Respondable,
        OrSend,
    },
    games::{
        game::Game,
        link::{
            LinkGame,
            TextLink,
        },
        sequence::{
            SequenceGame,
            TextSequence,
        },
    },
    error::Error,
};

/// Prints the number of games Nancy knows about
#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .read(|s| {
            log::info!("Number of games: {}", s.games.len());
            ResponseOk::new(ctx, msg)
                .with_content(format!("Number of games: {}", s.games.len()))
        })
        .await
        .send()
        .await
}

/// Adds a game encoded in json - not a very friendly way of adding games
#[command]
#[only_in("dm")]
#[aliases("add-game")]
async fn add_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let dm_chan = msg.author.create_dm_channel(ctx).await?;
    Executor::new(ctx, msg)
        .try_write(|s| {
            let game: Game = serde_json::from_str(&data)
                .map_err(|e| Error::Serde(format!("{}", e)))?;
            let reply_msg = format!("```\n{:?}\n```", &game);
            s.add_game(game);
            Ok(ResponseOk::new(ctx, msg)
                .with_dm_channel(&dm_chan)
                .with_content(reply_msg)
                .with_react('👍'))
        })
        .await
        .send()
        .await
}

/// Adds a text-based "guess the link" game, expects five arguments (separated by a newline - shift+enter) for the
/// four clues and the answer.
///
/// Eg.
///
///     !add-text-link-game
///     earth
///     air
///     fire
///     water
///     the elements
///
#[command]
#[only_in("dm")]
#[aliases("add-text-link-game")]
async fn add_text_link_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args;
    let clue1: String = args.single()
        .or_else(|_| {
            let m = format!("Could't get clue1, try again");
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue2: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}`, but couldn't get clue2", &clue1);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue3: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}` and `clue2={}`, but couldn't get clue3", &clue1, &clue2);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue4: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}`, `clue2={}` and `clue3={}`, but couldn't get clue4", &clue1, &clue2, &clue3);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let answer: String = args.single()
        .or_else(|_| {
            let m = format!("I got the clues `clue1={}`, `clue2={}`, `clue3={}` and `clue4={}`, but couldn't get the answer", &clue1, &clue2, &clue3, &clue4);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let text_link_game = TextLink { clue1, clue2, clue3, clue4, answer };
    let game = Game::Link(LinkGame::Text(text_link_game));
    Executor::new(ctx, msg)
        .write(|s| {
            let game_str = format!("```\n{}\n```", &game);
            s.add_game(game);
            ResponseOk::new(ctx, msg)
                .with_content(format!("Added game:\n{}", game_str))
        })
        .await
        .send()
        .await
}

/// Adds a text-based "guess the sequence" game, expects six arguments (separated by a newline - shift+enter) for the
/// five clues and the answer.
///
/// Eg.
///
///     !add-text-link-game
///     a
///     e
///     i
///     o
///     u
///     vowels
///
#[command]
#[only_in("dm")]
#[aliases("add-text-sequence-game")]
async fn add_text_sequence_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args;
    let clue1: String = args.single()
        .or_else(|_| {
            let m = format!("Could't get clue1, try again");
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue2: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}`, but couldn't get clue2", &clue1);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue3: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}` and `clue2={}`, but couldn't get clue3", &clue1, &clue2);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue4: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}`, `clue2={}` and `clue3={}`, but couldn't get clue4", &clue1, &clue2, &clue3);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let clue5: String = args.single()
        .or_else(|_| {
            let m = format!("I got `clue1={}`, `clue2={}`, `clue3={}` and `clue4={}`, but couldn't get clue5", &clue1, &clue2, &clue3, &clue4);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let answer: String = args.single()
        .or_else(|_| {
            let m = format!("I got the clues `clue1={}`, `clue2={}`, `clue3={}`, `clue4={}` and `clue5={}`, but couldn't get the answer", &clue1, &clue2, &clue3, &clue4, &clue5);
            Err(ResponseErr::new(ctx, msg, Error::ArgError(m)))
        })
        .or_send()
        .await?;
    let text_seq_game = TextSequence { clue1, clue2, clue3, clue4, clue5, answer };
    let game = Game::Sequence(SequenceGame::Text(text_seq_game));
    Executor::new(ctx, msg)
        .write(|s| {
            let game_str = format!("```\n{}\n```", &game);
            s.add_game(game);
            ResponseOk::new(ctx, msg)
                .with_content(format!("Added game:\n{}", game_str))
        })
        .await
        .send()
        .await
}

/// Pulls a game out of the queue and starts a round with the first clue. To get the next clue
/// use `!next-clue`, or if you think you know the answer, use `!reveal` to show all the clues
/// and the answer (hidden with spoiler tags).
#[command]
#[aliases("play")]
#[only_in("guild")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let clue = s.queue_game()
                .and_then(|()| s.next_clue())?;
            let game_type = s.playing.as_ref().ok_or(Error::NoGamePlaying)
                .map(|p| match p.game {
                    Game::Link(_) => "Guess what connects the four clues".to_string(),
                    Game::Sequence(_) => "Guess the next element in the sequence (and name the pattern)".to_string(),
                })?;
            Ok(ResponseOk::new(ctx, msg)
               .with_content(format!(
r#"{}

The first clue is

>>> {}"#, game_type, clue)))
        })
        .await
        .send()
        .await
}

/// Gets the next clue in the game. If you think you know the answer, use `!reveal` to reveal
/// all the clues and the answer (hidden by a spoiler tag)
#[command]
#[aliases("next-clue")]
#[only_in("guild")]
async fn next_clue(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let game_type = s.playing.as_ref().ok_or(Error::NoGamePlaying)
                .map(|p| match p.game {
                    Game::Link(_) => "Guess what connects the four clues".to_string(),
                    Game::Sequence(_) => "Guess the next element in the sequence (and name the pattern)".to_string(),
                })?;
            let clue = s.next_clue()?;
            Ok(ResponseOk::new(ctx, msg)
               .with_content(format!(
r#"{}

The clues so far are

>>> {}"# , game_type, clue)))
        })
        .await
        .send()
        .await
}

/// Gets all the clues as well as the answer (hidden by a spoiler tag)
#[command]
#[only_in("guild")]
async fn reveal(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let answer = s.reveal()?;
            Ok(ResponseOk::new(ctx, msg)
               .with_content(format!("{}", answer)))
        })
        .await
        .send()
        .await
}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = plain(context, msg, args, &help_options, groups, owners).await;
    Ok(())
}


#[group]
#[commands(status, add_game, play, add_text_link_game, add_text_sequence_game, next_clue, reveal)]
struct General;

struct Handler;

impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Off)
        .with_module_level("nancy", log::LevelFilter::Info)
        .init()
        .expect("Couldn't init logger");

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Couldn't get discord token");

    let games_dir = std::env::var("NANCY_GAMES")
        .unwrap_or("./games/".to_string());

    let games = std::fs::read_dir(games_dir)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }).map(|r| {
            r.map_err(|e| format!("{}", e))
                .and_then(|e| {
                    std::fs::read_to_string(e.path())
                        .map_err(|e| format!("{}", e))
                        .and_then(|s| serde_json::from_str(&s).map_err(|e| format!("{}", e)))
                })
        }).collect::<Result<Vec<Game>, _>>()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")
                   .delimiter("\n"))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);

    Client::builder(&token)
        .type_map(TypeMap::new())
        .type_map_insert::<State>(State::new()
                                  .with_games(games))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't build client")
        .start()
        .await
        .expect("Couldn't start client")

}
