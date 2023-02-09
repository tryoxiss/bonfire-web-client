// rocket import amoung others

// This seems useless, but helps with code clarity as we can now use the type Guid.
struct Guid { guid: i128 }

struct Permissions {
    view_channel: Bool,
    view_audit_log: Bool,
    view_message_history: Bool,

    manage_channel: Bool,
    manage_roles: Bool,
    manage_emoji: Bool,
    manage_webhooks: Bool,
    manage_bots: Bool,
    manage_guild: Bool,
    manage_alias: Bool,
    manage_threads: Bool,
    manage_messages: Bool,
    manage_threads: Bool,
    manage_events: Bool,

    edit_alias: Bool,

    can_kick: Bool,
    can_ban: Bool,
    can_mute: Bool,
    can_react: Bool,
    can_ping_all: Bool,
    can_ping_here: Bool,
    can_ping_online: Bool,
    can_ping_random: Bool,
    can_ping_this: Bool,
    can_ping_roles: Bool,
    can_move_members: Bool,

    force_voice_mute: Bool,
    force_voice_deafen: Bool,

    create_invite: Bool,
    send_messages: Bool,
    create_threads: Bool,
    
    embed_images: Bool,
    embed_links: Bool,

    use_external_emoji: Bool,
    use_commands: Bool,

    voice_connect: Bool,
    voice_speak: Bool,
    voice_video: Bool,
    voice_activity: Bool,
    voice_video: Bool,
    voice_priority: Bool,

    blind_messages: Bool,

    manage_all: Bool,
}

struct Account {
    username: String,
    discriminator: String,
    prefered_name: String,

    email: String,
    phone: String,

    guid: Guid,
    prefrences: Vec, 
    profile: Vec
}

struct Message {
    author: Guid,
    guid: Guid, 
    content: String,
    reactions: Vec,

    channel: Guid,
    reply_to: Guid,

    posted_time: String,  // DD-MMM-YYYY HH:MM:SS (07-FEB-2023 14:12:23)
    edited_time: String 
}

struct Channel {
    guid: Guid,     // Chanels GUID
    parent: Guid,   // Category GUID (If uncategorised: Hub GUID)
    name: String,
    tropic: String,

    channel_type: String,  // Only specific values are valid. See the docs for more info

    permissions: Permissions, // for each role that has overrides, and the channels overrides.
}

struct Hub {
    guid: Guid,
    name: String,
    icon: String, // URI

    roles: Vec, 
    permissions: Permissions
}

struct Role {
    guid: Guid,
    name: String,
    badge: String, // URI
    permissions: Permissions,

    channel_overrides: Vec // Diffrent permission objects for each channel.
}
