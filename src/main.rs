use dotenvy::dotenv;
use kuso_kuso_bot::markov::Markov;
use log::{debug, error, info, trace, warn};
use poise::serenity_prelude::{
    self as serenity, ChannelType, GetMessages, GuildId, Http, Message, MessageId, UserId,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use systemd_journal_logger::JournalLog;

// User data, which is stored and accessible in all command invocations
struct Data {
    generator: std::sync::Mutex<Markov>,
    discord_kuso_bot_id: UserId,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
struct ToSaveWithJson {
    generator: Markov,
    last_msg_id: Option<MessageId>,
}

#[tokio::main()]
async fn main() {
    dotenv().ok(); // load .env
    JournalLog::new().unwrap().install().unwrap();
    log::set_max_level(
        match env::var("RUST_LOG")
            .unwrap_or("info".to_string())
            .to_lowercase()
            .as_str()
        {
            "off" => log::LevelFilter::Off,
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            others => panic!("RUST_LOG is set unknown log level: {others}"),
        },
    );
    error!("test log. error!");
    warn!("test log. warn!");
    info!("test log. info!");
    debug!("test log. debug!");
    trace!("test log. trace!");
    serve_bot().await;
}

async fn serve_bot() {
    // load env vars.
    let discord_token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN.");
    let discord_guild_id = GuildId::new(
        env::var("DISCORD_GUILD_ID")
            .expect("missing DISCORD_GUILD_ID.")
            .parse()
            .expect("DISCORD_GUILD_ID must be integer."),
    );
    let discord_kuso_bot_id = UserId::new(
        env::var("DISCORD_KUSO_BOT_ID")
            .expect("missing DISCORD_KUSO_BOT_ID.")
            .parse()
            .expect("DISCORD_KUSO_BOT_ID must be integer."),
    );

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut loaded = load_json();
    info!("Finished setup!");
    info!("loaded: {:?}", loaded);

    if let Some(last_msg_id) = loaded.last_msg_id {
        info!("load msgs after {},,,", last_msg_id);
        loaded.last_msg_id = Some(
            load_msgs_after(
                &mut loaded.generator,
                &last_msg_id,
                &discord_token,
                &discord_guild_id,
                &discord_kuso_bot_id,
            )
            .await
            .unwrap(),
        );
    } else {
        info!("load all messages,,,");
        let (loaded_generator, loaded_last_msg_id) =
            load_all_msgs(&discord_token, &discord_guild_id, &discord_kuso_bot_id).await;
        loaded.generator = loaded_generator;
        loaded.last_msg_id = Some(loaded_last_msg_id);
    }
    info!("loaded.");
    info!("saving,,,");
    if let Err(e) = save_json(loaded.clone()) {
        error!("Error while saving. e: {e}");
    } else {
        info!("successfully saved.");
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![kusokuso()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    generator: std::sync::Mutex::new(loaded.generator),
                    discord_kuso_bot_id: discord_kuso_bot_id,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

fn load_json() -> ToSaveWithJson {
    let filepath = "./data.json";
    info!("Open {},,,.", filepath);
    let maybe_loaded: Result<ToSaveWithJson, Error> = (|| {
        let f = File::open(filepath)?;
        info!("File has opened successfully!");
        let hoge: ToSaveWithJson = serde_json::from_reader(BufReader::new(f))?;
        info!("Load JSON successfully!");
        Ok(hoge)
    })();

    if let Err(e) = &maybe_loaded {
        warn!("ignored error while loading json. e: {e}");
    }

    maybe_loaded.unwrap_or_default()
}

fn save_json(to_save_with_json: ToSaveWithJson) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = "./data.json";
    info!("Open {},,,.", filepath);
    let f = File::create(filepath)?;
    serde_json::to_writer(BufWriter::new(f), &to_save_with_json)?;
    Ok(())
}

async fn load_all_msgs(
    discord_token: &String,
    discord_guild_id: &GuildId,
    discord_kuso_bot_id: &UserId,
) -> (Markov, MessageId) {
    let http = Http::new(&discord_token);

    // fetch msgs
    let msgs = fetch_all_user_messages_in_guild(&http, discord_guild_id, discord_kuso_bot_id)
        .await
        .unwrap();

    let mut generator = Markov::new("");
    msgs.iter()
        .for_each(|msg| generator.add(&format!("\n{}\n", msg.content)));

    let last_msg_id = msgs.iter().map(|m| m.id).max().unwrap();

    (generator, last_msg_id)
}

async fn fetch_all_user_messages_in_guild(
    http: &Http,
    guild_id: &GuildId,
    target_user: &UserId,
) -> serenity::Result<Vec<Message>> {
    let channels = guild_id.channels(http).await?;

    let mut result = Vec::new();

    for (_, channel) in channels {
        if channel.kind != ChannelType::Text {
            continue;
        }

        info!("{}", channel.name());

        let mut before = None;

        loop {
            let mut builder = GetMessages::new().limit(100);

            if let Some(id) = before {
                builder = builder.before(id);
            }

            let messages = channel.id.messages(http, builder).await;
            if let Err(e) = messages {
                warn!(
                    "ignored error occurred while loading messages from #{} (id: {}).\nYou should reset ./data.json and restart later. e: \n{}",
                    channel.name(),
                    channel.id,
                    e
                );
                break;
            }
            let messages = messages.unwrap();

            if messages.is_empty() {
                break;
            }

            result.extend(
                messages
                    .iter()
                    .filter(|m| m.author.id == *target_user)
                    .cloned(),
            );

            before = messages.last().map(|m| m.id);
        }
    }

    Ok(result)
}

async fn load_msgs_after(
    generator: &mut Markov,
    after: &MessageId,
    discord_token: &String,
    discord_guild_id: &GuildId,
    discord_kuso_bot_id: &UserId,
) -> serenity::Result<MessageId> {
    let http = Http::new(discord_token);

    let (msgs, last_msg_id) =
        fetch_user_messages_after(&http, discord_guild_id, discord_kuso_bot_id, after).await?;

    msgs.iter()
        .for_each(|m| generator.add(&format!("\n{}\n", m.content)));

    Ok(last_msg_id)
}

async fn fetch_user_messages_after(
    http: &Http,
    guild_id: &GuildId,
    target_user: &UserId,
    after: &MessageId,
) -> serenity::Result<(Vec<Message>, MessageId)> {
    let channels = guild_id.channels(&http).await?;

    let mut newer_msgs = Vec::new();
    let mut last_msg_id: MessageId = after.clone();

    for (_, channel) in channels {
        if channel.kind != ChannelType::Text {
            continue;
        }

        info!("{}", channel.name());

        let mut current_after = after.clone();

        loop {
            let messages = channel
                .id
                .messages(&http, GetMessages::new().after(current_after).limit(100))
                .await;
            if let Err(e) = messages {
                warn!(
                    "ignored error occurred while loading messages from #{} (id: {}).\nYou should reset ./data.json and restart later. e: \n{}",
                    channel.name(),
                    channel.id,
                    e
                );
                break;
            }
            let messages = messages.unwrap();

            if messages.is_empty() {
                break;
            }

            if let Some(max_id) = messages.iter().map(|m| m.id).max() {
                current_after = max_id;
                last_msg_id = last_msg_id.max(current_after);
            }

            newer_msgs.extend(
                messages
                    .iter()
                    .filter(|m| m.author.id == *target_user)
                    .cloned(),
            );

            if messages.len() < 100 {
                break;
            }
        }
    }

    Ok((newer_msgs, last_msg_id))
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message } => {
            if &new_message.author.id == &data.discord_kuso_bot_id {
                match data.generator.lock() {
                    Err(e) => {
                        error!("Error occurred while locking generator in event handler. e: {e}")
                    }
                    Ok(mut unlocked) => {
                        unlocked.add(&format!("\n{}\n", new_message.content));

                        info!("saving new message,,,");
                        let to_save_with_json = ToSaveWithJson {
                            generator: unlocked.clone(),
                            last_msg_id: Some(new_message.id),
                        };
                        if let Err(e) = save_json(to_save_with_json) {
                            error!("Error occurred while saving. e: {e}");
                        } else {
                            info!("saved successfully.");
                        }
                    }
                }
            }
        }

        _ => {}
    }

    Ok(())
}

/// クソクソbotが口をきいてくれます。
#[poise::command(slash_command, prefix_command)]
async fn kusokuso(
    ctx: Context<'_>,
    #[description = "回数"] time: Option<u32>,
) -> Result<(), Error> {
    if let Some(_time) = time {
        if _time > 100 {
            ctx.say("100回までにしてね。").await?;
            return Ok(());
        }
    }

    for _ in 0..time.unwrap_or(1) {
        let message = match ctx.data().generator.lock() {
            Err(e) => {
                error!("Error occurred in kusokuso. e: {e}");
                break;
            }
            Ok(unlocked) => unlocked
                .generate()
                .unwrap_or("うおｗ生成失敗してて草ｗ".to_string()),
        };

        ctx.say(message).await?;
    }

    Ok(())
}
