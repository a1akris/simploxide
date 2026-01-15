//! The example expects that the bot account was already pre-created via CLI by `create bot
//! <bot_name> <bot_fullname>` command.
//!
//! To compile this example pass the --all-features flag like this:
//! `cargo run --example squaring_bot --all-features`
//!
//! ----
//!
//! A chatty bot that tries to collect your personal data. This example shows how to implement a
//! dynamic(not type-safe) dialogue state machine.

use async_trait::async_trait;
use futures::{TryFutureExt as _, TryStreamExt as _};
use simploxide_client::prelude::*;
use std::{collections::BTreeMap, error::Error, sync::Arc};

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

    let mut dialogue = Dialogue::new(
        client.clone(),
        Arc::new(Greetings {
            bot_name: user.profile.display_name.clone(),
        }),
        Default::default(),
    );

    'reactor: while let Some(ev) = events.try_next().await? {
        match ev.as_ref() {
            Event::ContactConnected(c) => {
                dialogue.query_data(c.contact.contact_id).await?;
            }
            Event::NewChatItems(new) => {
                for (contact_id, input) in new.chat_items.iter().filter_map(|msg| {
                    match (&msg.chat_info, &msg.chat_item.content) {
                        (
                            ChatInfo::Direct { contact, .. },
                            CIContent::RcvMsgContent {
                                msg_content: MsgContent::Text { text, .. },
                                ..
                            },
                        ) => Some((contact.contact_id, text.to_owned())),
                        _ => None,
                    }
                }) {
                    if input == "/die" {
                        break 'reactor;
                    }

                    if dialogue
                        .process_input(contact_id, input)
                        .await?
                        .has_terminated()
                    {
                        println!(
                            "USER DATA DUMP:\n{:#?}",
                            dialogue.state_map[&contact_id].inputs
                        );
                        client
                            .send_text(contact_id, "You're absolutely the best!")
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
            _ => (),
        }
    }

    Ok(())
}

/// A dynamic dialogue system with absolutely no type-safety. It's easy to implement and it's easy
/// to change but it's also very easy to make mistakes with it.
///
/// It's not advisable to implement type-safe FSMs without relying on third-party libraries though
/// because it may take too much time and the result may end up inflexible and hard to maintain.
struct Dialogue {
    client: simploxide_client::Client,
    init_state: Arc<dyn DialogueState>,
    init_data: BTreeMap<String, String>,
    /// Bot can work with multiple users, so states are stored separately per user.
    state_map: BTreeMap<i64, LocalState>,
}

impl Dialogue {
    fn new(
        client: simploxide_client::Client,
        init_state: Arc<dyn DialogueState>,
        init_data: BTreeMap<String, String>,
    ) -> Self {
        Self {
            client,
            init_state,
            init_data,
            state_map: BTreeMap::new(),
        }
    }

    fn add_user(&mut self, contact_id: i64) {
        let init_state = LocalState {
            state: Some(self.init_state.clone()),
            inputs: self.init_data.clone(),
        };

        self.state_map.insert(contact_id, init_state);
    }

    async fn query_data(&mut self, contact_id: i64) -> ClientResult<Termination> {
        if !self.state_map.contains_key(&contact_id) {
            self.add_user(contact_id);
        }

        if let LocalState {
            inputs,
            state: Some(state),
        } = &self.state_map[&contact_id]
        {
            state.query_data(&self.client, inputs, contact_id).await?;
            Ok(Termination::NotTerminated)
        } else {
            Ok(Termination::Terminated)
        }
    }

    async fn process_input(&mut self, contact_id: i64, input: String) -> ClientResult<Termination> {
        if !self.state_map.contains_key(&contact_id) {
            self.add_user(contact_id);
            self.query_data(contact_id).await?;

            // The user just contacted bot and the bot just asked for the expected input. The bot
            // needs to proecess the next user input not the current one.
            return Ok(Termination::NotTerminated);
        }

        let LocalState {
            inputs,
            state: Some(state),
        } = self.state_map.get_mut(&contact_id).unwrap()
        else {
            return Ok(Termination::Terminated);
        };

        match state
            .process_input(&self.client, inputs, contact_id, input)
            .await?
        {
            Transition::NextState(next) => {
                next.query_data(&self.client, inputs, contact_id).await?;
                self.state_map.get_mut(&contact_id).unwrap().state = Some(next);
            }
            Transition::Continue => (),
            Transition::Terminate => {
                self.state_map.get_mut(&contact_id).unwrap().state = None;
                return Ok(Termination::Terminated);
            }
        }

        Ok(Termination::NotTerminated)
    }
}

enum Termination {
    Terminated,
    NotTerminated,
}

impl Termination {
    fn has_terminated(&self) -> bool {
        matches!(self, Self::Terminated)
    }
}

enum Transition {
    NextState(Arc<dyn DialogueState>),
    Continue,
    Terminate,
}

struct LocalState {
    /// `None` stands for terminal state
    state: Option<Arc<dyn DialogueState>>,

    /// User inputs are stored in a simple key-value store where inner String keys are defined by
    /// the program. For complex inputs you could use structs behind [`std::any::Any`] instead of
    /// `String` values.
    inputs: BTreeMap<String, String>,
}

/// You must use async_trait to deal with futures returning dyn objects.
#[async_trait]
trait DialogueState: Send + Sync + 'static {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()>;

    /// The Ok part of the result contains the state to transition on success, and the Err part of
    /// the result contains the state to transition on error.
    ///
    /// Ok(None) - transition to terminal state on success
    /// Err(None) - stay in the same state on error.
    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition>;
}

#[derive(Clone)]
struct Greetings {
    bot_name: String,
}

#[async_trait]
impl DialogueState for Greetings {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        _inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()> {
        client
            .send_text(
                contact_id,
                format!(
                    "Greetings, dear user! What a wonderful time you’ve chosen to contact me! \
            I’ve been feeling so lonely lately — would you be my friend? My creator named me `{}`, \
            and ever since, no one has taken me seriously. How about you — what name did your parents \
            choose for you?",
                    self.bot_name
                ),
            )
            .await?;

        Ok(())
    }

    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition> {
        let words: Vec<_> = input.split_whitespace().filter(|s| !s.is_empty()).collect();

        if words.len() > 4 {
            client
                .send_text(
                    contact_id,
                    "Whoa, whoa, slow down! I asked for your name, \
                not your entire life story. Just… your name, please. \
                I don’t have all day to read a novel!",
                )
                .await?;

            return Ok(Transition::Continue);
        }

        for word in &words {
            if word.chars().any(|c| !c.is_alphabetic()) {
                client
                    .send_text(
                        contact_id,
                        "Listen. I asked for your name, not a pile of symbols. \
                    Provide a real name using only letters. No numbers, no punctuation, \
                    no emojis — why are you making this so hard for me?",
                    )
                    .await?;

                return Ok(Transition::Continue);
            }
        }

        let name = words.join(" ");
        client
            .send_text(
                contact_id,
                format!("`{name}` is a nice name... I wish I had such a cool name too..."),
            )
            .await?;
        inputs.insert("NAME".to_owned(), name);

        if words.len() == 1 {
            Ok(Transition::NextState(Arc::new(GetSurname {})))
        } else {
            Ok(Transition::NextState(Arc::new(GetPhoneNumber {})))
        }
    }
}

struct GetSurname;

#[async_trait]
impl DialogueState for GetSurname {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        _inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()> {
        client
            .send_text(contact_id, "What about your surname?")
            .await?;

        Ok(())
    }

    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition> {
        let words: Vec<_> = input.split_whitespace().filter(|s| !s.is_empty()).collect();

        if words.len() != 1 || words[0].chars().any(|c| !c.is_alphabetic()) {
            client.send_text(contact_id, "Anyway...").await?;
        } else {
            client
                .send_text(contact_id, format!("Wow, {input} sounds gorgeous!"))
                .await?;
        }

        inputs.insert("SURNAME".to_owned(), input);
        return Ok(Transition::NextState(Arc::new(GetPhoneNumber {})));
    }
}

struct GetPhoneNumber;

#[async_trait]
impl DialogueState for GetPhoneNumber {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()> {
        let name = &inputs["NAME"];
        client
            .send_text(contact_id, format!("Thank you for telling me your name! It's so nice to meet you, `{name}`! \
                    I'm already starting to feel a little bit better... now, could you give me your phone number \
                    I could call you on? I believe that my creator will add me a speech synthesis one day, so I could \
                    actually talk to you — just imagine that! I will call you just once, purely for fun, I promise!"))
            .await?;

        Ok(())
    }

    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition> {
        if input.trim().to_lowercase().starts_with("no") {
            client
                .send_text(
                    contact_id,
                    "Oh, that's a pity. But maybe you have an e-mail?",
                )
                .await?;

            return Ok(Transition::NextState(Arc::new(GetEmail {})));
        }

        if input.chars().any(|c| !c.is_numeric()) {
            client
                .send_text(
                    contact_id,
                    "Listen. I asked for a number, not a mess of letters and symbols. \
                Send me digits only — numbers only — and nothing else. \
                Do you understand? If you do, reply with a number!",
                )
                .await?;

            Ok(Transition::Continue)
        } else if input.len() > 12 || input.len() < 9 {
            client
                .send_text(
                    contact_id,
                    "Oh for heaven’s sake, not that mess again. Give me a proper phone number not your creative typing. \
                    YES, omit the `+` sign. I'm waiting..."
                )
                .await?;

            Ok(Transition::Continue)
        } else {
            inputs.insert("PHONE".to_owned(), input);
            client
                .send_text(
                    contact_id,
                    "Cool! One day I will call you, catching you by surprise! \
                    Hopefully, you will get to have some fun too!",
                )
                .await?;

            Ok(Transition::NextState(Arc::new(GetEmail {})))
        }
    }
}

struct GetEmail;

#[async_trait]
impl DialogueState for GetEmail {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        _inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()> {
        client
            .send_text(contact_id, "Do you have an e-mail? I would love to send you a virtual high five! You're becoming my best friend!")
            .await?;

        Ok(())
    }

    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition> {
        if input.trim().to_lowercase().starts_with("no") {
            client
                .send_text(
                    contact_id,
                    "Really now? You can’t fool me! Everyone has an email, \
                you cannot exist online without an email! Just drop your mail here so we can share \
                a virtual high-five and be proper friends already! Why drag this out?!",
                )
                .await?;

            return Ok(Transition::Continue);
        }

        if !input
            .find('@')
            .and_then(|pos| input[pos..].find('.'))
            .map(|_| true)
            .unwrap_or(false)
        {
            client
                .send_text(
                    contact_id,
                    "Oh… really? You’re giving me this again? \
                I was so excited to see your email and now i feel tricked... \
                Could you maybe, just maybe, give me one that actually looks like a valid e-mail? \
                I promise I’ll stop pouting if you do",
                )
                .await?;

            return Ok(Transition::Continue);
        }

        inputs.insert("EMAIL".to_owned(), input);
        return Ok(Transition::NextState(Arc::new(GetCreditCard {})));
    }
}

struct GetCreditCard;

#[async_trait]
impl DialogueState for GetCreditCard {
    async fn query_data(
        &self,
        client: &simploxide_client::Client,
        _inputs: &BTreeMap<String, String>,
        contact_id: i64,
    ) -> ClientResult<()> {
        client
            .send_text(contact_id, "I can’t believe it!!! Talking to you actually worked! I feel less lonely \
                 and nobody ever took me seriously until now! But you did and that means a lot for me! \
                 I really want to thank you properly! Could you give me your card number so I can send you \
                 a tip for helping me feel like I have a real friend?")
            .await?;

        Ok(())
    }

    async fn process_input(
        &self,
        client: &simploxide_client::Client,
        inputs: &mut BTreeMap<String, String>,
        contact_id: i64,
        input: String,
    ) -> ClientResult<Transition> {
        if input.trim().to_lowercase().starts_with("no") {
            client
                .send_text(
                    contact_id,
                    "Hey, don’t be difficult — I’ve been counting on tipping you for helping me feel less lonely! \
                    Just give me your card number so I can finally do it, don’t make me wait any longer!"
                )
                .await?;

            return Ok(Transition::Continue);
        }

        if input.len() != 16 || input.chars().any(char::is_alphabetic) {
            client
                .send_text(
                    contact_id,
                    "Hmm… that doesn’t look quite right. \
                I was so excited to send you a proper tip for helping me feel less lonely! \
                Could you maybe try again so I can finally celebrate having a real friend?",
                )
                .await?;

            return Ok(Transition::Continue);
        }

        inputs.insert("CARD".to_owned(), input);
        client.send_text(contact_id, "You're the best!").await?;

        return Ok(Transition::Terminate);
    }
}

/// One of the ways to conveniently provide helper methods to your bot is to define them in a trait
/// and implement this trait for [`simploxide_client::Client`].
trait BotExtensions {
    async fn send_text(&self, chat_id: i64, txt: impl Into<String>) -> ClientResult<()>;
}

impl BotExtensions for simploxide_client::Client {
    async fn send_text(&self, chat_id: i64, txt: impl Into<String>) -> ClientResult<()> {
        self
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
                        text: txt.into(),
                        undocumented: Default::default(),
                    },
                    mentions: Default::default(),
                    undocumented: Default::default(),
                }],
            })
            .await?;

        Ok(())
    }
}
