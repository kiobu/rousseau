use serenity::{
    model::prelude::*,
    prelude::*,
};

pub fn exec(ctx: Context, msg: &Message, args: Vec<&str>) {
    match msg.channel_id.say(&ctx.http, format!("Ping! {:?}", args)) {
        Ok(_) => (),
        Err(why) => println!("There was an issue sending the reply. {:?}", why)
    }
}