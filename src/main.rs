use std::process::Command;
use serenity::Error;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

fn sendntfy(persontoannoy: String, annoyance: String, reasonforannoyance: String) {
    let mut command = Command::new("curl");
    command
        .arg("-H").arg("Title: Crystal nag!")
        .arg("-H").arg("Priority: default")
        .arg("-d").arg(format!("{} needs something! {}", annoyance, reasonforannoyance))
        .arg(format!("https://ntfy.sh/nagbot{}", persontoannoy)).status().unwrap();
}

pub async fn commandhandler(msg: Message, ctx: Context) -> Result<Message,Error> {
    if msg.content.to_lowercase().contains("!nag amy") {
        let _ = sendntfy("Amy".to_string(), msg.author.name, msg.content.to_lowercase().replace("!nag amy ", ""));
        let msg = msg.channel_id.send_message(&ctx, |m| m.content("Amy is a nag")).await;
        return msg;
    } else if msg.content.to_lowercase().contains("!nag matt") {
        let _ = sendntfy("Matt".to_string(), msg.author.name, msg.content.to_lowercase().replace("!nag matt", ""));
        let msg = msg.channel_id.send_message(&ctx, |m| m.content("Matt is a nag")).await;
        return msg;
    } else if msg.content.to_lowercase().contains("!nag michal") {
        let _ = sendntfy("Michal".to_string(), msg.author.name, msg.content.to_lowercase().replace("!nag michal", ""));
        let msg = msg.channel_id.send_message(&ctx, |m| m.content("Matt is a nag")).await;
        return msg;
    } else {
        return  msg.channel_id.send_message(&ctx, |m| m.content("I don't know that person :frowning2:")).await;
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("!nag") {
            if msg.author.has_role(ctx.http.clone(),  msg.guild_id.unwrap(), 912457510512902184).await.unwrap() {
                let msg = commandhandler(msg, ctx).await;
                if let Err(why) = msg {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                let msg = msg.channel_id.send_message(&ctx, |m| m.content("You don't have permission to nag people!")).await;
                if let Err(why) = msg {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}