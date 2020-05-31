use serenity::{
    model::prelude::*,
    prelude::*,
};

mod cmds;
mod config;

struct Handler; impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let substr = &msg.content[..];
        let mut args: Vec<&str> = substr.split(" ").collect();
        let command = args[0];
        args.remove(0); // remove prefix + command keyword from args list.

        if command == config::Config::prefix() + "ping" {
            cmds::ping::exec(ctx, &msg, args);
        }
        else if command == config::Config::prefix() + "pong" {
            cmds::pong::exec(ctx, &msg, args);
        }
    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let mut client = Client::new(config::Config::token(), Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}