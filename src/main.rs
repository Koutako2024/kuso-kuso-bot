use dotenvy::dotenv;
use kuso_kuso_bot::markov::Markov;
use poise::serenity_prelude::{self as serenity, ChannelType, GetMessages, Http};
use poise::serenity_prelude::{GuildId, Message, UserId};
use std::env;
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use std::thread::sleep;
use std::time::Duration;

// User data, which is stored and accessible in all command invocations
struct Data {
    generator: Markov<'static>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main()]
async fn main() -> () {
    load2text_file().await;
    // serve_cli();
    serve_bot().await;
}

fn serve_cli() -> () {
    let filepath = "./data.txt";
    println!("Open {},,,.", filepath);
    let mut f = File::open(filepath).expect("File not found!");
    println!("File has opened successfully!");

    println!("Load content,,,.");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Something went wrong reading file!");
    println!("Content has loaded successfully!");

    println!("Setup markov generator,,,.");
    let generator = Markov::new(&content);
    println!("Finished setup!");
    println!("raw_text: {:?}", &content);
    println!("v2v2cnt: {:?}", generator.v2v2cnt);

    println!("Start generating.");
    let duration = Duration::from_millis(500);
    loop {
        println!("{}", generator.generate());
        sleep(duration);
    }
}

async fn load2text_file() -> () {
    dotenv().unwrap();
    let temp = env::var("DISCORD_TOKEN").unwrap();
    let http = poise::serenity_prelude::Http::new(&temp);
    let messages = fetch_user_messages_in_guild(
        &http,
        GuildId::new(dotenvy::var("DISCORD_GUILD_ID").unwrap().parse().unwrap()),
        UserId::new(
            dotenvy::var("DISCORD_KUSO_BOT_ID")
                .unwrap()
                .parse()
                .unwrap(),
        ),
    );

    let mut writer = BufWriter::new(File::create("./data.txt").unwrap());

    let msgs = messages.await.unwrap();

    for msg in msgs {
        writer.write_all(&msg.content.into_bytes()).unwrap();
        writer.write_all(&"\n".as_bytes()).unwrap();
    }
}

async fn fetch_user_messages_in_guild(
    http: &Http,
    guild_id: GuildId,
    target_user: UserId,
) -> serenity::Result<Vec<Message>> {
    let channels = guild_id.channels(http).await?;

    let mut result = Vec::new();

    for (_, channel) in channels {
        if channel.kind != ChannelType::Text {
            continue;
        }

        let mut before = None;

        loop {
            let mut builder = GetMessages::new().limit(100);

            if let Some(id) = before {
                builder = builder.before(id);
            }

            let messages = channel.id.messages(http, builder).await?;

            if messages.is_empty() {
                break;
            }

            result.extend(
                messages
                    .iter()
                    .filter(|m| m.author.id == target_user)
                    .cloned(),
            );

            before = messages.last().map(|m| m.id);
        }
    }

    Ok(result)
}

async fn serve_bot() -> () {
    dotenv().expect(".env file not found"); // load .env
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    // setup markov
    let filepath = "./data.txt";
    println!("Open {},,,.", filepath);
    let mut f = File::open(filepath).expect("File not found!");
    println!("File has opened successfully!");

    println!("Load content,,,.");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("Something went wrong reading file!");
    println!("Content has loaded successfully!");

    println!("Setup markov generator,,,.");
    let static_content: &'static str = Box::leak(content.into_boxed_str());
    let generator = Markov::new(static_content);
    println!("Finished setup!");
    println!("raw_text: {:?}", static_content);
    println!("v2v2cnt: {:?}", generator.v2v2cnt);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![kusokuso()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { generator })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

/// クソクソbotが口をきいてくれます。
#[poise::command(slash_command, prefix_command)]
async fn kusokuso(
    ctx: Context<'_>,
    #[description = "回数"] time: Option<u32>,
) -> Result<(), Error> {
    for _ in 0..time.unwrap_or(1) {
        let generated = ctx.data().generator.generate();
        ctx.say(generated).await?;
    }
    Ok(())
}
