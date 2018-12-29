#[macro_use]
extern crate serenity;

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



fn main() {
    let token = env::var("BOT_TOKEN").expect("Please provide the bot's token in the environnement");

    let mut client: Client = Client::new(&token, Handler).expect("Error while creating the client");

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
        
                .lacking_role(HelpBehaviour::Nothing)
                
                .wrong_channel(HelpBehaviour::Strike)
                 })
        // utils
        .command("addresource", |c| c
            .cmd(commands::utils::addresource)
            .desc("Adds a resource to the ressources channel")
            .min_args(3)
        )
        .command("code", |c| c.cmd(commands::utils::code).desc("Shows you own to wrap your code"))
        // moderation
        .command("mute", |c| c
            .cmd(commands::moderation::mute)
            .check(checks::mod_check)
            .desc("Mutes the user")
            .min_args(1)
        )
        .command("unmute", |c| c
            .cmd(commands::moderation::unmute)
            .check(checks::mod_check)
            .desc("Unmutes the user")
            .num_args(1)
        );

    client.with_framework(fw);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
