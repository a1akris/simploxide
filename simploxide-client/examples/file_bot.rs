//! The example expects that SimpleX CLI is running on port `5225`. Run CLI with:
//! `simplex-chat -p 5225 --create-bot-display-name Default`
//!
//! To compile this example pass the --features flag like this:
//! `cargo run --example file_bot --features websocket`
//!
//! ----
//!
//! An example showing how to handle file events manually. The bot receives files and sends them
//! back to the user. The example also demonstrates the usage of the BotBuilder::connect method
//! that allows to connect to running SimpleX-CLI instance instead of launching it
//! programmatically(useful for debugging)

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
        .auto_accept()
        .connect()
        .await?;

    let address = bot.get_or_create_address().await?;
    println!("Bot address: {address}");

    events
        .into_dispatcher(bot)
        .on(recv_file_error)
        .on(recv_file_cancelled)
        .on(recv_file_complete)
        .on(send_file_error)
        .on(send_file_complete)
        .on(new_msgs)
        .dispatch()
        .await?;

    Ok(())
}

async fn new_msgs(ev: Arc<NewChatItems>, bot: ws::Bot) -> ws::ClientResult<StreamEvents> {
    for (cid, it, _) in ev.chat_items.filter_messages() {
        if !cid.is_direct() {
            // Support only direct conversations.
            return Ok(StreamEvents::Continue);
        }

        if it.meta.item_text.trim() == "/die" {
            return Ok(StreamEvents::Break);
        }

        let Some(file) = &it.file else {
            bot.send_msg(cid, "Hey, send me some files!").await?;
            return Ok(StreamEvents::Continue);
        };

        if file.file_size > 5 * 1024 * 1024 {
            bot.send_msg(cid, "Sorry, but the file must be <5MiB")
                .await?;

            bot.reject_file(file).await?;
        } else {
            bot.accept_file(file).await?;
        }
    }

    Ok(StreamEvents::Continue)
}

async fn recv_file_cancelled(
    ev: Arc<RcvFileSndCancelled>,
    bot: ws::Bot,
) -> ws::ClientResult<StreamEvents> {
    let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
        // Cannot operate in groups
        return Ok(StreamEvents::Continue);
    };

    bot.send_msg(
        contact,
        "I cannot process files if you keep cancelling them!",
    )
    .reply_to(&ev.chat_item)
    .await?;

    Ok(StreamEvents::Continue)
}

async fn recv_file_complete(
    ev: Arc<RcvFileComplete>,
    bot: ws::Bot,
) -> ws::ClientResult<StreamEvents> {
    println!("Received file:\n{:#?}", ev.chat_item.chat_item);

    let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
        // Cannot operate in groups
        return Ok(StreamEvents::Continue);
    };

    let crypto_file = ev
        .chat_item
        .chat_item
        .file
        .as_ref()
        .and_then(|x| x.file_source.clone())
        .unwrap();

    bot.send_msg(contact, "Take it back!")
        .attach(crypto_file)
        .reply_to(&ev.chat_item)
        .await?;

    Ok(StreamEvents::Continue)
}

async fn recv_file_error(ev: Arc<RcvFileError>, bot: ws::Bot) -> ws::ClientResult<StreamEvents> {
    eprintln!("Error receiving a file:\n{ev:#?}");

    if let Some(ChatInfo::Direct { contact, .. }) = ev.chat_item.as_ref().map(|c| &c.chat_info) {
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
}

async fn send_file_complete(
    ev: Arc<SndFileCompleteXftp>,
    bot: ws::Bot,
) -> ws::ClientResult<StreamEvents> {
    println!("Sent file to a user:\n{:#?}", ev.file_transfer_meta);

    let ChatInfo::Direct { ref contact, .. } = ev.chat_item.chat_info else {
        // Cannot operate in groups
        return Ok(StreamEvents::Continue);
    };

    bot.send_msg(contact, "Gimme more!").await?;
    Ok(StreamEvents::Continue)
}

async fn send_file_error(ev: Arc<SndFileError>, bot: ws::Bot) -> ws::ClientResult<StreamEvents> {
    eprintln!("Error sending a file:\n{ev:#?}");

    if let Some(ChatInfo::Direct { contact, .. }) = ev.chat_item.as_ref().map(|c| &c.chat_info) {
        bot.send_msg(
            contact,
            format!(
                "Failed to send back the {} due to an error {:#?}",
                ev.file_transfer_meta.file_name, ev.error_message
            ),
        )
        .await?;
    }

    Ok(StreamEvents::Continue)
}
