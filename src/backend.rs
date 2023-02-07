/// i128 almost universially means its a GUID. 
/// We format GUID a bit weirdly when displaying them. 
/// But in code we treat them as just 128 consecutive bits. 
/// 
/// When displaying them to users, we
/// 1. Remove Hyphens
/// 2. Encode it in base32 (0-9, a-y (skipping o, l, i for readability))
/// 
/// Distinct Base 32 chart (dyslexic, skips o, l, i): 
/// lead: 
/// 00          01          10          11
/// 
/// 00000 (0)   01000 (8)   10000 (g)   11000 (r)
/// 00001 (1)   01001 (9)   10001 (h)   11001 (s)
/// 00010 (2)   01010 (a)   10010 (j)   11010 (t)
/// 00011 (3)   01011 (b)   10011 (k)   11011 (u)
/// 00100 (4)   01100 (c)   10100 (m)   11100 (v)
/// 00101 (5)   01101 (d)   10101 (n)   11101 (w)
/// 00110 (6)   01110 (e)   10110 (p)   11110 (x)
/// 00111 (7)   01111 (f)   10111 (q)   11111 (y)
/// 
/// There will be (3) extra bits to encode it in true base32, so we add two moc zeros just before the last 3 (displayed as 'x' in the following guid). This means they will always be much smaller, but it's the best solution. 
/// 
/// 3. Place colons every 4 characters. 
/// 
/// So this guid (commas are for reasability only): 
/// 01101,00001,01100,10110,01110,00000,01111,10000,10100,11001,00101,00110,11101,01111,00010,01010,11111,10000,10011,01101,01111,11100,11000,01011,01000,xx010
/// 
/// Gets displayed like: 
/// d1cp:e0fg:ms56:wf2a:ygkd:fveb:82

// This seems useless, but helps with code clarity as we can now use the type Guid.
struct Guid { guid: i128 }

/// In this, all permissions are singular. But if its a global permission (not overriden) then it applies to multiple, ofc.
/// Some of these may not be applicable everywhere, if they are not, they are set to `Null`, or the applicable value. 
/// In the following list, the comment displays: 
/// Defult Value (Hub defult and in DM's), Exceptions to the defult what and (where)
struct Permissions { 
    //                            Defult,  Exceptions (No can mean "no exceptions" or "not applicable")
    view_channel: Bool,          // True,  No
    view_audit_log: Bool,        // False, No
    view_message_history: Bool,  // True,  No

    manage_channel: Bool,        // False, No
    manage_roles: Bool,          // False, Yes (DMs, Group Chat: Null)
    manage_emoji: Bool,          // False, No
    manage_webhooks: Bool,       // False, Yes
    manage_bots: Bool,           // False, Yes (Profile: True)
    manage_guild: Bool,          // False, No
    manage_alias: Bool,          // False, No
    manage_threads: Bool,        // False, No
    manage_messages: Bool,       // False, No
    manage_threads: Bool,        // False, No
    manage_events: Bool,         // False, No

    edit_alias: Bool,            // True,  No

    can_kick: Bool,              // False, Yes (Group Chats: True)
    can_ban: Bool,               // False, No
    can_mute: Bool,              // False, No
    can_react: Bool,             // True,  No
    can_ping_all: Bool,          // False, Yes (Group Chats, DMs: True)
    can_ping_here: Bool,         // True,  No
    can_ping_online: Bool,       // True,  No
    can_ping_random: Bool,       // True,  No
    can_ping_this: Bool,         // False, No (For roles: can ping this role, for users: can ping me without being a friend) Only Applicable To Roles
    can_ping_roles: Bool,        // False, No
    can_move_members: Bool,      // False, No

    force_voice_mute: Bool,      // False, No
    force_voice_deafen: Bool,    // False, No

    create_invite: Bool,         // True, No
    send_messages: Bool,         // True, Yes (Info Channels: False)
    create_threads: Bool,        // True, Yes (Info Channels: False)
    
    embed_images: Bool,          // True,  No
    embed_links: Bool,           // True,  No

    use_external_emoji: Bool,    // True,  No
    use_commands: Bool,          // True,  No   (You probably don't want this: Instead, configure the commands indevidually)

    voice_connect: Bool,         // True,  No
    voice_speak: Bool,           // True,  No
    voice_video: Bool,           // True,  No
    voice_activity: Bool,        // True,  No
    voice_video: Bool,           // True,  No
    voice_priority: Bool,        // False, No

    blind_messages: Bool,        // False, No

    manage_all: Bool,            // False, Yes (Hub Owner: True)

    // Permission stack (Higher levels take priority): 
    //
    // Administrator (manage_all) will ALWAYS override EVERYTHING ELSE
    // Roles (Higher overrides lower, except for manage_all. See more info below)
    // Channel Permissions
    // Defult hub permissions
    // Defult permissions

    // Roles stack: 
    // Lets say we have someone with two roles: 
    //
    // Can't Ping: 
    //    - Defult
    //    - Can't ping anyone
    //
    // Trusted: 
    //    - Defult
    //    - Can Ping All
    // 
    // and someone has both, the can't ping anyone would override it. Any permissions left as defult (passthrough to default/hub), will "shine through".
}

struct Account { 
    username: String,
    discriminator: String,
    prefered_name: String,

    email: String,
    Phone: String,

    guid: Guid,
    prefrences: Vec, 
    profile: Vec
}

struct Message { 
    author: Guid,    // get a refrence to thier profile to load name, pfp, etc
    guid: Guid, 
    content: String,
    reactions: Vec,

    channel: Guid,
    reply_to: Guid,

    posted_time: String,  // DD-MMM-YYYY HH:MM:SS (07-FEB-2023 14:12:23)
    edited_time: String 
}

struct Channel { 
    guid: Guid,            // Chanels GUID
    parent: Guid,          // Category GUID (If uncategorised: Hub GUID)
    name: String,
    tropic: String,

    // text: Text Channels
    // info: Contain info and support full markdown.
    // voice: Voice Calls and video
    // stage: Video Presentations
    // gallery: Display images in a clean way, better than text
    // events/Calander: Datetime
    // kanban: Create and manage Kanaban boards.
    // forum: Create threads, good for issue reports, forums, etc.
    channel_type: String,  // Only specific values are valid. See the docs for more info

    permissions: Permissions // for each role that has overrides, and the channels overrides.
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
    permissions: Permissions
}