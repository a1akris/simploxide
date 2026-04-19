//! The example expects that SimpleX CLI is running on port `5225`. Use `Spawn CLI with
//! simplex-chat -p 5225 --create-bot-display-name Default`
//!
//! To compile this example pass the --features flag like this:
//! `cargo run --example file_bot --features websocket`
//!
//! ----
//!
//! A bot that receives files and sends them back to the user.

use simploxide_client::{StreamEvents, preferences, prelude::*, ws};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (bot, events) = ws::BotBuilder::new("SimplOxide Examples", 5225)
        .with_preferences(Preferences {
            timed_messages: preferences::timed_messages::NO,
            full_delete: preferences::YES,
            reactions: preferences::NO,
            voice: preferences::NO,
            files: preferences::YES,
            calls: preferences::NO,
            sessions: preferences::NO,
            commands: None,
            undocumented: Default::default(),
        })
        .connect()
        .await?;

    let address = bot.get_or_create_address().await?;
    println!("Bot address: {address}");

    events
        .into_dispatcher(bot)
        .on(
            async |ev: Arc<ContactConnected>, bot| -> ws::ClientResult<_> {
                println!("{} connected", ev.contact.profile.display_name);

                bot.send_msg(
                    &ev.contact,
                    "Hello! I am a simple file bot - if you send me a file, I will send it back!",
                )
                .await?;

                Ok(StreamEvents::Continue)
            },
        )
        .on(async |ev: Arc<ReceivedContactRequest>, bot| {
            bot.client()
                .api_accept_contact(ev.contact_request.contact_request_id)
                .await?;

            println!(
                "Accepted user: {} ({})",
                ev.contact_request.profile.display_name, ev.contact_request.profile.full_name
            );

            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<RcvFileDescrReady>, bot| {
            let file_info = ev
                .chat_item
                .chat_item
                .file
                .as_ref()
                .expect("The file field must be present in RcvFileDescrReady event");

            let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
                // Cannot operate in groups
                return Ok(StreamEvents::Continue);
            };

            if file_info.file_size > 5 * 1024 * 1024 {
                bot.send_msg(contact, "Sorry, but the file must be <5MiB")
                    .await?;

                bot.client().cancel_file(file_info.file_id).await?;
                println!("File delivery cancelled: {file_info:#?}");
            } else {
                // Spawns a background file download.
                bot.client().recv_file(file_info.file_id).await;
            }

            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<RcvFileError>, bot| {
            eprintln!("Error receiving a file:\n{ev:#?}");

            if let Some(ChatInfo::Direct { contact, .. }) =
                ev.chat_item.as_ref().map(|c| &c.chat_info)
            {
                bot.send_msg(
                    contact,
                    format!(
                        "Failed to receive the {} due to this horrible error {:#?}",
                        ev.rcv_file_transfer.file_invitation.file_name, ev.agent_error
                    ),
                )
                .await?;
            }

            Ok(StreamEvents::Continue)
        })
        .on::<RcvFileWarning, _, _>(async |ev, bot| {
            eprintln!("Failure receiving a file:\n{ev:#?}");

            if let Some(ChatInfo::Direct { contact, .. }) =
                ev.chat_item.as_ref().map(|c| &c.chat_info)
            {
                bot.send_msg(
                    contact,
                    format!(
                        "Failed to receive the {} due to {:?}",
                        ev.rcv_file_transfer.file_invitation.file_name, ev.agent_error
                    ),
                )
                .await?;
            }
            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<RcvFileComplete>, bot| {
            println!("Received file:\n{:#?}", ev.chat_item.chat_item);

            let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
                // Cannot operate in groups
                return Ok(StreamEvents::Continue);
            };

            let file_client = bot.client().clone();
            let contact_id = contact.contact_id;
            let msg_id = ev.chat_item.chat_item.meta.item_id;
            let crypto_file = ev
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
            file_client
                .send_file(contact_id, msg_id, "Take it back!", crypto_file)
                .await;
            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<SndFileCompleteXftp>, bot| {
            println!("Returned file to a user:\n{:#?}", ev.file_transfer_meta);

            let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
                // Cannot operate in groups
                return Ok(StreamEvents::Continue);
            };

            bot.send_msg(contact, "Gimme more!").await?;

            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<SndFileWarning>, bot| {
            eprintln!("Failure sending a file:\n{ev:#?}");

            if let Some(ChatInfo::Direct { contact, .. }) =
                ev.chat_item.as_ref().map(|c| &c.chat_info)
            {
                bot.send_msg(
                    contact,
                    format!(
                        "Failed to send back the {} due to {:?}",
                        ev.file_transfer_meta.file_name, ev.error_message
                    ),
                )
                .await?;
            }

            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<SndFileError>, bot| {
            eprintln!("Error sending a file:\n{ev:#?}");

            if let Some(ChatInfo::Direct { contact, .. }) =
                ev.chat_item.as_ref().map(|c| &c.chat_info)
            {
                bot.send_msg(
                    contact,
                    format!(
                        "Failed to send back the {} due to this horrible error {:#?}",
                        ev.file_transfer_meta.file_name, ev.error_message
                    ),
                )
                .await?;
            }

            Ok(StreamEvents::Continue)
        })
        .on(async |ev: Arc<NewChatItems>, bot| {
            // Initially, you will receive a file as a new chat item. Then you will start to
            // receive RcvFile* events.
            //
            // You can call receive/cancel directly here without waiting for the
            // RcvFileDescrReady event but you will get `RcvFileAcceptedSndCancelled` response
            // if the user cancels the transmission at this stage while receiving a file after
            // the `RcvFileDescrReady` event normally succeeds.
            println!(
                "New chat items:\n{:#?}",
                ev.chat_items
                    .iter()
                    .map(|c| &c.chat_item)
                    .collect::<Vec<_>>()
            );

            for it in ev.chat_items.iter() {
                if it.chat_item.meta.item_text.trim() == "/die" {
                    return Ok(StreamEvents::Break);
                }

                if it.chat_item.file.is_none() {
                    let ChatInfo::Direct { ref contact, .. } = it.chat_info else {
                        // Cannot operate in groups
                        continue;
                    };

                    bot.send_msg(contact, "Hey, send me some files!").await?;
                }
            }

            Ok(StreamEvents::Continue)
        })
        .dispatch()
        .await?;

    Ok(())
}

/// One of the ways to conveniently provide helper methods to your bot is to define them in a trait
/// and implement the trait for [`simploxide_client::Client`].
trait BotExtensions {
    async fn send_file(
        &self,
        chat_id: i64,
        in_rely_to: i64,
        txt: impl Into<String>,
        file: CryptoFile,
    );

    async fn recv_file(&self, file_id: i64);
}

impl BotExtensions for ws::Client {
    async fn send_file(
        &self,
        chat_id: i64,
        in_reply_to: i64,
        txt: impl Into<String>,
        file: CryptoFile,
    ) {
        let client = self.clone();
        let text = txt.into();
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
    }

    async fn recv_file(&self, file_id: i64) {
        let client = self.clone();
        let _ = client
            .receive_file(ReceiveFile {
                file_id,
                user_approved_relays: false,
                store_encrypted: Some(true),
                file_inline: None,
                file_path: None,
            })
            .await;
    }
}
