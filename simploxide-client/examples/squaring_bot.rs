//! To compile this example pass the --features flag like this:
//! `cargo run --example squaring_bot --features bon,cli`
//!
//! The examples expects that SimpleX-CLI is installed locally on the system and is available via
//! $PATH or at the `simploxide/simploxide-client` directory(you can symlink it there)
//!
//! ----
//!
//! A bot that receives a number and sends back its square.

use simploxide_client::{
    StreamEvents,
    ext::FilterChatItems as _,
    id::ChatId,
    prelude::*,
    ws::{self, ClientResult},
};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (bot, events, mut cli) = ws::BotBuilder::new("SimplOxide Examples", 5225)
        .db_prefix("test_db/bot")
        .launch()
        .await?;

    let address = bot.get_or_create_address().await?;
    println!("Bot address: {address}");

    let connection = events
        .into_dispatcher(bot)
        .on(new_contact_request)
        .on(contact_connected)
        .on(new_msgs)
        .dispatch()
        .await?;

    drop(connection);
    cli.kill().await?;
    Ok(())
}

async fn contact_connected(ev: Arc<ContactConnected>, bot: ws::Bot) -> ClientResult<StreamEvents> {
    println!("{} connected", ev.contact.profile.display_name);

    reply(
        bot.client(),
        &ChatId::from_contact(&ev.contact),
        "Hello! I am a simple squaring bot - if you send me a number, I will calculate its square"
            .to_owned(),
    )
    .await?;

    Ok(StreamEvents::Continue)
}

async fn new_contact_request(
    ev: Arc<ReceivedContactRequest>,
    bot: ws::Bot,
) -> ClientResult<StreamEvents> {
    bot.client()
        .api_accept_contact(ev.contact_request.contact_request_id)
        .await?;

    Ok(StreamEvents::Continue)
}

async fn new_msgs(ev: Arc<NewChatItems>, bot: ws::Bot) -> ClientResult<StreamEvents> {
    // SimpleX sends a lot of utility messages like changed preferences and connection events.
    // These fake messages must be filtered out, we're interested only in real user messages
    for (cid, it, content) in ev.chat_items.filter_messages() {
        if it.meta.item_text.trim() == "/die" {
            return Ok(StreamEvents::Break);
        }

        if let Some(num) = content
            .text()
            .and_then(|txt| txt.trim().parse::<i64>().ok())
        {
            reply(
                bot.client(),
                &cid,
                format!("Squared: {}", num.wrapping_mul(num)),
            )
            .await?;
        } else {
            reply(bot.client(), &cid, "Me understands only numbers!").await?;
        }
    }

    Ok(StreamEvents::Continue)
}

// The client API is quite low level so helper functions are often required to deal with common bot
// actions.
async fn reply(
    client: &ws::Client,
    chat_id: &ChatId,
    reply: impl Into<String>,
) -> ClientResult<Arc<NewChatItemsResponse>> {
    // Use bon builders to build complicated requests. Availaible behind the "bon" feature
    // flag.
    client
        .api_send_messages(
            ApiSendMessages::builder()
                .send_ref(chat_id.into_chat_ref())
                .live_message(false)
                .composed_messages(vec![
                    ComposedMessage::builder()
                        .msg_content(MsgContent::make_text(reply.into()))
                        .mentions(Default::default())
                        .build(),
                ])
                .build(),
        )
        .await
}
