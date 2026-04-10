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
    prelude::*,
    ws::{self, ClientResult},
};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (bot, events, _cli) = ws::BotBuilder::new("SimplOxide Examples", 5225)
        // Set path to the bot database
        .db_prefix("test_db/bot")
        // create a public bot address auto-accepting new users with a welcome message
        .auto_reply(
            "Hello, I'm a simple squaring bot. Send me a number and I will calculate its square",
        )
        // Launch CLI, connect the client, and initialise the bot
        .launch()
        .await?;

    let address = bot.address().await?;
    println!("Bot address: {address}");

    events.into_dispatcher(bot).on(new_msgs).dispatch().await?;
    Ok(())
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
            let square = num.wrapping_mul(num);
            bot.send_msg(cid, format!("Squared: {square}"))
                .reply_to(it.meta.item_id)
                .await?;
        } else {
            bot.send_msg(cid, "Me understands only numbers!")
                .reply_to(it.meta.item_id)
                .await?;
        }
    }

    Ok(StreamEvents::Continue)
}
