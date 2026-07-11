use axum::routing::post;
use axum::{Json, Router};
use dotenvy::dotenv;
use kuso_kuso_bot::markov::Markov;
use poise::serenity_prelude::{self as serenity, ChannelType, GetMessages, Http};
use poise::serenity_prelude::{GuildId, Message, UserId};
use serde_json::{Value, json};
use std::env;
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use std::thread::sleep;
use std::time::Duration;
use tokio::net::TcpListener;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main()]
async fn main() -> () {
    load2text_file().await;
    serve_cli();
    // serve_bot().await;
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
    // load .env
    dotenv().expect(".env file not found");
    let discord_client_id = env::vars()
        .find(|(key, _)| key == "DISCORD_CLIENT_ID")
        .expect("discord client id not found!")
        .1;
    let discord_client_secret = env::vars()
        .find(|(key, _)| key == "DISCORD_CLIENT_SECRET")
        .expect("discord client secret not found!")
        .1;

    setup_slash_commands(&discord_client_id, &discord_client_secret).await;

    let app = Router::new().route("/discord/interactions", post(handle_interaction));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handle_interaction(Json(payload): Json<Value>) -> Json<Value> {
    let interaction_type = payload["type"].as_i64().unwrap();

    match interaction_type {
        1 => Json(json!({"type": 1})), // ping
        2 => {
            // APPLICATION_COMMAND
            Json(json!({
                "type": 4,
                "data": {
                    "content": "Hello from Rust HTTP bot!"
                }
            }))
        }
        _ => Json(json!({})),
    }
}

async fn setup_slash_commands(discord_client_id: &str, bot_token: &str) -> () {
    let res = reqwest::Client::new()
        .post(format!(
            "https://discord.com/api/v10/applications/{}/commands",
            discord_client_id
        ))
        .bearer_auth(bot_token)
        .json(&json!({
            "name":"hello",
            "description":"say hello",
        }))
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
}
