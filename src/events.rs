use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::Colour;

const WELCOME_CHANNEL: u64 = 527718209248296973;
const RULES_CHANNEL: u64 = 527885775270969364;

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
}
