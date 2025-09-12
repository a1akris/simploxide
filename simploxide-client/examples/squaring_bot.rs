//! The example expects that the bot account was already pre-created via CLI by `create bot
//! <bot_name> <bot_fullname>` command.
//!
//! To compile this example pass the --all-features flag like this:
//! `cargo run --example squaring_bot --all-features`

use futures::TryStreamExt as _;
use simploxide_client::{
    CoreError,
    prelude::*,
    types::{CIContent, ChatInfo, ChatType, ComposedMessage, MsgContent},
};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (client, mut events) = simploxide_client::connect("ws://127.0.0.1:5225").await?;

    // Use destructuring to extract data from the expected response
    let ShowActiveUserResponse::ActiveUser(ActiveUserResponse { ref user, .. }) =
        *client.show_active_user().await?
    else {
        return Err("No active user profile".into());
    };

    println!(
        "Bot profile: {} ({})",
        user.profile.display_name, user.profile.full_name
    );

    // Alternatively, use response getters
    let (address_long, address_short) = match client
        .api_show_my_address(user.user_id)
        .await?
        .user_contact_link()
    {
        Some(resp) => (
            resp.contact_link.conn_link_contact.conn_full_link.clone(),
            resp.contact_link.conn_link_contact.conn_short_link.clone(),
        ),
        None => {
            if let Some(resp) = client
                .api_create_my_address(user.user_id)
                .await?
                .user_contact_link_created()
            {
                (
                    resp.conn_link_contact.conn_full_link.clone(),
                    resp.conn_link_contact.conn_short_link.clone(),
                )
            } else {
                return Err("Failed to create bot address".into());
            }
        }
    };

    println!("Bot long address: {address_long}");
    println!("Bot short address: {address_short:?}");

    // The client API is low level, so defining helper functions is often required to deal with
    // common bot actions.
    let send_reply =
        async |dest: i64, reply: String| -> Result<Arc<ApiSendMessagesResponse>, CoreError> {
            // Use bon builders to deal with complicated request structures. Availaible behind
            // the "bon" feature flag.
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
                                .msg_content(MsgContent::Text {
                                    text: reply,
                                    undocumented: Default::default(),
                                })
                                .mentions(Default::default())
                                .build(),
                        ])
                        .build(),
                )
                .await
        };

    // Implement reactor
    'outer: while let Some(ev) = events.try_next().await? {
        match ev {
            // A new user connected
            Event::ContactConnected(connected) => {
                println!("{} connected", connected.contact.profile.display_name);

                send_reply(
                    connected.contact.contact_id,
                    "Hello! I am a simple squaring bot - if you send me a number, I will calculate its square".to_owned()
                ).await?;
            }
            // Got the message -> square the number
            Event::NewChatItems(new_msgs) => {
                for (contact, text) in new_msgs.chat_items.iter().filter_map(|msg| {
                    // Figuring out where the data you're interested in is actually located may
                    // take hours))
                    if let ChatInfo::Direct { ref contact, .. } = msg.chat_info {
                        let text = if let CIContent::RcvMsgContent {
                            msg_content: MsgContent::Text { ref text, .. },
                            ..
                        } = msg.chat_item.content
                        {
                            text.as_str()
                        } else {
                            ""
                        };
                        Some((contact, text))
                    } else {
                        None
                    }
                }) {
                    if text.trim().starts_with("/die") {
                        println!("DIED");
                        break 'outer;
                    }

                    if let Ok(num) = text.trim().parse::<i64>() {
                        send_reply(
                            contact.contact_id,
                            format!("Squared: {}", num.wrapping_mul(num)),
                        )
                        .await?;
                    } else {
                        send_reply(
                            contact.contact_id,
                            "Me understands only numbers!".to_owned(),
                        )
                        .await?;
                    }
                }
            }
            // Accept a request from a new user
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
