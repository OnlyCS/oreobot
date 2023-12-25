use crate::prelude::*;
use oreo_proc_macros::logger_wire;

pub async fn register() {
    mpmc::on(|_, event, _| async move {
        let mut logger = Client::<LoggingServer>::new().await?;

        logger_wire! {
            event,
            logger,

            // interaction
            InteractionCreate { interaction } => InteractionCreate(interaction.into()),

            // category
            ChannelCreate { channel } if channel.kind == ChannelType::Category => CategoryCreate(channel),
            ChannelUpdate { new, .. } if new.kind == ChannelType::Category => CategoryUpdate(new),
            ChannelDelete { channel, .. } if channel.kind == ChannelType::Category => CategoryDelete(channel.id),

            // channel
            ChannelCreate { channel } => ChannelCreate(channel),
            ChannelUpdate { new, .. } => ChannelUpdate(new),
            ChannelDelete { channel, .. } => ChannelDelete(channel.id),

            // message
            Message { new_message, .. } => MessageCreate(new_message),
            MessageUpdate { event, .. } => MessageUpdate(event),
            MessageDelete { deleted_message_id: id, .. } => MessageDelete(id),

            // role
            GuildRoleCreate { new } => RoleCreate(new),
            GuildRoleUpdate { new, .. } => RoleUpdate(new),
            GuildRoleDelete { removed_role_id: id, .. } => RoleDelete(id),

            // member
            GuildMemberAddition { new_member } => MemberCreate(new_member),
            GuildMemberUpdate { event, .. } => MemberUpdate(event),
            GuildMemberRemoval { user, .. } => MemberDelete(user.id),

            // ready
            Ready { .. } => ReadyEvent,

            _ => bail!(EventError::UnwantedEvent)
        };

        Ok(())
    }).await
}
