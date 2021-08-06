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
        ResponseErr,
        Respondable,
        OrSend,
    },
    games::game::Game,
    error::Error,
};

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .read(|s| {
            let host_name = s.host
                .as_ref()
                .map(|u| u.name.clone())
                .unwrap_or("NOTSET".to_string());
            ResponseOk::new(ctx, msg)
                .with_content(format!("```\nHost: {}\n```", host_name))
        })
        .await
        .send()
        .await
}

#[command]
#[aliases("become-host")]
async fn become_host(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .write(|s| {
            let host = &msg.author;
            s.set_host(&host);
            log::info!("{} has become the host", &host);
            ResponseOk::new(ctx, msg)
                .with_content(format!("I made {} the host", host.name))
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
#[aliases("play")]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    Executor::new(ctx, msg)
        .try_write_and_get(|s| s.queue_game())
        .await
        .send()
        .await?;
    Ok(())
}


#[group]
#[commands(status, become_host, add_game, play)]
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
        .configure(|c| c.prefix("!"))
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
