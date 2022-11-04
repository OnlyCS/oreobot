const utils = require("../utils/utils.js");

module.exports = async (reaction, user) => {
    const message = reaction.message;
    const starred = message.guild.channels.fetch(client.config.starred);

    try { await reaction.fetch() } catch (err) { return err };
    if (reaction.emoji.name !== "📌" || reaction.me || user.bot) return;

    await message.react("📌");
    await utils.clone(message.member, starred, message, true);
    
    return;
}