pub use crate::prelude::*;

/*** INTERACTIONS ***/
pub struct CommandInteractionEvent;
impl EmitterEvent for CommandInteractionEvent {
    type Argument = serenity::ApplicationCommandInteraction;
}

pub struct ComponentInteractionEvent;
impl EmitterEvent for ComponentInteractionEvent {
    type Argument = serenity::MessageComponentInteraction;
}

pub struct ModalInteractionEvent;
impl EmitterEvent for ModalInteractionEvent {
    type Argument = serenity::ModalSubmitInteraction;
}

/*** CHANNEL CATEGORY ***/
pub struct CategoryCreateEvent;
impl EmitterEvent for CategoryCreateEvent {
    type Argument = serenity::ChannelCategory;
}

pub struct CategoryUpdateEvent;
impl EmitterEvent for CategoryUpdateEvent {
    type Argument = serenity::ChannelCategory;
}

pub struct CategoryDeleteEvent;
impl EmitterEvent for CategoryDeleteEvent {
    type Argument = serenity::ChannelCategory;
}

/*** CHANNEL ***/
pub struct ChannelCreateEvent;
impl EmitterEvent for ChannelCreateEvent {
    type Argument = serenity::GuildChannel;
}

pub struct ChannelUpdateEvent;
impl EmitterEvent for ChannelUpdateEvent {
    type Argument = serenity::GuildChannel;
}

pub struct ChannelDeleteEvent;
impl EmitterEvent for ChannelDeleteEvent {
    type Argument = serenity::GuildChannel;
}

/*** MESSAGE ***/
pub struct MessageCreateEvent;
impl EmitterEvent for MessageCreateEvent {
    type Argument = serenity::Message;
}

pub struct MessageUpdateEvent;
impl EmitterEvent for MessageUpdateEvent {
    type Argument = serenity::MessageUpdateEvent;
}

pub struct MessageDeleteEvent;
impl EmitterEvent for MessageDeleteEvent {
    type Argument = payloads::MessageDeletePayload;
}

/*** ROLE ***/
pub struct RoleCreateEvent;
impl EmitterEvent for RoleCreateEvent {
    type Argument = serenity::Role;
}

pub struct RoleUpdateEvent;
impl EmitterEvent for RoleUpdateEvent {
    type Argument = serenity::Role;
}

pub struct RoleDeleteEvent;
impl EmitterEvent for RoleDeleteEvent {
    type Argument = payloads::RoleDeletePayload;
}

/*** MEMBER ***/
pub struct MemberJoinEvent;
impl EmitterEvent for MemberJoinEvent {
    type Argument = serenity::Member;
}

pub struct MemberUpdateEvent;
impl EmitterEvent for MemberUpdateEvent {
    type Argument = serenity::Member;
}

pub struct MemberLeaveEvent;
impl EmitterEvent for MemberLeaveEvent {
    type Argument = payloads::MemberLeavePayload;
}

/*** READY ***/
pub struct BotReadyEvent;
impl EmitterEvent for BotReadyEvent {
    type Argument = serenity::Ready;
}
