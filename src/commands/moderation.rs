use serenity::model::guild::Member;
use serenity::model::user::User;
use serenity::model::id::RoleId;
use serenity::model::id::ChannelId;
use serenity::model::id::MessageId;
use serenity::framework::standard::Args;
use serenity::builder::CreateEmbed;
use serenity::CACHE;
use serenity::utils::Colour;
use super::super::get_env_val;

fn get_reason(args: Args) -> String {
    match args.multiple::<String>() {
        Ok(s) => s.join(" "),
        Err(_) => String::from("Unknown")
    }
}

command!(mute(ctx, msg, args) {
    let MUTED_ROLE_ID = get_env_val(&ctx, "MUTED_ROLE_ID").unwrap().as_str().parse::<u64>().unwrap();;
    args.skip();
    let reason = get_reason(args);

    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();
    let guild = msg.guild_id.unwrap().to_guild_cached().unwrap();

    let g = guild.read();
    
    if let Some(role) = g.roles.get(&RoleId(MUTED_ROLE_ID)) {
        let _ = member.add_role(role.id);
        msg.channel_id.say(&format!(":hammer: **{}** has been muted for: **{}**", member.display_name(), reason));
            let dm = user.create_dm_channel().unwrap();
            dm.say(&format!("You have been muted for: **{}**", reason));
    } else {
        msg.channel_id.say("Please create the **Muted** role.");
    }

});

command!(unmute(ctx, msg, _args) {
    let MUTED_ROLE_ID = get_env_val(&ctx, "MUTED_ROLE_ID").unwrap().as_str().parse::<u64>().unwrap();
    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();

    let guild = msg.guild_id.unwrap().to_guild_cached().unwrap();
    
    let g = guild.read();
    if let Some(role) = g.roles.get(&RoleId(MUTED_ROLE_ID)) {
        match member.remove_role(role.id) {
            Err(_) => msg.channel_id.say("An error occurred, cannot remove the role."),
            Ok(_) => msg.channel_id.say(&format!("**{}** is now unmuted", member.display_name()))
        };
    } else {
        msg.channel_id.say("Please create the **Muted** role.");
    };
    
});

command!(clear(_ctx, msg, args) {
    let nb_to_delete = args.single::<u64>().unwrap();
    let channel = msg.channel_id.to_channel_cached().unwrap();
    let messages = channel.messages(|f| f.limit(nb_to_delete)).unwrap();

    let msg_ids: Vec<MessageId> = messages.iter().map(|m| m.id).collect();

    if let Err(_) = msg.channel_id.delete_messages(msg_ids) {
        msg.channel_id.say("Cannot clear messages").unwrap();
    } 

    msg.delete();

});

command!(ban(ctx, msg, args) {
    let cache = CACHE.read();
    let LOG_CHANNEL_ID = get_env_val(&ctx, "LOG_CHANNEL_ID").unwrap().as_str().parse::<u64>().unwrap();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();

    // arguments
    args.skip();
    let reason = get_reason(args);

    match member.ban(&reason) {
        Err(_) => {
            msg.channel_id.say(&format!("Cannot ban <@{}>", user.id));
        },
        Ok(_) => {
            let channels = msg.guild_id.unwrap().channels().unwrap();

            if let Some(channel) = channels.get(&ChannelId(LOG_CHANNEL_ID)) {

                let banned_usr_name = &format!("{}#{}", user.name, user.discriminator);
                let embed = |e: CreateEmbed| e 
                    .author(|a| a
                        .icon_url(&msg.author.default_avatar_url())
                        .name(banned_usr_name)
                    )
                    .title("has been banned")
                    .color(Colour::from_rgb(255, 161, 82))
                    .field("By", &format!("{}#{}", msg.author.name, msg.author.discriminator), true)                  
                    .field("Reason", reason.clone(), true);
                channel.send_message(|m| m
                    .embed(embed)
                );      
                msg.channel_id.say(&format!("**{}** has been banned for **{}**", banned_usr_name, reason))     ;   
            } else {
                msg.channel_id.say("Logs channel not found");
            };
        }
    };
});
