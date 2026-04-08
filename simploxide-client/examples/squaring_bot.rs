//! To compile this example pass the --features flag like this: `cargo run --example squaring_bot
//! --features bon,cli`
//!
//! The examples expects that SimpleX-CLI is installed locally on the system and is available via
//! $PATH or at the `simploxide/simploxide-client` directory(you can symlink it there)
//!
//! ----
//!
//! A bot that receives a number and sends back its square.

use futures::TryFutureExt as _;
use simploxide_client::{
    StreamEvents,
    prelude::*,
    ws::{self, ClientResult},
};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Current dir: {}",
        std::env::current_dir().unwrap().display()
    );

    let mut cli = ws::cli::SimplexCli::builder("SimplOxide Examples", 5225)
        .db_prefix("test_db/bot")
        .spawn()
        .await?;

    let (client, events) =
        ws::retry_connect("ws://127.0.0.1:5225", std::time::Duration::from_secs(1), 5).await?;

    let response = client.show_active_user().await?;

    println!(
        "Bot profile: {} ({})",
        response.user.profile.display_name, response.user.profile.full_name
    );

    // Get or create the bot address
    let (address_long, address_short) = client
        .api_show_my_address(response.user.user_id)
        .map_ok(|resp| {
            (
                resp.contact_link.conn_link_contact.conn_full_link.clone(),
                resp.contact_link.conn_link_contact.conn_short_link.clone(),
            )
        })
        .or_else(|_| {
            client
                .api_create_my_address(response.user.user_id)
                .map_ok(|resp| {
                    (
                        resp.conn_link_contact.conn_full_link.clone(),
                        resp.conn_link_contact.conn_short_link.clone(),
                    )
                })
        })
        .await?;

    println!("Bot long address: {address_long}");
    println!("Bot short address: {address_short:?}");

    let connection = events
        .into_dispatcher(client)
        // .fallback(async |ev, _| {
        //     println!("{ev:?}");
        //     Ok(StreamEvents::Continue)
        // })
        .on(new_contact_request)
        .on(contact_connected)
        .on(new_msg)
        .dispatch()
        .await?;

    drop(connection);

    cli.kill().await?;
    Ok(())
}

async fn contact_connected(
    ev: Arc<ContactConnected>,
    client: ws::Client,
) -> ClientResult<StreamEvents> {
    println!("{} connected", ev.contact.profile.display_name);

    reply(
        &client,
        ev.contact.contact_id,
        "Hello! I am a simple squaring bot - if you send me a number, I will calculate its square"
            .to_owned(),
    )
    .await?;

    Ok(StreamEvents::Continue)
}

async fn new_contact_request(
    ev: Arc<ReceivedContactRequest>,
    client: ws::Client,
) -> ClientResult<StreamEvents> {
    client
        .api_accept_contact(ev.contact_request.contact_request_id)
        .await?;

    println!(
        "Accepted user: {} ({})",
        ev.contact_request.profile.display_name, ev.contact_request.profile.full_name
    );

    Ok(StreamEvents::Continue)
}

async fn new_msg(ev: Arc<NewChatItems>, client: ws::Client) -> ClientResult<StreamEvents> {
    // SimpleX sends a lot of utility messages like enabled preferences and chat
    // features. These fake messages must be filtered out, we're interested only in
    // regular text messages
    for (contact, text) in ev.chat_items.iter().filter_map(|msg| {
        let ChatInfo::Direct { ref contact, .. } = msg.chat_info else {
            return None;
        };

        let CIContent::RcvMsgContent {
            msg_content: MsgContent::Text { ref text, .. },
            ..
        } = msg.chat_item.content
        else {
            return None;
        };

        Some((contact, text))
    }) {
        if text.trim() == "/die" {
            return Ok(StreamEvents::Break);
        }

        if let Ok(num) = text.trim().parse::<i64>() {
            reply(
                &client,
                contact.contact_id,
                format!("Squared: {}", num.wrapping_mul(num)),
            )
            .await?;
        } else {
            reply(
                &client,
                contact.contact_id,
                "Me understands only numbers!".to_owned(),
            )
            .await?;
        }
    }

    Ok(StreamEvents::Continue)
}

// The client API is quite low level so helper functions are often required to deal with common bot
// actions.
async fn reply(
    client: &ws::Client,
    dest: i64,
    reply: impl Into<String>,
) -> ClientResult<Arc<NewChatItemsResponse>> {
    // Use bon builders to build complicated requests. Availaible behind the "bon" feature
    // flag.
    client
        .api_send_messages(
            ApiSendMessages::builder()
                .send_ref(
                    ChatRef::builder()
                        .chat_type(ChatType::Direct)
                        .chat_id(dest)
                        .build(),
                )
                .live_message(false)
                .composed_messages(vec![
                    ComposedMessage::builder()
                        .msg_content(MsgContent::text(reply.into()))
                        .mentions(Default::default())
                        .build(),
                ])
                .build(),
        )
        .await
}
