# Oreo Bot

Oreo Bot is a Discord bot using ~~discord.js~~ Poise and Serentity, and continuing the legacy of the unmaintained Smarty. It's main goal is a utility bot with features that make everyday Discord usage enriched and easier.

Also Rust yeaaaaaaaa pog

## Notes

Make sure to do this when updating the db:

```rs
prisma
	.someschema()
	.someaction(some_args)
	// this part, remember to use either ? or unwrap();
	.exec().await?;
```

Remember to **include the prelude**

```rs
use crate::prelude::*
```