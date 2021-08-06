use serenity::{
    prelude::TypeMap,
    client::{
        Client, 
        Context,
        EventHandler,
    },
    model::channel::Message,
    framework::{
        standard::{
            StandardFramework,
            CommandResult,
            Args,
            macros::{
                command,
                group
            },
        }
    }
};
use nancy::{
    data::{
        State,
        Executor,
        ResponseOk,
        Respondable,
    },
    games::{
        game::Game,
        link::{
            LinkGame,
            TextLink,
        },
    },
    error::Error,
};

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

#[command]
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

#[command]
#[only_in("dm")]
#[aliases("parse-text-link")]
async fn parse_text_link(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = args;
    let clue1: String = args.single()?;
    let clue2: String = args.single()?;
    let clue3: String = args.single()?;
    let clue4: String = args.single()?;
    let answer: String = args.single()?;
    let text_link_game = TextLink { clue1, clue2, clue3, clue4, answer };
    let game = Game::Link(LinkGame::Text(text_link_game));
    let as_string = serde_json::to_string_pretty(&game)
        .map_err(|e| Error::Serde(e.to_string()))?;

    msg.reply(
        ctx,
        format!("```\n{}\n```", as_string)
    ).await?;
    Ok(())
}

#[command]
#[aliases("play")]
#[only_in("guild")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let clue = s.queue_game()
                .and_then(|()| s.next_clue())?;
            Ok(ResponseOk::new(ctx, msg)
               .with_content(format!("```\n{}\n```", clue)))
        })
        .await
        .send()
        .await
}

#[command]
#[aliases("next-clue")]
#[only_in("guild")]
async fn next_clue(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let clue = s.next_clue()?;
            Ok(ResponseOk::new(ctx, msg)
                .with_content(format!("```\n{}\n```", clue)))
        })
        .await
        .send()
        .await
}

#[command]
#[only_in("guild")]
async fn reveal(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write(|s| {
            let answer = s.reveal()?;
            Ok(ResponseOk::new(ctx, msg)
               .with_content(format!("```\n{}\n```", answer)))
        })
        .await
        .send()
        .await
}


#[group]
#[commands(status, add_game, play, parse_text_link, next_clue, reveal)]
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
