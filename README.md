# Oreo

A super extensible Discord bot written in Rust. This is a sucessor to the sometimes-maintained [smarty](https://github.com/pg_4919/smarty), which my friend made and abandoned . It currently handles QoL on our Discord server, sometimes integrating with Smarty to provide useful features (such as a command which allows you to jump from a news to the cloned chat message).

## Message Cloning

Message cloning is the main feature of Oreo (and Smarty). Using a webhook, the bot will copy a users name and avatar, and send a message with the same content, attachments, and embeds. It will even clone reply chains (using emojis and hyperlinks).

### Uses of message cloning

- Copying messages from `#news` to `#chat` for discussion
- A starboard that actually looks good
- Copying messages to other channels
- Impersonations

## Design

Oreo is designed to be as modular as possible, with a focus on extensibility. There are many different crates. Each binary crate (except for the main bot) has it's own tcp server, which is used to communicate with other parts of the bot. For example, the main bot can ask the logging server (which handles databasing) to copy a message.

## Build, Test, and Develop

1. Install the following:
    - [Rust](https://www.rust-lang.org/tools/install)
    - [Docker](https://docs.docker.com/get-docker/) for Docker Compose
    - [Docker Compose](https://docs.docker.com/compose/install/) to run the bot
    - [PostgreSQL](https://www.postgresql.org/download/) for the database
    - [Nushell](https://www.nushell.sh/book/installation.html) for build scripts

2. Make a bot from the [developer portal](https://discord.com/developers/applications) and get a token. Enable all intents.

3. Create a `.env` file with the following:

    ```env
    BOT_TOKEN="token from discord"
    DATABASE_URL="the connection string for your database"
    ```

4. Use `scripts/build.nu` to build bot bins:

    ```sh
    nu scripts/build.nu # --release for release build
    ```

5. Start the bot with:

    ```sh
    docker compose up
    ```

6. Stop the bot with:

    ```sh
    docker compose down
    ```

7. Cleanup with:

    ```sh
    nu scripts/clean.nu
    ```

## It seems every commit before last month has the same timestamp

I reset the tree of the git repo to point to my original commit.
Commits before the first used Smarty's codebase.
I did not like this because Smarty's codebase is unrelated to Oreo's.
