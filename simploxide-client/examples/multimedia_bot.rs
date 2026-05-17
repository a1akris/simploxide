//! The examples expects that SimpleX-CLI(`simplex-chat` binary) is installed locally on the system
//! and is available via `$PATH` or at the `simploxide/simploxide-client` directory(you can symlink
//! it there)
//!
//! To compile this example pass the --features flag like this:
//! `cargo run --example multimedia_bot --features fullcli`
//!
//! ----
//!
//! A bot applying a negative filter on incoming images. This example tries to showcase and
//! document all advanced simploxide features

use futures::TryStreamExt as _;
use simploxide_client::{
    StreamEvents,
    crypto::TokioEncryptedFile,
    preferences,
    prelude::*,
    preview::{ImagePreview, Transcoder},
    types::CIFile,
    ws::{self, Bot},
};
use std::{error::Error, io::Cursor, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt},
    sync::Semaphore,
};
use tokio_util::sync::CancellationToken;

const MAX_CONCURRENT_PROCESSORS: usize = 5;
const MAX_FILE_SIZE: usize = 32 * 1024 * 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (bot, events, mut cli) = ws::BotBuilder::new("SimplOxide Examples", 5225)
        .db_prefix("test_db/bot")
        // create a public bot address auto-accepting new users with a welcome message
        .auto_accept_with("Send me an image in any format and I will apply a negative filter to it")
        // Setting a bot profile picture from a file. Since the image is already preprocessed to
        // fit into SimpleX-Chat request limits(max 16KB for the whole request) the image
        // transoding is disabled. With the multimedia feature on `simploxide` automatically
        // transcodes image previews into jpgs satisfying SimpleX-Chat preview requirements
        .with_avatar(
            ImagePreview::from_file("./examples/multimedia_bot_avatar.jpg")
                .with_transcoder(Transcoder::disabled()),
        )
        .with_preferences(Preferences {
            timed_messages: preferences::timed_messages::yes(Duration::from_secs(3600)),
            full_delete: preferences::YES,
            reactions: preferences::NO,
            voice: preferences::NO,
            files: preferences::YES,
            calls: preferences::NO,
            sessions: preferences::NO,
            commands: None,
            undocumented: Default::default(),
        })
        // Launch CLI, connect the client, and initialise the bot
        .launch()
        .await?;

    let address = bot.address().await?;
    println!("Bot address: {address}");

    let cancellation = CancellationToken::new();
    let token = cancellation.clone();

    // Intercept Ctrl-C to run graceful shutdown
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        token.cancel();
    });

    let (mut events, ctx, mut buffered_events) = events
        .into_dispatcher(Ctx::new(bot, MAX_CONCURRENT_PROCESSORS))
        .on(new_msgs)
        .on(file_cancelled)
        .on(send_error)
        // Gracefully process the Ctrl-C event executing all in-flight handlers to completion
        // before exiting
        .dispatch_with_cancellation(cancellation)
        .await?;

    // ---- Graceful shutdown sequence ----
    // Dispatchers guarantee that all handlers are executed to completion before return so all is
    // left is to save the buffered events somehwere for future processing.

    // Shutdown the internal backend
    ctx.bot.shutdown().await;

    // Process left-over buffered events
    loop {
        match events.try_next().await {
            Ok(Some(ev)) => {
                // Real impl should dump the events from the buf somewhere to process them on the next launch
                buffered_events.push(ev);
            }
            Ok(None) => break,
            Err(e) => {
                eprintln!("Event stream error: {:#}", e);
            }
        }
    }

    if !buffered_events.is_empty() {
        println!("Unandled events: ");
        for ev in buffered_events {
            print!("{:?}, ", ev.kind())
        }
    }

    // Always try to call cli.kill().await? explicitly to ensure that CLI process is properly
    // reaped by the system. The Drop impl does it best to kill the process but it doesn't
    // guarantee success
    cli.kill().await?;

    Ok(())
}

async fn new_msgs(ev: Arc<NewChatItems>, ctx: Ctx) -> ws::ClientResult<StreamEvents> {
    for (chat, msg, content) in ev.chat_items.filter_messages() {
        if msg.meta.item_text == "/die" {
            // Break drains all in-flight handlers to completion before the dispatcher returns,
            // so any ongoing downloads/uploads finish cleanly.
            return Ok(StreamEvents::Break);
        }

        if content.text().is_some() {
            ctx.bot
                .send_msg(chat, "Please, send me an image to process")
                .await?;

            continue;
        }

        if content.image().is_none() {
            ctx.bot
                .send_msg(
                    chat,
                    "I can process only images. Send me a file as an image",
                )
                .reply_to(msg)
                .await?;

            continue;
        }

        let file = msg.file.as_ref().unwrap();

        if file.file_size > MAX_FILE_SIZE as i64 {
            ctx.bot
                .send_msg(
                    chat,
                    "The file is too large(max supported size is 32MB)".yellow(),
                )
                .reply_to(msg)
                .await?;

            continue;
        }

        // Bound how many images are processed at once across all concurrent handler invocations.
        let _permit = ctx
            .semaphore
            .acquire()
            .await
            .expect("semaphore is never closed");

        if let Err(e) = process_image(chat, &ctx.bot, file)
            .await
            .map_err(|e| e.to_string())
        {
            ctx.bot
                .send_msg(
                    chat,
                    format!("{}\n\n{}", "Failure processing an image".bold(), e.red()),
                )
                .reply_to(msg)
                .await?;
        }
    }

    Ok(StreamEvents::Continue)
}

async fn file_cancelled(ev: Arc<RcvFileSndCancelled>, ctx: Ctx) -> ws::ClientResult<StreamEvents> {
    if let Some(chat) = ChatId::from_chat_info(&ev.chat_item.chat_info) {
        ctx.bot
            .send_msg(
                chat,
                // or `"Cannot process a file because it was cancelled".yellow()`
                Text::Yellow("Cannot process a file because it was cancelled"),
            )
            .reply_to(&ev.chat_item)
            .await?;
    }

    Ok(StreamEvents::Continue)
}

async fn send_error(ev: Arc<SndFileError>, ctx: Ctx) -> ws::ClientResult<StreamEvents> {
    eprintln!("Failed to send a file: {}", ev.error_message);

    if let Some(chat) = ev
        .chat_item
        .as_ref()
        .and_then(|item| ChatId::from_chat_info(&item.chat_info))
    {
        ctx.bot
            .send_msg(
                chat,
                format!(
                    "{}\n\n{}",
                    "Failure sending a file".bold(),
                    ev.error_message.red()
                ),
            )
            .await?;
    }

    Ok(StreamEvents::Continue)
}

#[derive(Clone)]
struct Ctx {
    bot: Bot,
    semaphore: Arc<Semaphore>,
}

impl Ctx {
    fn new(bot: Bot, max_file_processors: usize) -> Self {
        Self {
            bot,
            semaphore: Arc::new(Semaphore::new(max_file_processors)),
        }
    }
}

async fn process_image(
    chat: ChatId,
    bot: &Bot,
    file: &CIFile,
) -> Result<(), Box<dyn std::error::Error>> {
    let received = bot.download_file(file).store_encrypted().await?;
    let file_source = received
        .chat_item
        .chat_item
        .file
        .as_ref()
        .and_then(|x| x.file_source.clone())
        .unwrap();

    // Processing the encrypted image in memory

    let path = file_source.file_path;
    let mut file =
        TokioEncryptedFile::open(&path, file_source.crypto_args.unwrap().try_into()?).await?;

    let mut buf = Vec::with_capacity(file.plaintext_size_hint());
    file.read_to_end(&mut buf).await?;

    // Image decoding and encoding are CPU-bound; offload to the blocking thread pool so the
    // async runtime is not stalled.
    let transcoded = tokio::task::spawn_blocking(move || -> image::ImageResult<Vec<u8>> {
        let mut img = image::ImageReader::new(Cursor::new(&buf))
            .with_guessed_format()?
            .decode()?;

        img.invert();

        buf.clear();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 95);
        img.write_with_encoder(encoder)?;

        Ok(buf)
    })
    .await??;

    // Re-encrypt the processed image in place. prepare_for_overwrite re-keys the file so the
    // new content gets a fresh key/nonce; crypto_args must be re-read afterwards because they
    // changed.
    file.prepare_for_overwrite().await?;
    let crypto_args = file.crypto_args().expose();

    file.write_all(&transcoded).await?;
    // Finalise the AEAD tag after all plaintext bytes are written. The callers must always do it
    // explicitly because the Drop impl works on the best effort basis potentially leaving the file
    // unauthenticated
    file.put_auth_tag().await?;

    bot.send_msg(chat, Image::new(path).with_crypto_args(crypto_args))
        // modifying the default image preview transcoder. The default transcoded produces pixelated
        // previews similar to SimpleX-Chat clients, this transcoder creates blurred previews instead
        .with_transcoder(Transcoder::default().with_blur(1.5).with_quality(80))
        .await?;

    Ok(())
}
