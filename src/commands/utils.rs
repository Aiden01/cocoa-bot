command!(code(_ctx, msg, _args) {
    let _ = msg.channel_id.say("Here's how to wrap code:\n\n`\n```language\nyour code here\n```\n`\n\nFor large amount, please use a service like https://hastebin.com.");
});
