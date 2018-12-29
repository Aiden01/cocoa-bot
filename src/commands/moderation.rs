use serenity::model::guild::Member;
use serenity::model::user::User;
use serenity::model::id::RoleId;
use serenity::CACHE;



command!(mute(_ctx, msg, args) {
    args.skip();
    let reason = match args.multiple::<String>() {
        Ok(s) => s.join(" "),
        Err(_) => String::from("Unknown")
    };

    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();
    let guild = msg.guild_id.unwrap().to_guild_cached().unwrap();
    
    
    if let Some(role) = guild.read().roles.get(&RoleId(527908267012390932)) {
        let _ = member.add_role(role.id);
        check_error(msg.channel_id.say(&format!(":hammer: **{}** has been muted for: **{}**", member.display_name(), reason)));
            let dm = user.create_dm_channel().unwrap();
            dm.say(&format!("You have been muted for: **{}**", reason));
    } else {
        check_error(msg.channel_id.say("Please create the **Muted** role."));
    }

});

command!(unmute(_ctx, msg, _args) {
    let cache = CACHE.read();
    let user: &User = &msg.mentions[0];
    let mut member: Member = cache.member(msg.guild_id.unwrap(), user.id).unwrap();

    let guild = msg.guild_id.unwrap().to_guild_cached().unwrap();
    
    
    if let Some(role) = guild.read().roles.get(&RoleId(527908267012390932)) {
        match member.remove_role(role.id) {
            Err(_) => check_error(msg.channel_id.say("An error occurred, cannot remove the role.")),
            Ok(_) => check_error(msg.channel_id.say(&format!("**{}** is now unmuted", member.display_name())))
        }
    } else {
        check_error(msg.channel_id.say("Please create the **Muted** role."));
    };


    
});
