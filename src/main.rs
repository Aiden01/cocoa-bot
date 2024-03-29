#[macro_use]
extern crate serenity;
extern crate typemap;
extern crate dotenv;

mod commands;
mod events;
mod checks;

use self::events::Handler;
use serenity::{
    framework::standard::{
        help_commands, DispatchError, HelpBehaviour, StandardFramework,
    },
    prelude::*,
};




use std::env;
use std::sync::Arc;
use typemap::Key;

pub struct Configuration;

impl Key for Configuration {
    type Value = Vec<Result<(String, String), dotenv::Error>>;
}

// retrieves a variable from the .env file
pub fn get_env_val(ctx: &Context, key: &str) -> Option<String> {
    let data = ctx.data.lock();
    let conf = data.get::<Configuration>().expect("Configuration not found");

    let var = conf.into_iter().find(|item| {
        if let Ok((k, v)) = item {
            k == key
        } else {
            false
        }
        
    }).unwrap();

   let s = var.clone();
   match s {
       Ok((_, v)) => Some(v.to_string()),
       Err(_) => None
   }

}



fn main() {
    let token = env::var("BOT_TOKEN").expect("Please provide the bot's token in the environnement");
    let mut client: Client = Client::new(&token, Handler).expect("Error while creating the client");

    // load .env file for the Configurations
    let env_name = if env::var("BOT_ENV").unwrap() == "prod" { ".env" } else { "dev.env" };
    let vars: Vec<Result<(String, String), dotenv::Error>> = dotenv::from_filename_iter(env_name).unwrap().collect();

    // set the daya
    {
        let mut data = client.data.lock();
        data.insert::<Configuration>(vars);
    }

    let fw = StandardFramework::new()
        .configure(|c| c.allow_whitespace(true).on_mention(true).prefix("++"))
        .before(|_ctx, msg, command_name| {
            println!("Got command {} by user {}", command_name, msg.author.name);
            true
        })
        .unrecognised_command(|_, msg, unknown_command_name| {
            let _ = msg.channel_id.say(&format!("Could not find command named **{}**", unknown_command_name));
        })
        .on_dispatch_error(|_ctx, msg, error| {
            if let DispatchError::RateLimited(seconds) = error {
                let _ = msg
                    .channel_id
                    .say(&format!("Try this again in {} seconds.", seconds));
            }
        })
        .customised_help(help_commands::with_embeds, |c| {
                // This replaces the information that a user can pass
                // a command-name as argument to gain specific information about it.
                c.individual_command_tip("Hello! こんにちは！Hola! Bonjour! 您好!\n\
                If you want more information about a specific command, just pass the command as argument.")
                .command_not_found_text("Could not find: `{}`.")

                .max_levenshtein_distance(3)
                .lacking_permissions(HelpBehaviour::Hide)
        
                .lacking_role(HelpBehaviour::Strike)
                .wrong_channel(HelpBehaviour::Strike)
        })
        // utils
        .group("Utility", |g| g
            .command("addresource", |c| c
                .cmd(commands::utils::addresource)
                .desc("Adds a resource to the resources channel")
                .usage("++addresource [Title (use - instead of spaces)] [Link] [Description]")
                .min_args(3)
            )
            .command("code", |c| c.cmd(commands::utils::code).desc("Shows you own to wrap your code"))
        )
        // moderation
        .group("Moderation", |g| g
            .command("mute", |c| c
                .cmd(commands::moderation::mute)
                .check(checks::cannot_use_on_them)
                .allowed_roles(vec!["Admin, Moderator"])
                .desc("Mutes the user")
                .min_args(1)
            )
            .command("unmute", |c| c
                .cmd(commands::moderation::unmute)
                .allowed_roles(vec!["Admin, Moderator"])
                .check(checks::cannot_use_on_them)
                .desc("Unmutes the user")
                .num_args(1)
            )
            .command("ban", |c| c
                .cmd(commands::moderation::ban)
                .allowed_roles(vec!["Admin, Moderator"])
                .check(checks::cannot_use_on_them)
                .desc("Bans the user")
                .min_args(1)
            )
            .command("clear", |c| c
                .cmd(commands::moderation::clear)
                .allowed_roles(vec!["Admin, Moderator"])
                .desc("Deletes x messages")
                .num_args(1)
            )
        );

    client.with_framework(fw);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
