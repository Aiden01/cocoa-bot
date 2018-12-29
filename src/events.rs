use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::Colour;
use serenity::model::channel::Reaction;
use serenity::CACHE;
// resources channel's id
use super::commands::utils::RESOURCES_CHANNEL;

const WELCOME_CHANNEL: u64 = 527310098679201795;
const RULES_CHANNEL: u64 = 527310131755745280;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, bot: Ready) {
        let _ = ctx.edit_profile(|profile| profile.username("CocoaBot"));
        println!("{} is now connected", bot.user.name);
    }

    // Fired when a user joins the server
    fn guild_member_addition(&self, _ctx: Context, guild_id: GuildId, new_member: Member) {
        let user = new_member.user_id().to_user().unwrap();

        let channels = guild_id.channels().unwrap();
        match channels.get(&ChannelId(WELCOME_CHANNEL)) {
            None => println!("Channel not found"),
            Some(channel) => {
                let msg = channel
                    .say(&format!(
                        "Welcome <@!{}>! Please make sure to read <#{}>",
                        new_member.user_id(),
                        RULES_CHANNEL,
                    ))
                    .expect("An error occured");

                msg.react("👋").unwrap();
            }
        }
    }

    fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        if(reaction.channel_id == RESOURCES_CHANNEL) {
            remove_resource(&reaction);
        }

    }

}

// Remove a resource with the author has clicked on the reaction
fn remove_resource(reaction: &Reaction) {
    // read the cache
    let cache = CACHE.read();

    let msg = reaction.message().unwrap();
    let author = reaction.user().unwrap();
    let user_id = msg.mentions[0].id;

    if(user_id == author.id) {
        msg.delete().unwrap();
        let dm = author.create_dm_channel().expect("Cannot create dm channel");
        dm.say("Your resource has been removed.");
    }
}
