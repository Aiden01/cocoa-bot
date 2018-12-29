use serenity::client::Context;
use serenity::framework::standard::{Args, CommandOptions};
use serenity::model::prelude::*;

const MOD_ROLE_NAME: &str = "Moderator";
const ADMIN_ROLE_NAME: &str = "Admin";

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
