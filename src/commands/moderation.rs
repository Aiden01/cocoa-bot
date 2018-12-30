use serenity::model::guild::Member;
use serenity::model::user::User;
use serenity::model::id::RoleId;
use serenity::CACHE;
use super::super::get_env_val;



command!(mute(ctx, msg, args) {
    let MUTED_ROLE_ID = get_env_val(&ctx, "MUTED_ROLE_ID").unwrap().as_str().parse::<u64>().unwrap();;
    args.skip();
    let reason = match args.multiple::<String>() {
        Ok(s) => s.join(" "),
        Err(_) => String::from("Unknown")
    };

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
    let MUTED_ROLE_ID = get_env_val(&ctx, "MUTED_ROLE_ID").unwrap().as_str().parse::<u64>().unwrap();;
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
