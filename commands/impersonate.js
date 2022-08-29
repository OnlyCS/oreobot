const discord = require("discord.js");
const utils = require("../utils/utils.js");
const fs = require("fs");

module.exports = {
    name: "impersonate",
    data: new discord.SlashCommandBuilder()
        .setName("impersonate")
        .setDescription("Impersonate another user")
        .addSubcommand(subcommand => subcommand
            .setName("start")
            .setDescription("Start impersonating someone")
            .addUserOption(option => option
                .setName("person")
                .setDescription("The person to impersonate")
                .setRequired(true)
            )
        )
        .addSubcommand(subcommand => subcommand
            .setName("stop")
            .setDescription("Stop impersonating someone")
        )
        .toJSON(),
        
    async execute(interaction) {
        switch (interaction.options.getSubcommand()) {
            case "start":
                const impersonators = require(`${utils.path.temp}/impersonate.json`);
                impersonators[interaction.user.id] = interaction.options.getUser("person").id;
                fs.writeFileSync(`${utils.path.temp}/impersonate.json`, JSON.stringify(impersonators));
                await interaction.reply({content: impersonators[interaction.user.id], ephemeral: true});
                break;

            case "stop":
                await interaction.reply({content: "alright", ephemeral: true});
                break;
        }
    }
}