use simploxide_api_types::{ChatInfo, ChatRef, ChatType, Contact, GroupChatScope};

macro_rules! typesafe_ids {
    ($($name:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(transparent)]
            pub struct $name(pub i64);

            impl From<$name> for i64 {
                fn from(id: $name) -> i64 {
                    id.0
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

typesafe_ids!(UserId, ContactId, GroupId, FileId, MessageId, MemberId);

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
    pub fn with_group_scope(id: GroupId, group_member_support_id: MemberId) -> Self {
        Self::Group {
            id,
            scope: Some(group_member_support_id),
        }
    }

    pub fn from_chat_ref(chat_ref: &ChatRef) -> Option<Self> {
        match chat_ref.chat_type {
            ChatType::Direct => Some(Self::Direct(ContactId(chat_ref.chat_id))),
            ChatType::Group => Some(Self::Group {
                id: GroupId(chat_ref.chat_id),
                scope: chat_ref.chat_scope.as_ref().and_then(|scope| {
                    scope
                        .member_support()
                        .and_then(|id| id.as_ref().copied().map(MemberId))
                }),
            }),
            ChatType::Local => Some(Self::Local(UserId(chat_ref.chat_id))),
            _ => None,
        }
    }

    pub fn from_contact(contact: &Contact) -> Self {
        ContactId(contact.contact_id).into()
    }

    pub fn from_chat_info(chat_info: &ChatInfo) -> Option<Self> {
        match chat_info {
            ChatInfo::Direct { contact, .. } => Some(Self::Direct(ContactId(contact.contact_id))),
            ChatInfo::Group {
                group_info,
                group_chat_scope,
                ..
            } => Some(Self::Group {
                id: GroupId(group_info.group_id),
                scope: group_chat_scope.as_ref().and_then(|scope| {
                    scope
                        .member_support()
                        .and_then(|member| member.as_ref().map(|member| MemberId(member.group_id)))
                }),
            }),
            ChatInfo::Local { note_folder, .. } => Some(Self::Local(UserId(note_folder.user_id))),
            _ => None,
        }
    }

    pub fn into_chat_ref(self) -> ChatRef {
        let (chat_type, chat_id, chat_scope) = match self {
            Self::Direct(contact_id) => (ChatType::Direct, contact_id.0, None),
            Self::Group {
                id: group_id,
                scope,
            } => (
                ChatType::Group,
                group_id.0,
                scope.map(|member_id| GroupChatScope::MemberSupport {
                    group_member_id: Some(member_id.0),
                    undocumented: Default::default(),
                }),
            ),
            Self::Local(user_id) => (ChatType::Local, user_id.0, None),
        };

        ChatRef {
            chat_type,
            chat_id,
            chat_scope,
            undocumented: Default::default(),
        }
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
