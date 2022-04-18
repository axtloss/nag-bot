use std::process::Command;
use std::fs::File;
use std::io::Read;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Error,
};

use serde::{Deserialize, Serialize};
//use serde_json::Result;


#[derive(Serialize, Deserialize)]
struct config {
    trigger: String,
    bot_token: String,
    nag_role_id: String,
    nag_people: Vec<nag_people>
}

#[derive(Serialize, Deserialize)]
struct nag_people {
    name: String,
    ntfy_id: String
}

struct Handler;

fn parseconfig() -> config {
    let mut config = String::new();
    match File::open(format!("{}/.config/nagbot.json", std::env::var("HOME").unwrap())) {
        Ok(mut file) => {
            file.read_to_string(&mut config).unwrap();
        },
        Err(error) => {
            println!("Error opening config file: {}", error);
        },
    }
    let parsed: config = serde_json::from_str(config.as_str()).unwrap();
    return parsed;
}

fn sendntfy(persontoannoy_id: String, annoyance: String, reasonforannoyance: String) {
    let mut command = Command::new("curl");
    command
        .arg("-H").arg("Title: Crystal nag!")
        .arg("-H").arg("Priority: default")
        .arg("-d").arg(format!("{} needs something! {}", annoyance, reasonforannoyance))
        .arg(format!("https://ntfy.sh/{}", persontoannoy_id)).status().unwrap();
}

pub async fn commandhandler(msg: Message, ctx: Context) -> Result<Message,Error> {
    let parsed: config = parseconfig();
    println!{"{:?}", parsed.nag_people[0].name};
    for i in parsed.nag_people {
        let msgauthor = &msg.author.name;
        if msg.content.to_lowercase().contains(format!("{} {}",parsed.trigger, i.name).as_str()) {
            let _ = sendntfy(i.ntfy_id, msgauthor.to_string(), msg.content.to_lowercase().replace(format!("{} {}", parsed.trigger, i.name).as_str(), ""));
            return  msg.channel_id.send_message(&ctx, |m| m.content(format!("{} is a nag", i.name))).await;
        }
    }
    return msg.channel_id.send_message(&ctx, |m| m.content("I don't know that person :frowning2:")).await;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let parsed: config = parseconfig();
        if msg.content.contains(&parsed.trigger) {
            if msg.author.has_role(ctx.http.clone(),  msg.guild_id.unwrap(), parsed.nag_role_id.parse::<u64>().unwrap()).await.unwrap() {
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
    let token = parseconfig().bot_token;
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}