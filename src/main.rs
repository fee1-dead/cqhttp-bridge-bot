#![feature(once_cell)]
use std::env;
use std::sync::OnceLock;

use api::Request;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use serenity::async_trait;
use serenity::model::prelude::{ChannelId, Message, Ready};
use serenity::prelude::*;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::api::{EventInner, MessageInner};

mod api;

struct Handler(Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>);

static CONTEXT: OnceLock<Context> = OnceLock::new();

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, _ctx: Context, msg: Message) {
        if let Some(guild_id) = msg.guild_id {
            if guild_id == 1051160112036851733 {
                let req = Request {
                    action: "send_msg".to_string(),
                    params: vec![
                        ("group_id".to_string(), "345968883".to_string()),
                        ("message".to_string(), msg.content.clone()),
                    ]
                    .into_iter()
                    .collect(),
                };
                self.0
                    .lock()
                    .await
                    .send(WsMessage::Text(serde_json::to_string(&req).unwrap()))
                    .await
                    .unwrap();
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let _ = CONTEXT.set(ctx);
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    main_inner().await
}

async fn main_inner() -> color_eyre::Result<()> {
    let url = "ws://localhost:8080";
    let (ws_stream, _) = connect_async(url).await?;
    let (send, receive) = ws_stream.split();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler(Mutex::new(send)))
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        client.start().await.expect("Err with client");
    });

    tokio::spawn(async {
        receive
            .for_each(|msg| async {
                if let Ok(WsMessage::Text(t)) = msg {
                    dbg!(&t);
                    let Ok(event) = serde_json::from_str::<api::Event>(&t) else { return };
                    match event.inner {
                        EventInner::Message {
                            inner:
                                MessageInner::Group {
                                    group_id: 345968883,
                                },
                            message,
                            ..
                        } => {
                            ChannelId(1051160112661794878)
                                .send_message(CONTEXT.get().unwrap(), |x| x.content(message))
                                .await
                                .unwrap();
                        }
                        _ => {}
                    }
                }
            })
            .await
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
