use serenity::{
    async_trait,
    client::EventHandler,
    prelude::Context,
    model::{
        id::UserId,
        gateway::Ready,
        channel::Message
    }
};
use regex::Regex;

pub struct Handler;

macro_rules! check_bot_msg {
    ($msg:ident) => {
        if $msg.author.id == UserId::from(848269517490749440) {
            return;
        }
    };
}

macro_rules! return_exception {
    ($result:ident) => {
        if $result.is_ok() { Ok(()) } else { Err($result.unwrap_err()) }
    };
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        check_bot_msg!(msg);
        println!("[MSG] {}", msg.content);
        let shard = ctx.shard_id;
        let result = match msg.content.to_lowercase().as_str() {
            "!help" => help(&ctx, &msg).await,
            "!ping" => ping(&ctx, &msg).await,
            "!dm" => dm(&ctx, &msg).await,
            "hi" | "hello" => hello(&ctx, &msg).await,
            "!emoji" => emoji(&ctx, &msg).await,
            _ => {
                let expr = Regex::new("[-,.:'\" ]vim[ .,:'\"\n]").unwrap();
                if expr.is_match(format!(" {} ", msg.content).as_str()) {
                    let res = vim(&ctx, &msg).await;
                    return_exception!(res)
                } else {
                    Ok(())
                }
            }
        };
        if let Err(err) = result {
            eprintln!("[{}] Error: {}", shard, err);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

async fn help(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let result = msg.reply(&ctx.http, "There's no hope for help for us").await;
    return_exception!(result)
}


async fn ping(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let result = msg.channel_id
        .say(&ctx.http, format!("Pong from shard {}", ctx.shard_id)).await;
    return_exception!(result)
}

async fn dm(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let res = msg.author.dm(&ctx, |m| m.content("No DMs for you")).await;
    return_exception!(res)
}

async fn hello(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let response = serenity::utils::MessageBuilder::new()
        .push("Hello, ")
        .push(&msg.author)
        .push(" <:man:865360237041418279>")
        .build();
    let res = msg.channel_id.say(&ctx.http, response).await;
    return_exception!(res)
}

async fn emoji(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let resp = serenity::utils::MessageBuilder::new()
        .push("<:3dkek:896069079826305035>")
        .build();
    let res = msg.channel_id.say(&ctx.http, resp).await;
    return_exception!(res)
}

async fn vim(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    let res = msg.channel_id.say(&ctx.http, "vim )").await;
    return_exception!(res)
}
