use serenity::client::Context;
use serenity::framework::standard::{Args, CommandOptions};
use serenity::model::prelude::*;
use serenity::model::guild::Member;
use serenity::CACHE;

const MOD_ROLE_NAME: &str = "Moderator";
const ADMIN_ROLE_NAME: &str = "Admin";

// check if the user has the right permissions
pub fn mod_check(_ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    let role = msg.member().unwrap().roles.into_iter().find(|r| {
        let role = r.to_role_cached().unwrap();
        role.name == MOD_ROLE_NAME || role.name == ADMIN_ROLE_NAME
    });

    match role {
        Some(_) => true,
        None => false,
    }
}


pub fn cannot_use_on_them(_ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> bool {
    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();

    msg.author.id != member.user_id()

}
