//! Type-safe wrappers for SimpleX Chat integer IDs and conversions from API structs.
//!
//!  ID types implement `From` for their corresponding API structs(and references to them), so you can pass a
//! `&Contact`, `GroupInfo`, `ChatItem`, etc. directly wherever a typed ID is expected.

use simploxide_api_types::{
    AChatItem, CIFile, CIMeta, ChatInfo, ChatItem, ChatRef, ChatType, Contact, FileTransferMeta,
    GroupChatScope, GroupInfo, GroupMember, GroupRelay, RcvFileTransfer, SndFileTransfer, User,
    UserContactRequest, UserInfo,
};

use std::num::NonZeroI64;

macro_rules! typesafe_ids {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            pub struct $name(NonZeroI64);

            impl $name {
                /// # Safety
                ///
                /// All SimpleX IDs are starting from 1 which is guaranteed by Sqlite and
                /// PostgreSQL DBs so it is generally safe to call this method when passing an
                /// ID value received from SimpleX backend.
                ///
                /// In other cases of if in doubts - use [from_raw](Self::from_raw) for version that panics or
                /// [Self::try_from] for version that returns an error.
                pub unsafe fn from_raw_unchecked(id: i64) -> Self {
                    unsafe {
                        Self(NonZeroI64::new_unchecked(id))
                    }
                }

                /// Panics when `id == 0`. Use [Self::try_from] to handle an error
                pub fn from_raw(id: i64) -> Self {
                    Self(NonZeroI64::try_from(id).unwrap())
                }

                pub fn raw(&self) -> i64 {
                    self.0.get()
                }
            }

            impl TryFrom<i64> for $name {
                type Error = Zero;

                fn try_from(id: i64) -> Result<Self, Self::Error> {
                    let id = NonZeroI64::new(id).ok_or(Zero(stringify!($name)))?;
                    Ok(Self(id))
                }
            }

            impl From<NonZeroI64> for $name {
                fn from(id: NonZeroI64) -> Self {
                    Self(id)
                }
            }

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl ::std::str::FromStr for $name {
                type Err = ::std::num::ParseIntError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(Self(s.parse()?))
                }
            }
        )*
    }
}

typesafe_ids!(
    UserId,
    ContactId,
    ContactRequestId,
    GroupId,
    FileId,
    MessageId,
    MemberId,
    RelayId
);

/// Identifies a chat: direct contact, group (optionally scoped to a member support thread),
/// or local note-to-self.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChatId {
    Direct(ContactId),
    Group {
        id: GroupId,
        scope: Option<MemberId>,
    },
    Local(UserId),
}

impl ChatId {
    /// Creates a [`ChatId::Group`] scoped to a member support thread.
    pub fn with_group_scope(id: GroupId, group_member_support_id: MemberId) -> Self {
        Self::Group {
            id,
            scope: Some(group_member_support_id),
        }
    }

    /// Converts a [`ChatRef`] from a SimpleX API response. Returns `None` for unrecognised chat types.
    pub fn from_chat_ref(chat_ref: &ChatRef) -> Option<Self> {
        match chat_ref.chat_type {
            ChatType::Direct => Some(Self::Direct(unsafe {
                ContactId::from_raw_unchecked(chat_ref.chat_id)
            })),
            ChatType::Group => Some(Self::Group {
                id: unsafe { GroupId::from_raw_unchecked(chat_ref.chat_id) },
                scope: chat_ref.chat_scope.as_ref().and_then(|scope| {
                    scope.member_support().and_then(|id| {
                        id.as_ref()
                            .copied()
                            .map(|id| unsafe { MemberId::from_raw_unchecked(id) })
                    })
                }),
            }),
            ChatType::Local => Some(Self::Local(unsafe {
                UserId::from_raw_unchecked(chat_ref.chat_id)
            })),
            _ => None,
        }
    }

    /// Converts a [`ChatInfo`] from a SimpleX API response. Returns `None` for unrecognised chat types.
    pub fn from_chat_info(chat_info: &ChatInfo) -> Option<Self> {
        match chat_info {
            ChatInfo::Direct { contact, .. } => Some(Self::Direct(ContactId::from(contact))),
            ChatInfo::Group {
                group_info,
                group_chat_scope,
                ..
            } => Some(Self::Group {
                id: GroupId::from(group_info),
                scope: group_chat_scope.as_ref().and_then(|scope| {
                    scope
                        .member_support()
                        .and_then(|member| member.as_ref().map(MemberId::from))
                }),
            }),
            ChatInfo::Local { note_folder, .. } => Some(Self::Local(unsafe {
                UserId::from_raw_unchecked(note_folder.user_id)
            })),
            _ => None,
        }
    }

    /// Converts back into a [`ChatRef`] for use in raw API calls.
    pub fn into_chat_ref(self) -> ChatRef {
        let (chat_type, chat_id, chat_scope) = match self {
            Self::Direct(contact_id) => (ChatType::Direct, contact_id.raw(), None),
            Self::Group {
                id: group_id,
                scope,
            } => (
                ChatType::Group,
                group_id.raw(),
                scope.map(|member_id| GroupChatScope::MemberSupport {
                    group_member_id: Some(member_id.raw()),
                    undocumented: Default::default(),
                }),
            ),
            Self::Local(user_id) => (ChatType::Local, user_id.raw(), None),
        };

        ChatRef {
            chat_type,
            chat_id,
            chat_scope,
            undocumented: Default::default(),
        }
    }

    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Direct(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group { .. })
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }
}

impl From<ContactId> for ChatId {
    fn from(id: ContactId) -> Self {
        Self::Direct(id)
    }
}

impl From<GroupId> for ChatId {
    fn from(id: GroupId) -> Self {
        Self::Group { id, scope: None }
    }
}

impl From<UserId> for ChatId {
    fn from(id: UserId) -> Self {
        Self::Local(id)
    }
}

#[derive(Debug)]
pub struct Zero(&'static str);

impl std::fmt::Display for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Got {} equal to zero", self.0)
    }
}

impl std::error::Error for Zero {}

macro_rules! impl_id_from_struct {
    ($strct:ty as $id:ty, $val:ident, $conversion:expr) => {
        impl From<$strct> for $id {
            fn from($val: $strct) -> Self {
                $conversion
            }
        }

        impl<'a> From<&'a $strct> for $id {
            fn from($val: &'a $strct) -> Self {
                $conversion
            }
        }

        impl<'a> From<&'a mut $strct> for $id {
            fn from($val: &'a mut $strct) -> Self {
                $conversion
            }
        }
    };
}

impl_id_from_struct!(User as UserId, user, unsafe {
    UserId::from_raw_unchecked(user.user_id)
});
impl_id_from_struct!(UserInfo as UserId, info, UserId::from(&info.user));

impl_id_from_struct!(Contact as ContactId, contact, unsafe {
    ContactId::from_raw_unchecked(contact.contact_id)
});
impl_id_from_struct!(Contact as ChatId, contact, ContactId::from(contact).into());

impl_id_from_struct!(UserContactRequest as ContactRequestId, req, unsafe {
    ContactRequestId::from_raw_unchecked(req.contact_request_id)
});

impl_id_from_struct!(GroupInfo as GroupId, group, unsafe {
    GroupId::from_raw_unchecked(group.group_id)
});
impl_id_from_struct!(GroupInfo as ChatId, group, GroupId::from(group).into());

impl_id_from_struct!(CIMeta as MessageId, meta, unsafe {
    MessageId::from_raw_unchecked(meta.item_id)
});
impl_id_from_struct!(ChatItem as MessageId, item, MessageId::from(&item.meta));
impl_id_from_struct!(AChatItem as MessageId, it, MessageId::from(&it.chat_item));

impl_id_from_struct!(CIFile as FileId, file, unsafe {
    FileId::from_raw_unchecked(file.file_id)
});
impl_id_from_struct!(RcvFileTransfer as FileId, ft, unsafe {
    FileId::from_raw_unchecked(ft.file_id)
});
impl_id_from_struct!(FileTransferMeta as FileId, ft, unsafe {
    FileId::from_raw_unchecked(ft.file_id)
});
impl_id_from_struct!(SndFileTransfer as FileId, ft, unsafe {
    FileId::from_raw_unchecked(ft.file_id)
});

impl_id_from_struct!(GroupMember as MemberId, member, unsafe {
    MemberId::from_raw_unchecked(member.group_member_id)
});
impl_id_from_struct!(GroupRelay as RelayId, relay, unsafe {
    RelayId::from_raw_unchecked(relay.group_relay_id)
});
