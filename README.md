# Oreo

A super extensible Discord bot written in Rust. This is a sucessor to the sometimes-maintained [smarty](https://github.com/pg_4919/smarty), which my friend made and abandoned . It currently handles qol on our Discord server, sometimes integrating with Smarty to provide useful features (such as a command which allows you to jump from a news to the cloned chat message).

## Message Cloning
Message cloning is the main feature of Oreo (and Smarty), essentially the backbone for most of the bots features. Using a webhook, the bot will copy a users name and avatar, and send a message with the same content, attachments, and embeds. It will even clone reply chains (using emojis).

### Uses of message cloning
- Copying messages from news to chat for discussion
- Copying messages from any channel to a starboard
- Leaking messages from one channel to another because you thought it was funny
- Impersonations

## Design
Oreo is designed to be as modular as possible, with a focus on extensibility. There are many different crates. Each binary crate (except for the main bot) has it's own tcp server, which is used to communicate with other parts of the bot. For example, the main bot can request the logger to log a message.

## Build, Test, and Develop
Oreo has no unit tests.

1. Install [Rust](https://www.rust-lang.org/tools/install), [Docker](https://docs.docker.com/get-docker/), [Docker Compose](https://docs.docker.com/compose/install/), [PostgreSQL](https://www.postgresql.org/download/), and [Nushell](https://www.nushell.sh/book/installation.html). 

2. Make an application from [the Discord Developer Portal](https://discord.com/developers/applications) and get a token for it. Invite it to your server.

3. Replace some constants.
	- `crates/prelude/nci.rs`
	- `.env`:
		```
		BOT_TOKEN="your token here"
		DATABASE_URL="database url here"
		```

4. Generate prisma and update db schema
	```
	cargo prisma-release db push
	```

	or

	```
	cargo prisma db push
	```
	for a debug build

5. Setup for `docker compose` by running `nu ./build-release.nu`

6. Run `docker compose up -d` to start the bot

7. Make sure to force-rebuild docker containers as the files will change!