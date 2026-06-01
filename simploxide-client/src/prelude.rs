//! Re-exports everything that is needed to send commands, match events and destructure responses

pub use crate::{
    ClientApi as _, StreamEvents,
    bot::{BotProfileSettings, BotSettings, Connection},
    client_api::*,
    commands::*,
    events::*,
    ext::{
        ClientApiExt as _, DeleteMode, FileSourceExt as _, FilterChatItems as _, GroupLinkExt as _,
        Reaction,
    },
    id::*,
    messages::*,
    preferences,
    preview::ImagePreview,
    responses::*,
    types::{
        AddressSettings, CIContent, CIDeleteMode, CIFile, ChatBotCommand, ChatDeleteMode, ChatInfo,
        ChatPeerType, ChatRef, ChatType, ComposedMessage, CreatedConnLink, CryptoFile,
        CryptoFileArgs, FeatureAllowed, GroupInfo, GroupMember, GroupMemberRole, GroupProfile,
        GroupRelay, JsonObject, MsgContent, MsgReaction, NewUser, Preferences, Profile,
        SimplePreference, UpdatedMessage, User, UserInfo,
    },
};
