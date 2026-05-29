//! The example expects that SimpleX CLI is running on port `5225`. Run CLI with:
//! `simplex-chat -p 5225 -d test_db/bot --create-bot-display-name Default`
//!
//! To compile this example pass the --features flag like this:
//! `cargo run --example support_bots --no-default-features --features websocket,farm,cancellation`
//!
//! ----
//!
//! An simple example showing how to utilize bot farms to manage multibot instances

use simploxide_api_types::MsgChatLink;
use simploxide_client::{
    prelude::*,
    ws::{self, EventStream, FarmBot},
};
use tokio_util::sync::CancellationToken;

use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut farm = ws::BotFarmBuilder::new("SimplOxide Examples", 5225)
        .connect()
        .await?;

    let junior_support = farm
        .prepare_bot(
            BotSettings::new("Carl")
                .auto_accept_with("Welcome to the ACME customer support")
                .with_avatar(ImagePreview::from_file("examples/img/carl.jpg")),
        )
        .await?;

    let support_specialist = farm
        .prepare_bot(
            BotSettings::new("Maggie the Specialist")
                .auto_accept()
                .with_avatar(ImagePreview::from_file("examples/img/maggie.jpg")),
        )
        .await?;

    let manager = farm
        .prepare_bot(
            BotSettings::new("Support manager")
                .auto_accept()
                .with_avatar(ImagePreview::from_file("examples/img/mngr.jpg")),
        )
        .await?;

    let (farm, unhandled_events) = farm.run();
    // Not interested in unhandled events for this scenario but they're useful if you want to
    // manage virtual/ghost bots(e.g. bots without their own event streams)
    unhandled_events.discard().await;

    let (junior_support, junior_events) = farm.take_bot(junior_support);
    let (support_specialist, specialist_events) = farm.take_bot(support_specialist);
    let (manager, manager_events) = farm.take_bot(manager);

    let (jr_addr, spec_addr, mngr_addr) = futures::future::try_join3(
        junior_support.address(),
        support_specialist.address(),
        manager.address(),
    )
    .await?;

    let (jr_profile, spec_profile, mngr_profile) = futures::future::try_join3(
        junior_support.profile(),
        support_specialist.profile(),
        manager.profile(),
    )
    .await?;

    println!("You can find the customer support service at {jr_addr}");

    let cancellation = CancellationToken::new();
    let token = cancellation.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        token.cancel();
    });

    let junior_task = tokio::spawn(Box::pin(run_task(Task {
        bot: junior_support,
        events: junior_events,
        next: Arc::new(Next::new(spec_addr, spec_profile)),
        cancellation: cancellation.clone(),
        greetings: String::from("Welcome to the ACME customer support! How can I help you?"),
        problem_reply: String::from(
            "Hmm... This problem is out of my experience. I will find you a specialist in a momemnt",
        ),
        redirect_reply: String::from("Please, Connect with Maggie to resolve your problem"),
    })));

    let spec_task = tokio::spawn(Box::pin(run_task(Task {
        bot: support_specialist,
        events: specialist_events,
        next: Arc::new(Next::new(mngr_addr, mngr_profile)),
        cancellation: cancellation.clone(),
        greetings: String::from(
            "Greetings! Please, describe me your issue and I will see what I can do",
        ),
        problem_reply: String::from(
            "This is very interesting... You better consult with our manager, give me a minute...",
        ),
        redirect_reply: String::from("Here! They will help you"),
    })));

    let mngr_task = tokio::spawn(Box::pin(run_task(Task {
        bot: manager,
        events: manager_events,
        next: Arc::new(Next::new(jr_addr, jr_profile)),
        cancellation: cancellation.clone(),
        greetings: String::from("Hey, what's up, bud?"),
        problem_reply: String::from("Ah, that one. I know the right guy who can help you"),
        redirect_reply: String::from("I bet this guy will have no problems resolving your issue"),
    })));

    junior_task.await?;
    spec_task.await?;
    mngr_task.await?;

    Ok(())
}

async fn run_task(task: Task) {
    #[derive(Clone)]
    struct Ctx {
        bot: FarmBot,
        next: Arc<Next>,
        greetings: String,
        problem_reply: String,
        redirect_reply: String,
    }

    let ctx = Ctx {
        bot: task.bot,
        next: task.next,
        greetings: task.greetings,
        problem_reply: task.problem_reply,
        redirect_reply: task.redirect_reply,
    };

    if let Err(e) = task
        .events
        .into_dispatcher(ctx)
        .on(
            async |ev: Arc<ContactConnected>, ctx| -> ws::ClientResult<StreamEvents> {
                ctx.bot.send_msg(&ev.contact, ctx.greetings).await?;
                Ok(StreamEvents::Continue)
            },
        )
        .on(async |ev: Arc<NewChatItems>, ctx| {
            for (chat, msg, _content) in ev.chat_items.filter_messages() {
                ctx.bot
                    .send_msg(chat, ctx.problem_reply.clone())
                    .reply_to(msg)
                    .await?;

                tokio::time::sleep(std::time::Duration::from_secs(6)).await;

                ctx.bot
                    .send_msg(chat, ctx.redirect_reply.clone())
                    .link_chat(Chat::new(ctx.next.chat_link()))
                    .reply_to(msg)
                    .await?;
            }

            Ok(StreamEvents::Continue)
        })
        .dispatch_with_cancellation(task.cancellation)
        .await
    {
        eprintln!("Oops: {e}");
    }
}

struct Task {
    bot: FarmBot,
    events: EventStream,
    next: Arc<Next>,
    cancellation: CancellationToken,
    greetings: String,
    problem_reply: String,
    redirect_reply: String,
}

struct Next {
    link: String,
    profile: Profile,
}

impl Next {
    fn new(link: String, profile: Profile) -> Self {
        Self { link, profile }
    }

    fn chat_link(&self) -> MsgChatLink {
        MsgChatLink::Contact {
            conn_link: self.link.clone(),
            business: false,
            profile: self.profile.clone(),
            undocumented: Default::default(),
        }
    }
}
