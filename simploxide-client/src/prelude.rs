//! Re-exports everything that is needed to send commands, match events and destructure responses

pub use crate::{
    ClientApi as _, StreamEvents,
    client_api::*,
    commands::*,
    events::*,
    ext::{ClientApiExt as _, DeleteMode, FilterChatItems as _},
    id::*,
    messages::*,
    preferences,
    responses::*,
    types::{
        AddressSettings, CIContent, CIDeleteMode, ChatBotCommand, ChatDeleteMode, ChatInfo,
        ChatPeerType, ChatRef, ChatType, ComposedMessage, CreatedConnLink, CryptoFile,
        CryptoFileArgs, FeatureAllowed, GroupMemberRole, GroupProfile, MsgContent, MsgReaction,
        NewUser, Preferences, Profile, SimplePreference, UpdatedMessage, User, UserInfo,
    },
};
