use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
};
use serenity::framework::StandardFramework;
use serenity::client::{Context, EventHandler};

struct Handler;

const PREFIX: &str = "?";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if &msg.content[0..1] != PREFIX {
            return
        }

        let to_send = match msg.content[1..].trim() {
            "ping" => "Pong!",
            "help" => ":eyes:\t!ping\n<:PeepoSalute:692461814562160660>",
            _ => "that option does not exist! <:PeepoKnife:692461814587457657>",
        };
        if let Err(e) = msg.channel_id.say(&ctx.http, to_send).await {
            println!("Error sending message: {:?}", e)
        }
    }

    async fn ready(&self, _ctx: Context, bot_data: Ready) {
        println!("{} is connected", bot_data.user.name)
    }
}

pub async fn setup() {
    let token = env::var("DISCORD_TOKEN").expect("Couldn't get token from environment variables");

    // let framework = StandardFramework::new(); TODO
    let mut client = serenity::Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e)
    }
}
