use serde::Deserialize;
use simploxide_api_types::{
    NewUser, User,
    client_api::ClientApi,
    commands::{ApiDeleteUser, ApiSetActiveUser, CancelFile, ListUsers, ReceiveFile},
    responses::{CancelFileResponse, ListUsersResponse, ReceiveFileResponse, UsersListResponse},
};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    oneshot,
};

use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
};

use crate::{EventParser, EventStream, bot::BotSettings, ext::ClientApiExt as _, id::UserId};

mod demux;
mod mux;

use demux::{BotMap, Channel};

use super::Bot;

#[cfg(feature = "xftp")]
pub type FarmBot<C> = Bot<crate::xftp::XftpClient<DelegateClient<C>>>;

#[cfg(not(feature = "xftp"))]
pub type FarmBot<C> = Bot<DelegateClient<C>>;

#[derive(Clone)]
pub struct BotFarm<S> {
    state: S,
}

impl<C: ClientApi, P: EventParser> BotFarm<Init<C, P>> {
    pub async fn init(
        farm_name: String,
        client: C,
        events: EventStream<P>,
    ) -> Result<Self, C::Error> {
        let mut farm_id = BotId::anybot();
        let mut active_name = String::new();
        let mut cache = HashMap::new();
        let bots = demux::FxDashMap::with_hasher(rustc_hash::FxBuildHasher);

        let resp = client.list_users().await?;

        for info in &resp.users {
            let bot_id = BotId(info.user.user_id);

            if info.user.active_user {
                active_name = info.user.profile.display_name.clone();
            }

            if info.user.profile.display_name == farm_name {
                farm_id = bot_id;
                continue;
            }

            bots.insert(bot_id, Channel::Ghost);
            cache.insert(info.user.profile.display_name.clone(), info.user.clone());
        }

        if farm_id.is_anybot() {
            let resp = client
                .create_active_user(NewUser {
                    profile: Some(Bot::<C>::default_profile(farm_name.clone())),
                    past_timestamp: false,
                    user_chat_relay: false,
                    undocumented: Default::default(),
                })
                .await?;

            farm_id = BotId(resp.user.user_id);
            active_name = farm_name.clone();
        }

        let state = Init {
            client,
            events,
            farm_id,
            farm_name,
            active_name,
            bots,
            cache,
        };

        Ok(Self { state })
    }

    pub fn users_count(&self) -> usize {
        self.state.cache.len()
    }

    pub fn users(&self) -> impl Iterator<Item = &User> {
        self.state.cache.values()
    }

    pub fn user(&self, name: &str) -> Option<&User> {
        self.state.cache.get(name)
    }

    pub async fn remove(&mut self, user_id: UserId) -> Result<(), C::Error> {
        self.state
            .client
            .api_set_active_user(ApiSetActiveUser::new(self.state.farm_id.0))
            .await?;

        self.state.active_name = self.state.farm_name.clone();

        let resp = self
            .state
            .client
            .api_delete_user(ApiDeleteUser {
                user_id: user_id.0,
                del_smp_queues: true,
                view_pwd: None,
            })
            .await?;

        let user = resp.user.as_ref().unwrap();

        self.state.bots.remove(&user_id.into());
        self.state.cache.remove(&user.profile.display_name);

        Ok(())
    }

    pub async fn remove_by_name(&mut self, name: &str) -> Result<(), C::Error> {
        let Some(user) = self.state.cache.remove(name) else {
            return Ok(());
        };

        let result = self.remove(UserId(user.user_id)).await;

        if result.is_err() {
            self.state.cache.insert(name.to_owned(), user);
        }

        result
    }

    pub async fn prepare_bot(
        &mut self,
        settings: BotSettings,
    ) -> Result<UserId, CreateError<C::Error>>
    where
        C: Clone,
    {
        let user_id = self.prepare_inner(settings).await?;
        self.state.bots.insert(user_id.into(), Channel::new_bot());

        Ok(user_id)
    }

    pub async fn prepare_ghost(
        &mut self,
        settings: BotSettings,
    ) -> Result<UserId, CreateError<C::Error>>
    where
        C: Clone,
    {
        let user_id = self.prepare_inner(settings).await?;
        self.state.bots.insert(user_id.into(), Channel::Ghost);

        Ok(user_id)
    }

    pub fn run(self) -> (BotFarm<Running<C, P>>, EventStream<P>)
    where
        C: 'static + Send,
        C::Error: Send,
        P: 'static + Send,
    {
        let (delegate_client, rx) = DelegateClient::new(self.state.farm_id);
        mux::start(self.state.client, rx);

        let bots = Arc::new(self.state.bots);
        let (suspender, unmuxed_events) = demux::start(bots.clone(), self.state.events);

        #[cfg(feature = "xftp")]
        let (xftp_client, unmuxed_events) = unmuxed_events.hook_xftp(delegate_client.clone());

        let state = Running {
            farm_name: self.state.farm_name,
            client: delegate_client,
            suspender,
            bots,
            #[cfg(feature = "xftp")]
            xftp: xftp_client.manager(),
        };

        (BotFarm { state }, unmuxed_events)
    }

    async fn prepare_inner(
        &mut self,
        settings: BotSettings,
    ) -> Result<UserId, CreateError<C::Error>>
    where
        C: Clone,
    {
        if settings.display_name == self.state.farm_name {
            return Err(CreateError::FarmUser);
        }

        match self.state.cache.entry(settings.display_name.clone()) {
            Entry::Occupied(mut occupied) => {
                let bot = Bot::<C>::init_existing(
                    self.state.client.clone(),
                    occupied.get_mut(),
                    settings,
                )
                .await?;
                let update = bot.info().await?;

                *occupied.get_mut() = update.user.clone();
                self.change_active_user(update.user.profile.display_name.clone());

                Ok(bot.user_id())
            }
            Entry::Vacant(vacant) => {
                let bot = Bot::<C>::init_new(self.state.client.clone(), settings).await?;
                let update = bot.info().await?;

                vacant.insert(update.user.clone());

                self.change_active_user(update.user.profile.display_name.clone());
                Ok(bot.user_id())
            }
        }
    }

    fn change_active_user(&mut self, new_active_username: String) {
        if new_active_username == self.state.active_name {
            return;
        }

        if let Some(user) = self.state.cache.get_mut(&self.state.active_name) {
            user.active_user = false;
        }

        self.state.active_name = new_active_username;
    }
}

impl<C: 'static + ClientApi, P: EventParser> BotFarm<Running<C, P>>
where
    C::Error: Send,
{
    pub fn ghost(&self, user_id: UserId) -> Option<FarmBot<C>> {
        let chan = self.state.bots.get(&user_id.into())?;

        if let Channel::Ghost = chan.value() {
            Some(self.make_ghost(user_id))
        } else {
            None
        }
    }

    /// Panics if user_id doesn't exist, the user was never initialized as a bot, or the bot was already taken
    pub fn take_bot(&self, user_id: UserId) -> (FarmBot<C>, EventStream<P>) {
        let mut chan = self.state.bots.get_mut(&user_id.into()).unwrap();

        if chan.is_ghost() {
            panic!("The {user_id:?} was not initialized as bot");
        }

        let receiver = chan
            .take_receiver()
            .unwrap_or_else(|| panic!("The {user_id:?} was already taken"));

        self.make_bot(user_id, receiver)
    }

    pub fn take_bot_checked(&self, user_id: UserId) -> Option<(FarmBot<C>, EventStream<P>)> {
        self.state
            .bots
            .get_mut(&user_id.into())
            .and_then(|mut chan| chan.take_receiver())
            .map(|receiver| self.make_bot(user_id, receiver))
    }

    pub async fn create_bot(
        &self,
        settings: BotSettings,
    ) -> Result<(FarmBot<C>, EventStream<P>), CreateError<C::Error>> {
        let user_id = self.create_inner(settings, true).await?;
        let (bot, stream) = self.take_bot(user_id);
        Ok((bot, stream))
    }

    pub async fn get_or_create_bot(
        &self,
        settings: BotSettings,
    ) -> Result<(FarmBot<C>, EventStream<P>), CreateError<C::Error>> {
        let resp = self.state.client.list_users().await?;

        match resp.users.iter().find_map(|info| {
            (info.user.profile.display_name == settings.display_name)
                .then_some(BotId(info.user.user_id))
        }) {
            Some(user_id) => match self.state.bots.get_mut(&user_id) {
                Some(mut entry) => match entry.value_mut() {
                    Channel::Bot(pipe) => {
                        let receiver = pipe.take_receiver().ok_or(CreateError::BotAlreadyTaken)?;
                        Ok(self.make_bot(user_id.into(), receiver))
                    }
                    Channel::Ghost => Err(CreateError::BotIsGhost),
                },
                None => Err(CreateError::Desync),
            },
            None => self.create_bot(settings).await,
        }
    }

    pub async fn create_ghost(
        &self,
        settings: BotSettings,
    ) -> Result<FarmBot<C>, CreateError<C::Error>> {
        let user_id = self.create_inner(settings, false).await?;
        Ok(self.ghost(user_id).unwrap())
    }

    pub async fn get_or_create_ghost(
        &self,
        settings: BotSettings,
    ) -> Result<FarmBot<C>, CreateError<C::Error>> {
        let resp = self.state.client.list_users().await?;

        match resp.users.iter().find_map(|info| {
            (info.user.profile.display_name == settings.display_name)
                .then_some(BotId(info.user.user_id))
        }) {
            Some(user_id) => match self.state.bots.get(&user_id) {
                Some(entry) => match entry.value() {
                    Channel::Bot(_) => Err(CreateError::GhostIsBot),
                    Channel::Ghost => Ok(self.make_ghost(user_id.into())),
                },
                None => Err(CreateError::Desync),
            },
            None => self.create_ghost(settings).await,
        }
    }

    pub async fn delete(&self, user_id: UserId) -> Result<(), C::Error> {
        self.state
            .client
            .api_delete_user(ApiDeleteUser {
                user_id: user_id.0,
                del_smp_queues: true,
                view_pwd: None,
            })
            .await?;

        self.state.bots.remove(&user_id.into());
        Ok(())
    }

    async fn create_inner(
        &self,
        settings: BotSettings,
        is_bot: bool,
    ) -> Result<UserId, CreateError<C::Error>> {
        if settings.display_name == self.state.farm_name {
            return Err(CreateError::FarmUser);
        }

        let (_guard, suspension) = oneshot::channel();
        let _ = self.state.suspender.send(suspension);

        let mut resp = self
            .state
            .client
            .new_user(NewUser {
                profile: Some(Bot::<C>::default_profile(&settings.display_name)),
                past_timestamp: false,
                user_chat_relay: false,
                undocumented: Default::default(),
            })
            .await?;

        if is_bot {
            self.state
                .bots
                .insert(BotId(resp.user.user_id), Channel::new_bot());
        } else {
            self.state
                .bots
                .insert(BotId(resp.user.user_id), Channel::Ghost);
        }

        let resp = Arc::get_mut(&mut resp).unwrap();
        let client = self.state.client.delegate_to(BotId(resp.user.user_id));

        match Bot::init_existing(client, &mut resp.user, settings).await {
            Ok(bot) => Ok(bot.user_id()),
            Err(e) => {
                self.state
                    .bots
                    .insert(BotId(resp.user.user_id), Channel::Ghost);

                Err(e.into())
            }
        }
    }

    fn make_bot(
        &self,
        user_id: UserId,
        receiver: UnboundedReceiver<P>,
    ) -> (FarmBot<C>, EventStream<P>) {
        let bot_client = self.state.client.delegate_to(user_id);
        let stream = EventStream::from(receiver);

        #[cfg(feature = "xftp")]
        let (bot_client, stream) = stream.hook_xftp(bot_client);

        (Bot::new(bot_client, user_id), stream)
    }

    fn make_ghost(&self, user_id: UserId) -> FarmBot<C> {
        let bot_client = self.state.client.delegate_to(user_id);

        #[cfg(feature = "xftp")]
        let bot_client = crate::xftp::XftpClient::new(bot_client, self.state.xftp.clone());

        Bot::new(bot_client, user_id)
    }
}

pub struct Init<C, P> {
    client: C,
    events: EventStream<P>,
    farm_id: BotId,
    farm_name: String,
    active_name: String,
    bots: BotMap<P>,
    cache: HashMap<String, User>,
}

#[derive(Clone)]
pub struct Running<C: ClientApi, P> {
    farm_name: String,
    client: DelegateClient<C>,
    suspender: demux::Suspender,
    bots: Arc<BotMap<P>>,
    #[cfg(feature = "xftp")]
    xftp: Arc<crate::xftp::XftpManager>,
}

pub struct DelegateClient<C: ClientApi> {
    bot_id: BotId,
    sender: DelegateSender<C>,
}

impl<C: ClientApi> Clone for DelegateClient<C> {
    fn clone(&self) -> Self {
        Self {
            bot_id: self.bot_id,
            sender: self.sender.clone(),
        }
    }
}

impl<C: ClientApi> DelegateClient<C> {
    fn new(bot_id: BotId) -> (Self, DelegateReceiver<C>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        (Self { bot_id, sender }, receiver)
    }

    fn delegate_to(&self, bot_id: impl Into<BotId>) -> Self {
        Self {
            bot_id: bot_id.into(),
            sender: self.sender.clone(),
        }
    }
}

impl<C: ClientApi> ClientApi for DelegateClient<C>
where
    C::Error: Send,
{
    type ResponseShape<'de, T: 'de + Deserialize<'de>> = C::ResponseShape<'de, T>;
    type Error = C::Error;

    async fn send_raw(&self, cmd: String) -> Result<String, Self::Error> {
        let (responder, response) = oneshot::channel();

        let request = DelegateRequest {
            bot_id: self.bot_id,
            cmd,
            responder,
        };

        self.sender
            .send(request)
            .expect("Delegate client cannot outlive background task");

        response
            .await
            .expect("Delegate client cannot outlive background task")
    }

    async fn list_users(&self) -> Result<Arc<UsersListResponse>, Self::Error> {
        let client = self.delegate_to(BotId::anybot());
        let response: ListUsersResponse = client.send(ListUsers {}).await?;
        Ok(response.into_inner())
    }

    async fn receive_file(&self, cmd: ReceiveFile) -> Result<ReceiveFileResponse, Self::Error> {
        let client = self.delegate_to(BotId::anybot());
        client.send(cmd).await
    }

    async fn cancel_file(&self, file_id: i64) -> Result<CancelFileResponse, Self::Error> {
        let client = self.delegate_to(BotId::anybot());
        client.send(CancelFile { file_id }).await
    }
}

#[derive(Debug)]
pub enum CreateError<E> {
    /// Farm user cannot be interacted with directly
    FarmUser,
    /// Bot cannot be created because ghost with same name already exists
    BotIsGhost,
    /// Ghost cannot be craeted because bot with same name already exists
    GhostIsBot,
    /// The bot already exists and was already taken with the `take_bot`
    BotAlreadyTaken,
    /// The in memory state is not synced with the DB, retry later
    Desync,
    Api(E),
}

impl<E> From<E> for CreateError<E> {
    fn from(value: E) -> Self {
        Self::Api(value)
    }
}

impl<E> std::fmt::Display for CreateError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FarmUser => write!(
                f,
                "Attempt to create a farm user. Farm user is special and cannot be interacted with directly"
            ),
            Self::BotIsGhost => write!(
                f,
                "Cannot create a bot because the ghost user with the same name already exists"
            ),
            Self::GhostIsBot => write!(
                f,
                "Cannot create a ghost because the bot user with the same name already exists"
            ),
            Self::BotAlreadyTaken => {
                write!(
                    f,
                    "The bot already exists and has been taken from the farm. Cannot recreate operational bots"
                )
            }
            Self::Desync => {
                write!(
                    f,
                    "The DB state was not in sync with the memory state, try again"
                )
            }
            Self::Api(e) => write!(f, "{e:#}"),
        }
    }
}

impl<E: 'static + std::error::Error> std::error::Error for CreateError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Api(err) = self {
            Some(err)
        } else {
            None
        }
    }
}

struct DelegateRequest<C: ClientApi> {
    bot_id: BotId,
    cmd: String,
    responder: oneshot::Sender<Result<String, C::Error>>,
}

type DelegateSender<C> = UnboundedSender<DelegateRequest<C>>;
type DelegateReceiver<C> = UnboundedReceiver<DelegateRequest<C>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct BotId(i64);

impl BotId {
    /// Used as an optimization for commands that can execute from any active bot account
    fn anybot() -> Self {
        Self(0)
    }

    fn is_anybot(self) -> bool {
        self.0 == 0
    }
}

impl From<UserId> for BotId {
    fn from(user_id: UserId) -> Self {
        Self(user_id.0)
    }
}

impl From<BotId> for UserId {
    fn from(bot_id: BotId) -> Self {
        Self(bot_id.0)
    }
}
