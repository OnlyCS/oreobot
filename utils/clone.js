const discord = require("discord.js");
const truncate = require("./truncate.js");

module.exports = async (destination, message, link = false) => {
    const webhooks = await destination.fetchWebhooks();
    const webhook = webhooks.first() || await destination.createWebhook({ name: "Smarty" });

    const {
        attachments,
        author,
        channel,
        client,
        content,
        embeds,
        member,
        reference,
        type,
        url
    } = message;

    const { curved, straight } = client.config.emojis;

    let reftext = "";
    if (reference && type === 19) {
        const reply = await channel.messages.fetch(reference.messageId);
        const { member, author, content } = reply;
        const truncated = truncate(content, 50);

        reftext = `<:curved:${curved}> `
            + discord.bold(member?.displayName || author?.username || "Anonymous")
            + ` ${truncated}\n`
            + `<:straight:${straight}>\n `;
    }

    await webhook.send({
        allowedMentions: { parse: [] },
        avatarURL: member?.displayAvatarURL() || null,
        content: reftext + content + (link ? `\n[\[jump\]](${url})` : "") || "",
        embeds: [...embeds],
        files: [...(attachments?.values() || [null])],
        username: member?.displayName || author?.username || "Anonymous"
    }).catch(err => console.log);

    return;
}