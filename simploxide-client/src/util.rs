use serde::Deserialize;
use serde_aux::prelude::*;

pub fn cast_file_size(file_size: u64) -> std::io::Result<usize> {
    file_size.try_into().map_err(file_is_too_large)
}

pub fn file_is_too_large<E>(reason: E) -> std::io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    std::io::Error::new(std::io::ErrorKind::FileTooLarge, reason)
}

#[derive(Deserialize)]
pub struct TypeField<'a> {
    #[serde(rename = "type", borrow)]
    pub typ: &'a str,
}

#[derive(Deserialize)]
pub struct UserField {
    pub user: UserIdField,
}

#[derive(Deserialize)]
pub struct UserIdField {
    #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct RelaysResp {
    #[serde(rename = "userServers")]
    pub user_servers: Vec<UserServerGroup>,
}

#[derive(Deserialize)]
pub struct UserServerGroup {
    #[serde(rename = "chatRelays", default)]
    pub chat_relays: Vec<ChatRelay>,
}

#[derive(Deserialize)]
pub struct ChatRelay {
    #[serde(
        rename = "chatRelayId",
        deserialize_with = "deserialize_number_from_string"
    )]
    pub chat_relay_id: i64,
    #[serde(default)]
    pub enabled: bool,
}
