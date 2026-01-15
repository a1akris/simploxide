//! The example expects that the bot account was already pre-created via CLI by `create bot
//! <bot_name> <bot_fullname>` command.
//!
//! To compile this example pass the --all-features flag like this: `cargo run --example file_bot
//! --all-features`.
//!
//! ----
//!
//! A bot that receives files and sends them back to the user. The interesting thing here is
//! that files are stored encrypted on your device(you can find them in the `/tmp` directory).
//! Files larger than 5MiB are rejected by the bot.
//!
//! NOTE: If your bot account hadn't enabled the "files" preference before running this example,
//! then it may take your SimpleX Client several minutes after launching this bot to recognize this
//! preference and show you the file upload icon.

use futures::{TryFutureExt as _, TryStreamExt as _};
use simploxide_client::{
    prelude::*,
    types::{CryptoFile, FeatureAllowed, SimplePreference},
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (client, mut events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;

    let user = client.show_active_user().await?;
    println!(
        "Bot profile: {} ({})",
        user.profile.display_name, user.profile.full_name
    );

    // Get or create the bot address
    let (address_long, address_short) = client
        .api_show_my_address(user.user_id)
        .map_ok(|resp| {
            (
                resp.contact_link.conn_link_contact.conn_full_link.clone(),
                resp.contact_link.conn_link_contact.conn_short_link.clone(),
            )
        })
        .or_else(|_| {
            client.api_create_my_address(user.user_id).map_ok(|resp| {
                (
                    resp.conn_link_contact.conn_full_link.clone(),
                    resp.conn_link_contact.conn_short_link.clone(),
                )
            })
        })
        .await?;

    println!("Bot long address: {address_long}");
    println!("Bot short address: {address_short:?}");

    // Allow file operations
    //
    // NOTE: It may take several minutes for your SimpleX Client to recognize this update and show
    // a file upload icon.
    client
        .api_update_profile(
            user.user_id,
            Profile::builder()
                .display_name(user.profile.display_name.clone())
                .full_name(user.profile.full_name.clone())
                .preferences(
                    Preferences::builder()
                        .files(
                            SimplePreference::builder()
                                .allow(FeatureAllowed::Yes)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .await?;

    'reactor: while let Some(ev) = events.try_next().await? {
        match ev.as_ref() {
            // A new user connected
            Event::ContactConnected(connected) => {
                println!("{} connected", connected.contact.profile.display_name);

                client.send_text(
                    connected.contact.contact_id,
                    "Hello! I am a simple file bot - if you send me a file, I will send it back!",
                );
            }
            Event::NewChatItems(new_msgs) => {
                // Initially, you will receive a file as a new chat item. Then you will start to
                // receive RcvFile* events.
                //
                // You can call receive/cancel directly here without waiting for the
                // RcvFileDescrReady event but you will get `RcvFileAcceptedSndCancelled` response
                // if the user cancels the transmission at this stage while receiving a file after
                // the `RcvFileDescrReady` event normally succeeds.
                println!(
                    "New chat items:\n{:#?}",
                    new_msgs
                        .chat_items
                        .iter()
                        .map(|c| &c.chat_item)
                        .collect::<Vec<_>>()
                );

                for it in new_msgs.chat_items.iter() {
                    if it.chat_item.meta.item_text.trim() == "/die" {
                        break 'reactor;
                    }

                    if it.chat_item.file.is_none() {
                        let ChatInfo::Direct { ref contact, .. } = it.chat_info else {
                            // Cannot operate in groups
                            continue;
                        };

                        client.send_text(contact.contact_id, "Hey, send me some files!")
                    }
                }
            }
            // This event signals that the file is ready for download.
            Event::RcvFileDescrReady(descr) => {
                let file_info = descr
                    .chat_item
                    .chat_item
                    .file
                    .as_ref()
                    .expect("The file field must be present in RcvFileDescrReady event");

                let ChatInfo::Direct { ref contact, .. } = descr.chat_item.chat_info else {
                    // Cannot operate in groups
                    continue;
                };

                if file_info.file_size > 5 * 1024 * 1024 {
                    client.send_text(contact.contact_id, "Sorry, but the file must be <5MiB");
                    client.cancel_file(file_info.file_id).await?;
                    println!("File delivery cancelled: {file_info:#?}");
                } else {
                    // Spawns a background file download.
                    client.recv_file(file_info.file_id);
                }
            }
            Event::RcvFileError(e) => {
                eprintln!("Error receiving a file:\n{e:#?}");

                if let Some(ChatInfo::Direct { contact, .. }) =
                    e.chat_item.as_ref().map(|c| &c.chat_info)
                {
                    client.send_text(
                        contact.contact_id,
                        format!(
                            "Failed to receive the {} due to this horrible error {:#?}",
                            e.rcv_file_transfer.file_invitation.file_name, e.agent_error
                        ),
                    );
                }
            }
            Event::RcvFileWarning(e) => {
                eprintln!("Failure receiving a file:\n{e:#?}");

                if let Some(ChatInfo::Direct { contact, .. }) =
                    e.chat_item.as_ref().map(|c| &c.chat_info)
                {
                    client.send_text(
                        contact.contact_id,
                        format!(
                            "Failed to receive the {} due to {:?}",
                            e.rcv_file_transfer.file_invitation.file_name, e.agent_error
                        ),
                    );
                }
            }
            Event::RcvFileComplete(descr) => {
                println!("Received file:\n{:#?}", descr.chat_item.chat_item);

                let ChatInfo::Direct { ref contact, .. } = descr.chat_item.chat_info else {
                    // Cannot operate in groups
                    continue;
                };

                let file_client = client.clone();
                let contact_id = contact.contact_id;
                let msg_id = descr.chat_item.chat_item.meta.item_id;
                let crypto_file = descr
                    .chat_item
                    .chat_item
                    .file
                    .as_ref()
                    .unwrap()
                    .file_source
                    .as_ref()
                    .unwrap()
                    .clone();

                // Spawns a background file upload. The crypto_file struct has a crypto_args
                // parameter. Set it to `None` to send a file without decrypting it.
                file_client.send_file(contact_id, msg_id, "Take it back!", crypto_file)
            }
            Event::SndFileCompleteXftp(descr) => {
                println!("Returned file to a user:\n{:#?}", descr.file_transfer_meta);

                let ChatInfo::Direct { ref contact, .. } = descr.chat_item.chat_info else {
                    // Cannot operate in groups
                    continue;
                };

                client.send_text(contact.contact_id, "Gimme more!");
            }
            Event::SndFileWarning(e) => {
                eprintln!("Failure sending a file:\n{e:#?}");

                if let Some(ChatInfo::Direct { contact, .. }) =
                    e.chat_item.as_ref().map(|c| &c.chat_info)
                {
                    client.send_text(
                        contact.contact_id,
                        format!(
                            "Failed to send back the {} due to {:?}",
                            e.file_transfer_meta.file_name, e.error_message
                        ),
                    );
                }
            }
            Event::SndFileError(e) => {
                eprintln!("Error sending a file:\n{e:#?}");

                if let Some(ChatInfo::Direct { contact, .. }) =
                    e.chat_item.as_ref().map(|c| &c.chat_info)
                {
                    client.send_text(
                        contact.contact_id,
                        format!(
                            "Failed to send back the {} due to this horrible error {:#?}",
                            e.file_transfer_meta.file_name, e.error_message
                        ),
                    )
                }
            }
            Event::ReceivedContactRequest(req) => {
                client
                    .api_accept_contact(req.contact_request.contact_request_id)
                    .await?;

                println!(
                    "Accepted user: {} ({})",
                    req.contact_request.profile.display_name, req.contact_request.profile.full_name
                );
            }
            // Ignore all other events
            _ => (),
        }
    }

    Ok(())
}

/// One of the ways to conveniently provide helper methods to your bot is to define them in a trait
/// and implement the trait for [`simploxide_client::Client`].
trait BotExtensions {
    fn send_text(&self, chat_id: i64, txt: impl Into<String>);

    fn send_file(&self, chat_id: i64, in_rely_to: i64, txt: impl Into<String>, file: CryptoFile);

    fn recv_file(&self, file_id: i64);
}

impl BotExtensions for simploxide_client::Client {
    fn send_text(&self, chat_id: i64, txt: impl Into<String>) {
        let client = self.clone();
        let text = txt.into();

        // Spawn the message sending in a background to not block the main reactor loop
        tokio::spawn(async move {
            // It's not critical if bot fails to send text messages in this scenario, so the errors
            // may be ignored.
            let _ = client
                // An example of a request constructed without the use of builders.
                .api_send_messages(ApiSendMessages {
                    send_ref: ChatRef {
                        chat_type: ChatType::Direct,
                        chat_id,
                        chat_scope: None,
                        undocumented: Default::default(),
                    },
                    live_message: false,
                    ttl: None,
                    composed_messages: vec![ComposedMessage {
                        file_source: None,
                        quoted_item_id: None,
                        msg_content: MsgContent::Text {
                            text,
                            undocumented: Default::default(),
                        },
                        mentions: Default::default(),
                        undocumented: Default::default(),
                    }],
                })
                .await;
        });
    }

    fn send_file(&self, chat_id: i64, in_reply_to: i64, txt: impl Into<String>, file: CryptoFile) {
        let client = self.clone();
        let text = txt.into();

        // Spawning a background file upload task
        tokio::spawn(async move {
            // Ignoring the response because we will get the same data in the
            // SndFileCompleteXftp/SndFileError events.
            let _ = client
                .api_send_messages(ApiSendMessages {
                    send_ref: ChatRef {
                        chat_type: ChatType::Direct,
                        chat_id,
                        chat_scope: None,
                        undocumented: Default::default(),
                    },
                    live_message: false,
                    ttl: None,
                    composed_messages: vec![ComposedMessage {
                        file_source: Some(file),
                        quoted_item_id: Some(in_reply_to),
                        msg_content: MsgContent::File {
                            text,
                            undocumented: Default::default(),
                        },
                        mentions: Default::default(),
                        undocumented: Default::default(),
                    }],
                })
                .await;
        });
    }

    fn recv_file(&self, file_id: i64) {
        let client = self.clone();

        // Spawning a background file download task
        tokio::spawn(async move {
            // Ignoring the response because we will get the same data in the
            // RcvFileComplete/RcvFileError events.
            let _ = client
                .receive_file(
                    ReceiveFile::builder()
                        .file_id(file_id)
                        .user_approved_relays(false)
                        .store_encrypted(true)
                        .build(),
                )
                .await;
        });
    }
}
