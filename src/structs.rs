// // rocket import amoung others

// // This seems useless, but helps with code clarity as we can now use the type Guid.
struct Guid { guid: u128 }

pub struct Permissions {

    perms_for: Guid, // role or user
    // Useful so that channels can track a list of permissions objects, knowing what each is for. 

    require_e2e: bool, 

    // viewing permissions
    view_channel: bool,
    view_audit_log: bool,
    view_message_history: bool,

    // group management permissions
    manage_channel: bool,
    manage_roles: bool,
    manage_emoji: bool,
    manage_webhooks: bool,
    manage_bots: bool,
    manage_guild: bool,
    manage_alias: bool,
    manage_messages: bool,
    manage_threads: bool,
    manage_events: bool,

    // member managementpermissions
    edit_alias: bool,
    kick: bool,
    ban: bool,
    mute_others: bool,
    move_members: bool,
    force_voice_mute: bool,
    force_voice_deafen: bool,

    // ??? 
    add_react: bool,

    // role permissions
    can_ping_all: bool,
    can_ping_here: bool,
    can_ping_online: bool,
    can_ping_random: bool,
    can_ping_this: bool,
    can_ping_roles: bool,

    create_invite: bool,
    send_messages: bool,
    create_threads: bool,

    use_external_emoji: bool,
    use_commands: bool,

    voice_connect: bool,
    voice_speak: bool,
    voice_video: bool,
    voice_activity: bool,
    voice_priority: bool,

    blind_messages: bool,

    grant_all: bool, 
}

pub struct Status { 
    guid: Guid, // users GUID, NOT statuses. They dont get GUIDs

    is_rich: bool, // if false: the icon is instead an emoji, and only the status is filled, the rest are null

    title: String,
    status: String,
    details: String,

    allow_join_game: bool, 
    join_secret: String,
    icon: String
}

pub struct Account { 
    guid: Guid, 
    email: String,
    phone: String,

    // profiles: Vec,
}

pub struct Profile {
    // (Almost) all accesible and editable via the settings menu

    username: String,
    discriminator: u16, // min: 0000 (system) max: 9999
    prefered_name: String,

    alt_names: String,
    ponouns: String, 

    guid: Guid,

    // Prefrences. Warning: LONG. 
    // Dosen't make sense to seperate it into a seperate struct because they are only used here. 

    // privacy and safety

    require_e2e: bool,
    try_serverless: bool, // not truly serverless, but avoids most server interaction by going directly peer-to-peer.
    expand_embeded_urls: bool, // show the raw text for [embeeded links](url)

    // dm = direct message
    require_friendship_to_dm: bool, // if  on: overrides all below settings in this block to FALSE
    allow_group_members_to_dm: bool,
    allow_seccondhand_friends_to_dm: bool, // friends of friends
    allow_instance_members_to_dm: bool,
    allow_anyone_to_dm: bool,
    require_message_requests: bool,

    // fr = friend request
    allow_group_members_to_fr: bool,
    allow_seccondhand_friends_to_fr: bool, // friends of friends
    allow_instance_members_to_fr: bool,
    allow_anyone_to_fr: bool,

    //# appearance
    theme: String, // defults: Sync (Detect System Prefrence, defult) Dark or Mocha (ctp-mocha), light or Latte (ctp-latte), frappe (ctp-frappe), machitto (ctp-machitto). hi-dark (), hi-light,
    // add your own theme by adding a CSS file to the /themes/ directory on your client. 
    // Customisation may not be avalible on browsers. 
    icons: String,

    // these can also be edited in-depth with client customisation and advanced themes. 
    display_messages: String, // cozy (avatars and breathing room), compact (name next to message)
    font_size: u8, // 1rem = 16px (fdefult: 1rem), number is pixel value
    message_spacing: u8, 
    zoom_level: f32, // percent of the defult scale. 

    //# accesability
    always_underline_links: bool, 
    saturation: f32, // desaturate all colors in an ae filter. 
    seperate_role_color: bool, // display the role clor as a not next to the name instead
    display_role_color: bool, // display role color at all. 
    display_profile_themes: bool, // some profiles may be overstimulating or hard to read, disable to this to force them to all match your client theme

    // the following block can be toggled all at once with reduced motion
    autoplay_gifs: bool, 
    autoplay_animated_emoji: bool,
    
    show_send_message_button: bool, // may be easier for people who only use a mouse
    
    use_text_to_speech: bool, // have a built in screen reader read out  the interface
    tts_words_per_minnute: u8, 


    // voice chat
    voice_input_device: String,
    voice_output_device: String, 
    video_input_device: String,
    input_volume: f32,  // multipler
    output_volume: f32, // multiplier, e.g., 2 is twie as loud, etc. 

    voice_join_status: String, 
    /// muted: Always join muted
    /// unmuted: Always join unmuted
    /// ask: Always ask
    /// preserve: Preserve your previojus mute status. 

    use_push_to_talk: bool, // auto detect mic actiity when false
    input_sensativity: u8, // decibels needed for mic activity to begin sending data
    voice_gain: u8, // boost voice bby this moung
    voice_normalise: u8, // normalise your voice to this volume. 0 disables it. 

    voice_cleaning: String, // noise supression, echo cancellation, etc. 
    attenuation_threshold: i8, // Lower the volume of other applications when receving voice input. 0 disables  it. Unit: Debibels
    show_warning_when_no_mic: bool, // display a warning when no mic is deteced in vc

    // text and images
    force_create_embeds: bool, // always show other peoples links as embeds, even if they don't have permission to in the group 

    disable_reactions: bool, // hide reactions on all messages. They're not gone, you just cant see them. People can still react to your message. 

    convert_bitmoji_to_emoji: bool, // converts thngs like :) to thier emoji variant. 
    emoji_prefered_gender: String, // "person", "womman", "man" -- edits the shortcode, when you just do :shrug: it will append yourPrefrence_ to make it represent you better, for example, to :womman_shrug:
    preview_message_syntax: String, // show markdown and emoji syntax as you type
    click_to_show_spoiler: bool, // false: always show spoilered content. 

    attempt_auto_translate: bool,
    translate_engine: String, 

    //# notifcations
    desktop_notificiations: bool, 
    unread_message_badge: bool, 

    // avoid sending messaghes to thier phone when they are at thier computer. THis is the amount of inactivity needed before we will start sending notifications to thier phone. THe unit is minutes. 
    mobile_notification_timeout: u8, 

    tts_notifications: bool, 
    same_channel_notifications: bool, 
    notification_sounds: bool, 

    //# Audio sounds that play when various actions occur. These can be edited in your sound pack. 

    sound_pack: String, // FOLDER path. 

    sound_message: bool, 
    sound_deafen: bool, 
    sound_undeafen: bool, 
    sound_mute: bool, 
    sound_unmute: bool, 
    voice_disconnected: bool, 
    voice_user_join: bool, 
    voice_user_leave: bool, 
    voice_user_moved: bool,
    call_outgoing_ring: bool, 
    call_incoming_ring: bool,
    screenshare_started: bool,
    video_started: bool,
    viewer_join: bool, 
    viewer_leave: bool, 
    invited_to_speak: bool, // stages

    //# email notifcations
    email_frend_requests: bool, 
    email_invites: bool,
    email_events: bool,
    emal_group_announcements: bool,
    emal_instance_announcements: bool,
    email_bonfire_announcement: bool, // bonfire announcements

    //# keybindings

    //#
    language: String, // ISO codes, e.g., en-US (or just en), da, etc. 
    use_silly_shortcodes: bool,

    //# Diagnostic data collection
    send_data_on_crash: bool,
    send_data_on_startup: bool,
    send_data_on_bug_report: bool,

    send_data_network_area: bool, // timezone. 
    send_data_hardware: bool, 
    send_data_client: bool, 
    send_data_os: bool,

    //# OS Settings
    open_on_startup: bool, 
    start_minimised: bool, 

    //# Streamer Mode
    streamer_enabled: bool,
    streamer_streaming_button: bool, 
    streamer_show_status: bool, // provide a button for people to watch your stream when you are lived!
    streamer_stream_url: String, 
    streamer_auto_toggle: bool, // detect when your screen is being recorded and enable it
    streamer_hide_personal_info: bool,
    streamer_hide_invite_links: bool,
    streamer_disable_sounds: bool,
    streamer_disable_notifications: bool,
    streamer_hide_groups: bool,
    streamer_hide_dms: bool,

    //# advanced (Split dev mode up into MODULAR PIECES)
    developer_mode: bool, // Toggle the ability to toggle the below settings. 
    use_variable_names: bool, // use variable names for options instead of easier-to-read subsitutes
    show_raw_json: bool, 
    application_test_mode: bool, // show debug information
    prefer_branch: String, // stable (reccommended), beta (early access to new features), nightly (every built patch, WARNING: HIGHLY UNSTABLE). 
    copy_id_button: bool,
    show_user_ids: bool,
    show_debug_menu: bool,
    show_performance_stats: bool,


    date_format: String, // ISO: Defult DD/MMM/YYYY
    use_24h_time: bool, // defult: true
    timezone: String, // ISO shortcode.

    hardware_acceleration: bool, // render on the GPU. Disable if you have frame drops
    navigate_to_home: bool, // for groups with a designated home channel, go there automatically if  you have been away for a while. 
    navigate_to_home_timeout: u64, // minutes before you will be navigated to home automatically

    //# Status - can all be configred per status, just defults. 
    activity_as_status: bool, 
    share_status_in_groups: bool, // can be configured per-group. this is just a defult. 
    allow_join_game: bool, // allow friends to join yur game via rich status

    auto_update: bool,
    auto_display_patch_notes: bool, // display patch notes on first statup snce an update

}

#[derive(Properties, PartialEq)]
pub struct Message {
    author: Guid,
    guid: Guid, 
    content: String,
    // reactions: Vec,

    // channel: Guid,
    // reply_to: Guid,

    // posted_time: String,  // DD-MMM-YYYY HH:MM:SS (07-FEB-2023 14:12:23). All times are converted to UTC.
    // edited_time: String 
}

pub struct Channel {
    guid: Guid,     // Chanels GUID
    parent: Guid,   // Category GUID (If uncategorised: Hub GUID)
    name: String,
    topic: String,

    channel_type: String,  // Only specific values are valid. See the docs for more info

    permissions: Permissions, // for each role that has overrides, and the channels overrides.
}

pub struct Group {
    guid: Guid,
    name: String,
    icon: String, // URI

    // roles: Vec, 
    permissions: Permissions
}

pub struct Role {
    guid: Guid,
    name: String,
    key: String,
    badge: String, // URI
    permissions: Permissions,

    // channel_overrides: Vec // Diffrent permission objects for each channel.
}