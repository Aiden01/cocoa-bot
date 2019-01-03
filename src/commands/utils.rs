
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, GuildId};
use serenity::utils::Colour;
use serenity::builder::CreateEmbed;
use super::super::get_env_val;


command!(code(_ctx, msg, _args) {
    let _ = msg.channel_id.say("Here's how to wrap code:\n\n`\n```language\nyour code here\n```\n`\n\nSo it becomes\n```swift\nprint('Hello, World')\n```\nFor large amount, please use a service like https://hastebin.com.");
});




command!(addresource(ctx, msg, args) {
    let RESOURCES_CHANNEL = get_env_val(&ctx, "RESOURCES_CHANNEL").unwrap().as_str().parse::<u64>().unwrap();
    let channels = msg.guild_id.unwrap().channels().unwrap();

    if let Some(channel) = channels.get(&ChannelId(RESOURCES_CHANNEL)) {
            // Get the arguments
            let title: String = args.single::<String>().unwrap();
            let link: String = args.single::<String>().unwrap();
            let desc: String = args.multiple::<String>().unwrap().join(" ");

            let create_embed = |e: CreateEmbed| e
                .author(|a| a
                    .icon_url(&msg.author.default_avatar_url())
                    .name(&format!("{}#{}", msg.author.name, msg.author.discriminator))
                )
                .color(Colour::from_rgb(255, 161, 82))
                .field(title, desc, false)
                .field("Link", link, false);

            // send the resource to the channel
            let resource = channel.send_message(|m| m.content(&format!("New resource added by <@{}>", msg.author.id)).embed(create_embed)).unwrap();
            resource.react("🚫").unwrap();
            msg.channel_id.say("You resource has been added successfully :white_check_mark:. If you want to remove it, click on :no_entry_sign:.").unwrap();
            msg.delete().unwrap();
    } else {
        msg.channel_id.say("Resources channel not found").unwrap();
    }
});