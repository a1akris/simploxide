use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentCryptoError {
    #[serde(rename = "DECRYPT_AES")]
    DecryptAes,
    #[serde(rename = "DECRYPT_CB")]
    DecryptCb,
    #[serde(rename = "RATCHET_HEADER")]
    RatchetHeader,
    #[serde(rename = "RATCHET_SYNC")]
    RatchetSync,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentErrorType {
    #[serde(rename = "CMD")]
    Cmd {
        #[serde(rename = "cmdErr")]
        cmd_err: CommandErrorType,

        #[serde(rename = "errContext")]
        err_context: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "CONN")]
    Conn {
        #[serde(rename = "connErr")]
        conn_err: ConnectionErrorType,

        #[serde(rename = "errContext")]
        err_context: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "NO_USER")]
    NoUser,
    #[serde(rename = "SMP")]
    Smp {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "smpErr")]
        smp_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "NTF")]
    Ntf {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "ntfErr")]
        ntf_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "XFTP")]
    Xftp {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "xftpErr")]
        xftp_err: XFTPErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "FILE")]
    File {
        #[serde(rename = "fileErr")]
        file_err: FileErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "PROXY")]
    Proxy {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "relayServer")]
        relay_server: String,

        #[serde(rename = "proxyErr")]
        proxy_err: ProxyClientError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "RCP")]
    Rcp {
        #[serde(rename = "rcpErr")]
        rcp_err: RCErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "BROKER")]
    Broker {
        #[serde(rename = "brokerAddress")]
        broker_address: String,

        #[serde(rename = "brokerErr")]
        broker_err: BrokerErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "AGENT")]
    Agent {
        #[serde(rename = "agentErr")]
        agent_err: SMPAgentError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "INTERNAL")]
    Internal {
        #[serde(rename = "internalErr")]
        internal_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "CRITICAL")]
    Critical {
        #[serde(rename = "offerRestart")]
        offer_restart: bool,

        #[serde(rename = "criticalErr")]
        critical_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BrokerErrorType {
    #[serde(rename = "RESPONSE")]
    Response {
        #[serde(rename = "respErr")]
        resp_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "UNEXPECTED")]
    Unexpected {
        #[serde(rename = "respErr")]
        resp_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "NETWORK")]
    Network {
        #[serde(rename = "networkError")]
        network_error: NetworkError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "HOST")]
    Host,
    #[serde(rename = "NO_SERVICE")]
    NoService,
    #[serde(rename = "TRANSPORT")]
    Transport {
        #[serde(rename = "transportErr")]
        transport_err: TransportError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatError {
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "errorType")]
        error_type: ChatErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "errorAgent")]
    ErrorAgent {
        #[serde(rename = "agentError")]
        agent_error: AgentErrorType,

        #[serde(rename = "connectionEntity_", skip_serializing_if = "Option::is_none")]
        connection_entity: Option<ConnectionEntity>,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "errorStore")]
    ErrorStore {
        #[serde(rename = "storeError")]
        store_error: StoreError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatErrorType {
    #[serde(rename = "noActiveUser")]
    NoActiveUser,
    #[serde(rename = "noConnectionUser")]
    NoConnectionUser {
        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "noSndFileUser")]
    NoSndFileUser {
        #[serde(rename = "agentSndFileId")]
        agent_snd_file_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "noRcvFileUser")]
    NoRcvFileUser {
        #[serde(rename = "agentRcvFileId")]
        agent_rcv_file_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userUnknown")]
    UserUnknown,
    #[serde(rename = "activeUserExists")]
    ActiveUserExists,
    #[serde(rename = "userExists")]
    UserExists {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "differentActiveUser")]
    DifferentActiveUser {
        #[serde(
            rename = "commandUserId",
            deserialize_with = "deserialize_number_from_string"
        )]
        command_user_id: i64,

        #[serde(
            rename = "activeUserId",
            deserialize_with = "deserialize_number_from_string"
        )]
        active_user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "cantDeleteActiveUser")]
    CantDeleteActiveUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "cantDeleteLastUser")]
    CantDeleteLastUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "cantHideLastUser")]
    CantHideLastUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "hiddenUserAlwaysMuted")]
    HiddenUserAlwaysMuted {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "emptyUserPassword")]
    EmptyUserPassword {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userAlreadyHidden")]
    UserAlreadyHidden {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotHidden")]
    UserNotHidden {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invalidDisplayName")]
    InvalidDisplayName {
        #[serde(rename = "displayName")]
        display_name: String,

        #[serde(rename = "validName")]
        valid_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatNotStarted")]
    ChatNotStarted,
    #[serde(rename = "chatNotStopped")]
    ChatNotStopped,
    #[serde(rename = "chatStoreChanged")]
    ChatStoreChanged,
    #[serde(rename = "invalidConnReq")]
    InvalidConnReq,
    #[serde(rename = "unsupportedConnReq")]
    UnsupportedConnReq,
    #[serde(rename = "connReqMessageProhibited")]
    ConnReqMessageProhibited,
    #[serde(rename = "contactNotReady")]
    ContactNotReady {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotActive")]
    ContactNotActive {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactDisabled")]
    ContactDisabled {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "connectionDisabled")]
    ConnectionDisabled {
        #[serde(rename = "connection")]
        connection: Connection,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupUserRole")]
    GroupUserRole {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "requiredRole")]
        required_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberInitialRole")]
    GroupMemberInitialRole {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "initialRole")]
        initial_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactIncognitoCantInvite")]
    ContactIncognitoCantInvite,
    #[serde(rename = "groupIncognitoCantInvite")]
    GroupIncognitoCantInvite,
    #[serde(rename = "groupContactRole")]
    GroupContactRole {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupDuplicateMember")]
    GroupDuplicateMember {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupDuplicateMemberId")]
    GroupDuplicateMemberId,
    #[serde(rename = "groupNotJoined")]
    GroupNotJoined {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberNotActive")]
    GroupMemberNotActive,
    #[serde(rename = "cantBlockMemberForSelf")]
    CantBlockMemberForSelf {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "member")]
        member: GroupMember,

        #[serde(rename = "setShowMessages")]
        set_show_messages: bool,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberUserRemoved")]
    GroupMemberUserRemoved,
    #[serde(rename = "groupMemberNotFound")]
    GroupMemberNotFound,
    #[serde(rename = "groupCantResendInvitation")]
    GroupCantResendInvitation {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupInternal")]
    GroupInternal {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileNotFound")]
    FileNotFound {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileSize")]
    FileSize {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileAlreadyReceiving")]
    FileAlreadyReceiving {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileCancelled")]
    FileCancelled {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileCancel")]
    FileCancel {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileAlreadyExists")]
    FileAlreadyExists {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileRead")]
    FileRead {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileWrite")]
    FileWrite {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileSend")]
    FileSend {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "agentError")]
        agent_error: AgentErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileRcvChunk")]
    FileRcvChunk {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileInternal")]
    FileInternal {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileImageType")]
    FileImageType {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileImageSize")]
    FileImageSize {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileNotReceived")]
    FileNotReceived {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileNotApproved")]
    FileNotApproved {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "unknownServers")]
        unknown_servers: Vec<String>,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fallbackToSMPProhibited")]
    FallbackToSmpProhibited {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "inlineFileProhibited")]
    InlineFileProhibited {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invalidForward")]
    InvalidForward,
    #[serde(rename = "invalidChatItemUpdate")]
    InvalidChatItemUpdate,
    #[serde(rename = "invalidChatItemDelete")]
    InvalidChatItemDelete,
    #[serde(rename = "hasCurrentCall")]
    HasCurrentCall,
    #[serde(rename = "noCurrentCall")]
    NoCurrentCall,
    #[serde(rename = "callContact")]
    CallContact {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "directMessagesProhibited")]
    DirectMessagesProhibited {
        #[serde(rename = "direction")]
        direction: MsgDirection,

        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "agentVersion")]
    AgentVersion,
    #[serde(rename = "agentNoSubResult")]
    AgentNoSubResult {
        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "commandError")]
    CommandError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "agentCommandError")]
    AgentCommandError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invalidFileDescription")]
    InvalidFileDescription {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "connectionIncognitoChangeProhibited")]
    ConnectionIncognitoChangeProhibited,
    #[serde(rename = "connectionUserChangeProhibited")]
    ConnectionUserChangeProhibited,
    #[serde(rename = "peerChatVRangeIncompatible")]
    PeerChatVRangeIncompatible,
    #[serde(rename = "internalError")]
    InternalError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "exception")]
    Exception {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CommandError {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "SYNTAX")]
    Syntax,
    #[serde(rename = "PROHIBITED")]
    Prohibited,
    #[serde(rename = "NO_AUTH")]
    NoAuth,
    #[serde(rename = "HAS_AUTH")]
    HasAuth,
    #[serde(rename = "NO_ENTITY")]
    NoEntity,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CommandErrorType {
    #[serde(rename = "PROHIBITED")]
    Prohibited,
    #[serde(rename = "SYNTAX")]
    Syntax,
    #[serde(rename = "NO_CONN")]
    NoConn,
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "LARGE")]
    Large,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConnectionErrorType {
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "DUPLICATE")]
    Duplicate,
    #[serde(rename = "SIMPLEX")]
    Simplex,
    #[serde(rename = "NOT_ACCEPTED")]
    NotAccepted,
    #[serde(rename = "NOT_AVAILABLE")]
    NotAvailable,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ErrorType {
    #[serde(rename = "BLOCK")]
    Block,
    #[serde(rename = "SESSION")]
    Session,
    #[serde(rename = "CMD")]
    Cmd {
        #[serde(rename = "cmdErr")]
        cmd_err: CommandError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "PROXY")]
    Proxy {
        #[serde(rename = "proxyErr")]
        proxy_err: Arc<ProxyError>,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BLOCKED")]
    Blocked {
        #[serde(rename = "blockInfo")]
        block_info: BlockingInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "SERVICE")]
    Service,
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "QUOTA")]
    Quota,
    #[serde(rename = "STORE")]
    Store {
        #[serde(rename = "storeErr")]
        store_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "NO_MSG")]
    NoMsg,
    #[serde(rename = "LARGE_MSG")]
    LargeMsg,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "DUPLICATE_")]
    Duplicate,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FileError {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "blocked")]
    Blocked {
        #[serde(rename = "server")]
        server: String,

        #[serde(rename = "blockInfo")]
        block_info: BlockingInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "noFile")]
    NoFile,
    #[serde(rename = "relay")]
    Relay {
        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "other")]
    Other {
        #[serde(rename = "fileError")]
        file_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FileErrorType {
    #[serde(rename = "NOT_APPROVED")]
    NotApproved,
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "REDIRECT")]
    Redirect {
        #[serde(rename = "redirectError")]
        redirect_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "FILE_IO")]
    FileIo {
        #[serde(rename = "fileIOError")]
        file_io_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "NO_FILE")]
    NoFile,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HandshakeError {
    #[default]
    #[serde(rename = "PARSE")]
    Parse,
    #[serde(rename = "IDENTITY")]
    Identity,
    #[serde(rename = "BAD_AUTH")]
    BadAuth,
    #[serde(rename = "BAD_SERVICE")]
    BadService,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MsgDecryptError {
    #[default]
    #[serde(rename = "ratchetHeader")]
    RatchetHeader,
    #[serde(rename = "tooManySkipped")]
    TooManySkipped,
    #[serde(rename = "ratchetEarlier")]
    RatchetEarlier,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "ratchetSync")]
    RatchetSync,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum MsgErrorType {
    #[serde(rename = "msgSkipped")]
    MsgSkipped {
        #[serde(
            rename = "fromMsgId",
            deserialize_with = "deserialize_number_from_string"
        )]
        from_msg_id: i64,

        #[serde(
            rename = "toMsgId",
            deserialize_with = "deserialize_number_from_string"
        )]
        to_msg_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "msgBadId")]
    MsgBadId {
        #[serde(rename = "msgId", deserialize_with = "deserialize_number_from_string")]
        msg_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "msgBadHash")]
    MsgBadHash,
    #[serde(rename = "msgDuplicate")]
    MsgDuplicate,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum NetworkError {
    #[serde(rename = "connectError")]
    ConnectError {
        #[serde(rename = "connectError")]
        connect_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "tLSError")]
    TLsError {
        #[serde(rename = "tlsError")]
        tls_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "unknownCAError")]
    UnknownCaError,
    #[serde(rename = "failedError")]
    FailedError,
    #[serde(rename = "timeoutError")]
    TimeoutError,
    #[serde(rename = "subscribeError")]
    SubscribeError {
        #[serde(rename = "subscribeError")]
        subscribe_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ProxyClientError {
    #[serde(rename = "protocolError")]
    ProtocolError {
        #[serde(rename = "protocolErr")]
        protocol_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "unexpectedResponse")]
    UnexpectedResponse {
        #[serde(rename = "responseStr")]
        response_str: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "responseError")]
    ResponseError {
        #[serde(rename = "responseErr")]
        response_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ProxyError {
    #[serde(rename = "PROTOCOL")]
    Protocol {
        #[serde(rename = "protocolErr")]
        protocol_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "BROKER")]
    Broker {
        #[serde(rename = "brokerErr")]
        broker_err: BrokerErrorType,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "BASIC_AUTH")]
    BasicAuth,
    #[serde(rename = "NO_SESSION")]
    NoSession,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RCErrorType {
    #[serde(rename = "internal")]
    Internal {
        #[serde(rename = "internalErr")]
        internal_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "noLocalAddress")]
    NoLocalAddress,
    #[serde(rename = "newController")]
    NewController,
    #[serde(rename = "notDiscovered")]
    NotDiscovered,
    #[serde(rename = "tLSStartFailed")]
    TLsStartFailed,
    #[serde(rename = "exception")]
    Exception {
        #[serde(rename = "exception")]
        exception: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "ctrlAuth")]
    CtrlAuth,
    #[serde(rename = "ctrlNotFound")]
    CtrlNotFound,
    #[serde(rename = "ctrlError")]
    CtrlError {
        #[serde(rename = "ctrlErr")]
        ctrl_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invitation")]
    Invitation,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "encrypt")]
    Encrypt,
    #[serde(rename = "decrypt")]
    Decrypt,
    #[serde(rename = "blockSize")]
    BlockSize,
    #[serde(rename = "syntax")]
    Syntax {
        #[serde(rename = "syntaxErr")]
        syntax_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SMPAgentError {
    #[serde(rename = "A_MESSAGE")]
    AMessage,
    #[serde(rename = "A_PROHIBITED")]
    AProhibited {
        #[serde(rename = "prohibitedErr")]
        prohibited_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "A_VERSION")]
    AVersion,
    #[serde(rename = "A_LINK")]
    ALink {
        #[serde(rename = "linkErr")]
        link_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "A_CRYPTO")]
    ACrypto {
        #[serde(rename = "cryptoErr")]
        crypto_err: AgentCryptoError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "A_DUPLICATE")]
    ADuplicate,
    #[serde(rename = "A_QUEUE")]
    AQueue {
        #[serde(rename = "queueErr")]
        queue_err: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SndError {
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "quota")]
    Quota,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "relay")]
    Relay {
        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "proxy")]
    Proxy {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "proxyRelay")]
    ProxyRelay {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "other")]
    Other {
        #[serde(rename = "sndError")]
        snd_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SrvError {
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "other")]
    Other {
        #[serde(rename = "srvError")]
        srv_error: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum StoreError {
    #[serde(rename = "duplicateName")]
    DuplicateName,
    #[serde(rename = "userNotFound")]
    UserNotFound {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotFoundByName")]
    UserNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotFoundByContactId")]
    UserNotFoundByContactId {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotFoundByGroupId")]
    UserNotFoundByGroupId {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotFoundByFileId")]
    UserNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNotFoundByContactRequestId")]
    UserNotFoundByContactRequestId {
        #[serde(
            rename = "contactRequestId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_request_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotFound")]
    ContactNotFound {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotFoundByName")]
    ContactNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotFoundByMemberId")]
    ContactNotFoundByMemberId {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotReady")]
    ContactNotReady {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "duplicateContactLink")]
    DuplicateContactLink,
    #[serde(rename = "userContactLinkNotFound")]
    UserContactLinkNotFound,
    #[serde(rename = "contactRequestNotFound")]
    ContactRequestNotFound {
        #[serde(
            rename = "contactRequestId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_request_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactRequestNotFoundByName")]
    ContactRequestNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invalidContactRequestEntity")]
    InvalidContactRequestEntity {
        #[serde(
            rename = "contactRequestId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_request_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "invalidBusinessChatContactRequest")]
    InvalidBusinessChatContactRequest,
    #[serde(rename = "groupNotFound")]
    GroupNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupNotFoundByName")]
    GroupNotFoundByName {
        #[serde(rename = "groupName")]
        group_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberNameNotFound")]
    GroupMemberNameNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(rename = "groupMemberName")]
        group_member_name: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberNotFound")]
    GroupMemberNotFound {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupHostMemberNotFound")]
    GroupHostMemberNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupMemberNotFoundByMemberId")]
    GroupMemberNotFoundByMemberId {
        #[serde(rename = "memberId")]
        member_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "memberContactGroupMemberNotFound")]
    MemberContactGroupMemberNotFound {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupWithoutUser")]
    GroupWithoutUser,
    #[serde(rename = "duplicateGroupMember")]
    DuplicateGroupMember,
    #[serde(rename = "groupAlreadyJoined")]
    GroupAlreadyJoined,
    #[serde(rename = "groupInvitationNotFound")]
    GroupInvitationNotFound,
    #[serde(rename = "noteFolderAlreadyExists")]
    NoteFolderAlreadyExists {
        #[serde(
            rename = "noteFolderId",
            deserialize_with = "deserialize_number_from_string"
        )]
        note_folder_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "noteFolderNotFound")]
    NoteFolderNotFound {
        #[serde(
            rename = "noteFolderId",
            deserialize_with = "deserialize_number_from_string"
        )]
        note_folder_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "userNoteFolderNotFound")]
    UserNoteFolderNotFound,
    #[serde(rename = "sndFileNotFound")]
    SndFileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "sndFileInvalid")]
    SndFileInvalid {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileNotFound")]
    RcvFileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileDescrNotFound")]
    RcvFileDescrNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileNotFound")]
    FileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileInvalid")]
    RcvFileInvalid {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileInvalidDescrPart")]
    RcvFileInvalidDescrPart,
    #[serde(rename = "localFileNoTransfer")]
    LocalFileNoTransfer {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "sharedMsgIdNotFoundByFileId")]
    SharedMsgIdNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "fileIdNotFoundBySharedMsgId")]
    FileIdNotFoundBySharedMsgId {
        #[serde(rename = "sharedMsgId")]
        shared_msg_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "sndFileNotFoundXFTP")]
    SndFileNotFoundXftp {
        #[serde(rename = "agentSndFileId")]
        agent_snd_file_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "rcvFileNotFoundXFTP")]
    RcvFileNotFoundXftp {
        #[serde(rename = "agentRcvFileId")]
        agent_rcv_file_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "connectionNotFound")]
    ConnectionNotFound {
        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "connectionNotFoundById")]
    ConnectionNotFoundById {
        #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
        conn_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "connectionNotFoundByMemberId")]
    ConnectionNotFoundByMemberId {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "pendingConnectionNotFound")]
    PendingConnectionNotFound {
        #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
        conn_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "introNotFound")]
    IntroNotFound,
    #[serde(rename = "uniqueID")]
    UniqueId,
    #[serde(rename = "largeMsg")]
    LargeMsg,
    #[serde(rename = "internalError")]
    InternalError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "dBException")]
    DBException {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "dBBusyError")]
    DBBusyError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "badChatItem")]
    BadChatItem {
        #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
        item_id: i64,

        #[serde(rename = "itemTs", skip_serializing_if = "Option::is_none")]
        item_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemNotFound")]
    ChatItemNotFound {
        #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
        item_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemNotFoundByText")]
    ChatItemNotFoundByText {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemSharedMsgIdNotFound")]
    ChatItemSharedMsgIdNotFound {
        #[serde(rename = "sharedMsgId")]
        shared_msg_id: String,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemNotFoundByFileId")]
    ChatItemNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemNotFoundByContactId")]
    ChatItemNotFoundByContactId {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "chatItemNotFoundByGroupId")]
    ChatItemNotFoundByGroupId {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "profileNotFound")]
    ProfileNotFound {
        #[serde(
            rename = "profileId",
            deserialize_with = "deserialize_number_from_string"
        )]
        profile_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "duplicateGroupLink")]
    DuplicateGroupLink {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "groupLinkNotFound")]
    GroupLinkNotFound {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "hostMemberIdNotFound")]
    HostMemberIdNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "contactNotFoundByFileId")]
    ContactNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "noGroupSndStatus")]
    NoGroupSndStatus {
        #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
        item_id: i64,

        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "duplicateGroupMessage")]
    DuplicateGroupMessage {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(rename = "sharedMsgId")]
        shared_msg_id: String,

        #[serde(
            rename = "authorGroupMemberId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        author_group_member_id: Option<i64>,

        #[serde(
            rename = "forwardedByGroupMemberId",
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_option_number_from_string",
            default
        )]
        forwarded_by_group_member_id: Option<i64>,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "remoteHostNotFound")]
    RemoteHostNotFound {
        #[serde(
            rename = "remoteHostId",
            deserialize_with = "deserialize_number_from_string"
        )]
        remote_host_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "remoteHostUnknown")]
    RemoteHostUnknown,
    #[serde(rename = "remoteHostDuplicateCA")]
    RemoteHostDuplicateCa,
    #[serde(rename = "remoteCtrlNotFound")]
    RemoteCtrlNotFound {
        #[serde(
            rename = "remoteCtrlId",
            deserialize_with = "deserialize_number_from_string"
        )]
        remote_ctrl_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "remoteCtrlDuplicateCA")]
    RemoteCtrlDuplicateCa,
    #[serde(rename = "prohibitedDeleteUser")]
    ProhibitedDeleteUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "operatorNotFound")]
    OperatorNotFound {
        #[serde(
            rename = "serverOperatorId",
            deserialize_with = "deserialize_number_from_string"
        )]
        server_operator_id: i64,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "usageConditionsNotFound")]
    UsageConditionsNotFound,
    #[serde(rename = "invalidQuote")]
    InvalidQuote,
    #[serde(rename = "invalidMention")]
    InvalidMention,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TransportError {
    #[serde(rename = "badBlock")]
    BadBlock,
    #[serde(rename = "version")]
    Version,
    #[serde(rename = "largeMsg")]
    LargeMsg,
    #[serde(rename = "badSession")]
    BadSession,
    #[serde(rename = "noServerAuth")]
    NoServerAuth,
    #[serde(rename = "handshake")]
    Handshake {
        #[serde(rename = "handshakeErr")]
        handshake_err: HandshakeError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum XFTPErrorType {
    #[serde(rename = "BLOCK")]
    Block,
    #[serde(rename = "SESSION")]
    Session,
    #[serde(rename = "HANDSHAKE")]
    Handshake,
    #[serde(rename = "CMD")]
    Cmd {
        #[serde(rename = "cmdErr")]
        cmd_err: CommandError,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BLOCKED")]
    Blocked {
        #[serde(rename = "blockInfo")]
        block_info: BlockingInfo,

        #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
        undocumented: HashMap<String, JsonObject>,
    },
    #[serde(rename = "SIZE")]
    Size,
    #[serde(rename = "QUOTA")]
    Quota,
    #[serde(rename = "DIGEST")]
    Digest,
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "NO_FILE")]
    NoFile,
    #[serde(rename = "HAS_FILE")]
    HasFile,
    #[serde(rename = "FILE_IO")]
    FileIo,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "DUPLICATE_")]
    Duplicate,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

macro_rules! impl_error {
    ($($t:ty),+ $(,)?) => (
        $(
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#?}", self)
            }
        }

        impl std::error::Error for $t {}
        )+
    );
}

impl_error!(
    AgentCryptoError,
    AgentErrorType,
    BrokerErrorType,
    ChatError,
    ChatErrorType,
    CommandError,
    CommandErrorType,
    ConnectionErrorType,
    ErrorType,
    FileError,
    FileErrorType,
    HandshakeError,
    MsgDecryptError,
    MsgErrorType,
    NetworkError,
    ProxyClientError,
    ProxyError,
    RCErrorType,
    SMPAgentError,
    SndError,
    SrvError,
    StoreError,
    TransportError,
    XFTPErrorType
);
