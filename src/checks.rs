use serenity::client::Context;
use serenity::framework::standard::{Args, CommandOptions};
use serenity::model::prelude::*;
use serenity::model::guild::Member;
use serenity::CACHE;


pub fn cannot_use_on_them(_ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();

    msg.author.id != member.user_id()

}
