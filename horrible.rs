struct Handler;
#[serenity::async_trait]
impl serenity::prelude::EventHandler for Handler {
    async fn message(&self, ctx: serenity::prelude::Context, msg: serenity::model::channel::Message) {
        if msg.content.to_lowercase().contains("!nag usr1") && msg.author.has_role(ctx.http.clone(), msg.guild_id.unwrap(), std::env::var("NAG_ROLE_ID").unwrap().parse::<u64>().unwrap()).await.unwrap() {
            std::process::Command::new("curl").arg("-H").arg("Title: nag!").arg("-H").arg("Priority: default").arg("-d").arg(format!("{} needs something! {}", msg.author.name, msg.content.to_lowercase().replace("!nag usr1", ""))).arg("https://ntfy.sh/nagbotusr1").status();
            msg.channel_id.send_message(&ctx, |m| m.content("usr1 is a nag")).await;
        } else if msg.content.to_lowercase().contains("!nag usr2") && msg.author.has_role(ctx.http.clone(),  msg.guild_id.unwrap(), std::env::var("NAG_ROLE_ID").unwrap().parse::<u64>().unwrap()).await.unwrap() {
            std::process::Command::new("curl").arg("-H").arg("Title: nag!").arg("-H").arg("Priority: default").arg("-d").arg(format!("{} needs something! {}", msg.author.name, msg.content.to_lowercase().replace("!nag usr2", ""))).arg("https://ntfy.sh/nagbotusr2").status();
            msg.channel_id.send_message(&ctx, |m| m.content("usr2 is a nag")).await;
        } else if msg.content.to_lowercase().contains("!nag usr3") && msg.author.has_role(ctx.http.clone(),  msg.guild_id.unwrap(), std::env::var("NAG_ROLE_ID").unwrap().parse::<u64>().unwrap()).await.unwrap() {
            std::process::Command::new("curl").arg("-H").arg("Title: nag!").arg("-H").arg("Priority: default").arg("-d").arg(format!("{} needs something! {}", msg.author.name, msg.content.to_lowercase().replace("!nag usr3", ""))).arg("https://ntfy.sh/nagbotusr3").status();
            msg.channel_id.send_message(&ctx, |m| m.content("usr3 is a nag")).await;
        }
    }
}
#[tokio::main]
async fn main() {
    serenity::Client::builder(&std::env::var("DISCORD_TOKEN").unwrap()).event_handler(Handler).await.expect("Err creating client").start().await;
}