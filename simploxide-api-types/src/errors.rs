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

impl AgentCryptoError {
    pub fn is_decrypt_aes(&self) -> bool {
        matches!(self, Self::DecryptAes)
    }
    pub fn is_decrypt_cb(&self) -> bool {
        matches!(self, Self::DecryptCb)
    }
    pub fn is_ratchet_header(&self) -> bool {
        matches!(self, Self::RatchetHeader)
    }
    pub fn is_ratchet_sync(&self) -> bool {
        matches!(self, Self::RatchetSync)
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "CONN")]
    Conn {
        #[serde(rename = "connErr")]
        conn_err: ConnectionErrorType,

        #[serde(rename = "errContext")]
        err_context: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "NO_USER")]
    NoUser,
    #[serde(rename = "SMP")]
    Smp {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "smpErr")]
        smp_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "NTF")]
    Ntf {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "ntfErr")]
        ntf_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "XFTP")]
    Xftp {
        #[serde(rename = "serverAddress")]
        server_address: String,

        #[serde(rename = "xftpErr")]
        xftp_err: XFTPErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "FILE")]
    File {
        #[serde(rename = "fileErr")]
        file_err: FileErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "PROXY")]
    Proxy {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "relayServer")]
        relay_server: String,

        #[serde(rename = "proxyErr")]
        proxy_err: ProxyClientError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "RCP")]
    Rcp {
        #[serde(rename = "rcpErr")]
        rcp_err: RCErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "BROKER")]
    Broker {
        #[serde(rename = "brokerAddress")]
        broker_address: String,

        #[serde(rename = "brokerErr")]
        broker_err: BrokerErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "AGENT")]
    Agent {
        #[serde(rename = "agentErr")]
        agent_err: SMPAgentError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "NOTICE")]
    Notice {
        #[serde(rename = "server")]
        server: String,

        #[serde(rename = "preset", default)]
        preset: bool,

        #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
        expires_at: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "INTERNAL")]
    Internal {
        #[serde(rename = "internalErr")]
        internal_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "CRITICAL")]
    Critical {
        #[serde(rename = "offerRestart", default)]
        offer_restart: bool,

        #[serde(rename = "criticalErr")]
        critical_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl AgentErrorType {
    pub fn cmd(&self) -> Option<AgentErrorTypeCmdRef<'_>> {
        if let Self::Cmd {
            cmd_err,
            err_context,
            ..
        } = self
        {
            Some(AgentErrorTypeCmdRef {
                cmd_err,
                err_context,
            })
        } else {
            None
        }
    }
    pub fn conn(&self) -> Option<AgentErrorTypeConnRef<'_>> {
        if let Self::Conn {
            conn_err,
            err_context,
            ..
        } = self
        {
            Some(AgentErrorTypeConnRef {
                conn_err,
                err_context,
            })
        } else {
            None
        }
    }
    pub fn is_no_user(&self) -> bool {
        matches!(self, Self::NoUser)
    }
    pub fn smp(&self) -> Option<AgentErrorTypeSmpRef<'_>> {
        if let Self::Smp {
            server_address,
            smp_err,
            ..
        } = self
        {
            Some(AgentErrorTypeSmpRef {
                server_address,
                smp_err,
            })
        } else {
            None
        }
    }
    pub fn ntf(&self) -> Option<AgentErrorTypeNtfRef<'_>> {
        if let Self::Ntf {
            server_address,
            ntf_err,
            ..
        } = self
        {
            Some(AgentErrorTypeNtfRef {
                server_address,
                ntf_err,
            })
        } else {
            None
        }
    }
    pub fn xftp(&self) -> Option<AgentErrorTypeXftpRef<'_>> {
        if let Self::Xftp {
            server_address,
            xftp_err,
            ..
        } = self
        {
            Some(AgentErrorTypeXftpRef {
                server_address,
                xftp_err,
            })
        } else {
            None
        }
    }
    pub fn file(&self) -> Option<&FileErrorType> {
        if let Self::File { file_err, .. } = self {
            Some(file_err)
        } else {
            None
        }
    }
    pub fn proxy(&self) -> Option<AgentErrorTypeProxyRef<'_>> {
        if let Self::Proxy {
            proxy_server,
            relay_server,
            proxy_err,
            ..
        } = self
        {
            Some(AgentErrorTypeProxyRef {
                proxy_server,
                relay_server,
                proxy_err,
            })
        } else {
            None
        }
    }
    pub fn rcp(&self) -> Option<&RCErrorType> {
        if let Self::Rcp { rcp_err, .. } = self {
            Some(rcp_err)
        } else {
            None
        }
    }
    pub fn broker(&self) -> Option<AgentErrorTypeBrokerRef<'_>> {
        if let Self::Broker {
            broker_address,
            broker_err,
            ..
        } = self
        {
            Some(AgentErrorTypeBrokerRef {
                broker_address,
                broker_err,
            })
        } else {
            None
        }
    }
    pub fn agent(&self) -> Option<&SMPAgentError> {
        if let Self::Agent { agent_err, .. } = self {
            Some(agent_err)
        } else {
            None
        }
    }
    pub fn notice(&self) -> Option<AgentErrorTypeNoticeRef<'_>> {
        if let Self::Notice {
            server,
            preset,
            expires_at,
            ..
        } = self
        {
            Some(AgentErrorTypeNoticeRef {
                server,
                preset,
                expires_at,
            })
        } else {
            None
        }
    }
    pub fn internal(&self) -> Option<&String> {
        if let Self::Internal { internal_err, .. } = self {
            Some(internal_err)
        } else {
            None
        }
    }
    pub fn critical(&self) -> Option<AgentErrorTypeCriticalRef<'_>> {
        if let Self::Critical {
            offer_restart,
            critical_err,
            ..
        } = self
        {
            Some(AgentErrorTypeCriticalRef {
                offer_restart,
                critical_err,
            })
        } else {
            None
        }
    }
    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeCmdRef<'a> {
    pub cmd_err: &'a CommandErrorType,
    pub err_context: &'a String,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeConnRef<'a> {
    pub conn_err: &'a ConnectionErrorType,
    pub err_context: &'a String,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeSmpRef<'a> {
    pub server_address: &'a String,
    pub smp_err: &'a ErrorType,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeNtfRef<'a> {
    pub server_address: &'a String,
    pub ntf_err: &'a ErrorType,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeXftpRef<'a> {
    pub server_address: &'a String,
    pub xftp_err: &'a XFTPErrorType,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeProxyRef<'a> {
    pub proxy_server: &'a String,
    pub relay_server: &'a String,
    pub proxy_err: &'a ProxyClientError,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeBrokerRef<'a> {
    pub broker_address: &'a String,
    pub broker_err: &'a BrokerErrorType,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeNoticeRef<'a> {
    pub server: &'a String,
    pub preset: &'a bool,
    pub expires_at: &'a Option<UtcTime>,
}
#[derive(Clone, Copy)]
pub struct AgentErrorTypeCriticalRef<'a> {
    pub offer_restart: &'a bool,
    pub critical_err: &'a String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BrokerErrorType {
    #[serde(rename = "RESPONSE")]
    Response {
        #[serde(rename = "respErr")]
        resp_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "UNEXPECTED")]
    Unexpected {
        #[serde(rename = "respErr")]
        resp_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "NETWORK")]
    Network {
        #[serde(rename = "networkError")]
        network_error: NetworkError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "HOST")]
    Host,
    #[serde(rename = "NO_SERVICE")]
    NoService,
    #[serde(rename = "TRANSPORT")]
    Transport {
        #[serde(rename = "transportErr")]
        transport_err: TransportError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl BrokerErrorType {
    pub fn response(&self) -> Option<&String> {
        if let Self::Response { resp_err, .. } = self {
            Some(resp_err)
        } else {
            None
        }
    }
    pub fn unexpected(&self) -> Option<&String> {
        if let Self::Unexpected { resp_err, .. } = self {
            Some(resp_err)
        } else {
            None
        }
    }
    pub fn network(&self) -> Option<&NetworkError> {
        if let Self::Network { network_error, .. } = self {
            Some(network_error)
        } else {
            None
        }
    }
    pub fn is_host(&self) -> bool {
        matches!(self, Self::Host)
    }
    pub fn is_no_service(&self) -> bool {
        matches!(self, Self::NoService)
    }
    pub fn transport(&self) -> Option<&TransportError> {
        if let Self::Transport { transport_err, .. } = self {
            Some(transport_err)
        } else {
            None
        }
    }
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ChatError {
    #[serde(rename = "error")]
    Error {
        #[serde(rename = "errorType")]
        error_type: ChatErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "errorAgent")]
    ErrorAgent {
        #[serde(rename = "agentError")]
        agent_error: AgentErrorType,

        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(rename = "connectionEntity_", skip_serializing_if = "Option::is_none")]
        connection_entity: Option<ConnectionEntity>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "errorStore")]
    ErrorStore {
        #[serde(rename = "storeError")]
        store_error: StoreError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ChatError {
    pub fn error(&self) -> Option<&ChatErrorType> {
        if let Self::Error { error_type, .. } = self {
            Some(error_type)
        } else {
            None
        }
    }
    pub fn error_agent(&self) -> Option<ChatErrorErrorAgentRef<'_>> {
        if let Self::ErrorAgent {
            agent_error,
            agent_conn_id,
            connection_entity,
            ..
        } = self
        {
            Some(ChatErrorErrorAgentRef {
                agent_error,
                agent_conn_id,
                connection_entity,
            })
        } else {
            None
        }
    }
    pub fn error_store(&self) -> Option<&StoreError> {
        if let Self::ErrorStore { store_error, .. } = self {
            Some(store_error)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct ChatErrorErrorAgentRef<'a> {
    pub agent_error: &'a AgentErrorType,
    pub agent_conn_id: &'a String,
    pub connection_entity: &'a Option<ConnectionEntity>,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "noSndFileUser")]
    NoSndFileUser {
        #[serde(rename = "agentSndFileId")]
        agent_snd_file_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "noRcvFileUser")]
    NoRcvFileUser {
        #[serde(rename = "agentRcvFileId")]
        agent_rcv_file_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userUnknown")]
    UserUnknown,
    #[serde(rename = "activeUserExists")]
    ActiveUserExists,
    #[serde(rename = "userExists")]
    UserExists {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatRelayExists")]
    ChatRelayExists,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "cantDeleteActiveUser")]
    CantDeleteActiveUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "cantDeleteLastUser")]
    CantDeleteLastUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "cantHideLastUser")]
    CantHideLastUser {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "hiddenUserAlwaysMuted")]
    HiddenUserAlwaysMuted {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "emptyUserPassword")]
    EmptyUserPassword {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userAlreadyHidden")]
    UserAlreadyHidden {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNotHidden")]
    UserNotHidden {
        #[serde(rename = "userId", deserialize_with = "deserialize_number_from_string")]
        user_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidDisplayName")]
    InvalidDisplayName {
        #[serde(rename = "displayName")]
        display_name: String,

        #[serde(rename = "validName")]
        valid_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotActive")]
    ContactNotActive {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactDisabled")]
    ContactDisabled {
        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectionDisabled")]
    ConnectionDisabled {
        #[serde(rename = "connection")]
        connection: Connection,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupUserRole")]
    GroupUserRole {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "requiredRole")]
        required_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupMemberInitialRole")]
    GroupMemberInitialRole {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "initialRole")]
        initial_role: GroupMemberRole,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactIncognitoCantInvite")]
    ContactIncognitoCantInvite,
    #[serde(rename = "groupIncognitoCantInvite")]
    GroupIncognitoCantInvite,
    #[serde(rename = "groupContactRole")]
    GroupContactRole {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupDuplicateMember")]
    GroupDuplicateMember {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupDuplicateMemberId")]
    GroupDuplicateMemberId,
    #[serde(rename = "groupNotJoined")]
    GroupNotJoined {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupMemberNotActive")]
    GroupMemberNotActive,
    #[serde(rename = "cantBlockMemberForSelf")]
    CantBlockMemberForSelf {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(rename = "member")]
        member: GroupMember,

        #[serde(rename = "setShowMessages", default)]
        set_show_messages: bool,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupInternal")]
    GroupInternal {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileNotFound")]
    FileNotFound {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileSize")]
    FileSize {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileAlreadyReceiving")]
    FileAlreadyReceiving {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileCancelled")]
    FileCancelled {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileCancel")]
    FileCancel {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileAlreadyExists")]
    FileAlreadyExists {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileWrite")]
    FileWrite {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileSend")]
    FileSend {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "agentError")]
        agent_error: AgentErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileRcvChunk")]
    FileRcvChunk {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileInternal")]
    FileInternal {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileImageType")]
    FileImageType {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileImageSize")]
    FileImageSize {
        #[serde(rename = "filePath")]
        file_path: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileNotReceived")]
    FileNotReceived {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileNotApproved")]
    FileNotApproved {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(rename = "unknownServers")]
        unknown_servers: Vec<String>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fallbackToSMPProhibited")]
    FallbackToSmpProhibited {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "inlineFileProhibited")]
    InlineFileProhibited {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "directMessagesProhibited")]
    DirectMessagesProhibited {
        #[serde(rename = "direction")]
        direction: MsgDirection,

        #[serde(rename = "contact")]
        contact: Contact,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "agentVersion")]
    AgentVersion,
    #[serde(rename = "agentNoSubResult")]
    AgentNoSubResult {
        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "commandError")]
    CommandError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "agentCommandError")]
    AgentCommandError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidFileDescription")]
    InvalidFileDescription {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectionIncognitoChangeProhibited")]
    ConnectionIncognitoChangeProhibited,
    #[serde(rename = "connectionUserChangeProhibited")]
    ConnectionUserChangeProhibited,
    #[serde(rename = "peerChatVRangeIncompatible")]
    PeerChatVRangeIncompatible,
    #[serde(rename = "relayTestError")]
    RelayTestError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "internalError")]
    InternalError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "exception")]
    Exception {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ChatErrorType {
    pub fn is_no_active_user(&self) -> bool {
        matches!(self, Self::NoActiveUser)
    }
    pub fn no_connection_user(&self) -> Option<&String> {
        if let Self::NoConnectionUser { agent_conn_id, .. } = self {
            Some(agent_conn_id)
        } else {
            None
        }
    }
    pub fn no_snd_file_user(&self) -> Option<&String> {
        if let Self::NoSndFileUser {
            agent_snd_file_id, ..
        } = self
        {
            Some(agent_snd_file_id)
        } else {
            None
        }
    }
    pub fn no_rcv_file_user(&self) -> Option<&String> {
        if let Self::NoRcvFileUser {
            agent_rcv_file_id, ..
        } = self
        {
            Some(agent_rcv_file_id)
        } else {
            None
        }
    }
    pub fn is_user_unknown(&self) -> bool {
        matches!(self, Self::UserUnknown)
    }
    pub fn is_active_user_exists(&self) -> bool {
        matches!(self, Self::ActiveUserExists)
    }
    pub fn user_exists(&self) -> Option<&String> {
        if let Self::UserExists { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn is_chat_relay_exists(&self) -> bool {
        matches!(self, Self::ChatRelayExists)
    }
    pub fn different_active_user(&self) -> Option<ChatErrorTypeDifferentActiveUserRef<'_>> {
        if let Self::DifferentActiveUser {
            command_user_id,
            active_user_id,
            ..
        } = self
        {
            Some(ChatErrorTypeDifferentActiveUserRef {
                command_user_id,
                active_user_id,
            })
        } else {
            None
        }
    }
    pub fn cant_delete_active_user(&self) -> Option<&i64> {
        if let Self::CantDeleteActiveUser { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn cant_delete_last_user(&self) -> Option<&i64> {
        if let Self::CantDeleteLastUser { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn cant_hide_last_user(&self) -> Option<&i64> {
        if let Self::CantHideLastUser { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn hidden_user_always_muted(&self) -> Option<&i64> {
        if let Self::HiddenUserAlwaysMuted { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn empty_user_password(&self) -> Option<&i64> {
        if let Self::EmptyUserPassword { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn user_already_hidden(&self) -> Option<&i64> {
        if let Self::UserAlreadyHidden { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn user_not_hidden(&self) -> Option<&i64> {
        if let Self::UserNotHidden { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn invalid_display_name(&self) -> Option<ChatErrorTypeInvalidDisplayNameRef<'_>> {
        if let Self::InvalidDisplayName {
            display_name,
            valid_name,
            ..
        } = self
        {
            Some(ChatErrorTypeInvalidDisplayNameRef {
                display_name,
                valid_name,
            })
        } else {
            None
        }
    }
    pub fn is_chat_not_started(&self) -> bool {
        matches!(self, Self::ChatNotStarted)
    }
    pub fn is_chat_not_stopped(&self) -> bool {
        matches!(self, Self::ChatNotStopped)
    }
    pub fn is_chat_store_changed(&self) -> bool {
        matches!(self, Self::ChatStoreChanged)
    }
    pub fn is_invalid_conn_req(&self) -> bool {
        matches!(self, Self::InvalidConnReq)
    }
    pub fn is_unsupported_conn_req(&self) -> bool {
        matches!(self, Self::UnsupportedConnReq)
    }
    pub fn is_conn_req_message_prohibited(&self) -> bool {
        matches!(self, Self::ConnReqMessageProhibited)
    }
    pub fn contact_not_ready(&self) -> Option<&Contact> {
        if let Self::ContactNotReady { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn contact_not_active(&self) -> Option<&Contact> {
        if let Self::ContactNotActive { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn contact_disabled(&self) -> Option<&Contact> {
        if let Self::ContactDisabled { contact, .. } = self {
            Some(contact)
        } else {
            None
        }
    }
    pub fn connection_disabled(&self) -> Option<&Connection> {
        if let Self::ConnectionDisabled { connection, .. } = self {
            Some(connection)
        } else {
            None
        }
    }
    pub fn group_user_role(&self) -> Option<ChatErrorTypeGroupUserRoleRef<'_>> {
        if let Self::GroupUserRole {
            group_info,
            required_role,
            ..
        } = self
        {
            Some(ChatErrorTypeGroupUserRoleRef {
                group_info,
                required_role,
            })
        } else {
            None
        }
    }
    pub fn group_member_initial_role(&self) -> Option<ChatErrorTypeGroupMemberInitialRoleRef<'_>> {
        if let Self::GroupMemberInitialRole {
            group_info,
            initial_role,
            ..
        } = self
        {
            Some(ChatErrorTypeGroupMemberInitialRoleRef {
                group_info,
                initial_role,
            })
        } else {
            None
        }
    }
    pub fn is_contact_incognito_cant_invite(&self) -> bool {
        matches!(self, Self::ContactIncognitoCantInvite)
    }
    pub fn is_group_incognito_cant_invite(&self) -> bool {
        matches!(self, Self::GroupIncognitoCantInvite)
    }
    pub fn group_contact_role(&self) -> Option<&String> {
        if let Self::GroupContactRole { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn group_duplicate_member(&self) -> Option<&String> {
        if let Self::GroupDuplicateMember { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn is_group_duplicate_member_id(&self) -> bool {
        matches!(self, Self::GroupDuplicateMemberId)
    }
    pub fn group_not_joined(&self) -> Option<&GroupInfo> {
        if let Self::GroupNotJoined { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
    pub fn is_group_member_not_active(&self) -> bool {
        matches!(self, Self::GroupMemberNotActive)
    }
    pub fn cant_block_member_for_self(&self) -> Option<ChatErrorTypeCantBlockMemberForSelfRef<'_>> {
        if let Self::CantBlockMemberForSelf {
            group_info,
            member,
            set_show_messages,
            ..
        } = self
        {
            Some(ChatErrorTypeCantBlockMemberForSelfRef {
                group_info,
                member,
                set_show_messages,
            })
        } else {
            None
        }
    }
    pub fn is_group_member_user_removed(&self) -> bool {
        matches!(self, Self::GroupMemberUserRemoved)
    }
    pub fn is_group_member_not_found(&self) -> bool {
        matches!(self, Self::GroupMemberNotFound)
    }
    pub fn group_cant_resend_invitation(
        &self,
    ) -> Option<ChatErrorTypeGroupCantResendInvitationRef<'_>> {
        if let Self::GroupCantResendInvitation {
            group_info,
            contact_name,
            ..
        } = self
        {
            Some(ChatErrorTypeGroupCantResendInvitationRef {
                group_info,
                contact_name,
            })
        } else {
            None
        }
    }
    pub fn group_internal(&self) -> Option<&String> {
        if let Self::GroupInternal { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_not_found(&self) -> Option<&String> {
        if let Self::FileNotFound { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_size(&self) -> Option<&String> {
        if let Self::FileSize { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn file_already_receiving(&self) -> Option<&String> {
        if let Self::FileAlreadyReceiving { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_cancelled(&self) -> Option<&String> {
        if let Self::FileCancelled { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_cancel(&self) -> Option<ChatErrorTypeFileCancelRef<'_>> {
        if let Self::FileCancel {
            file_id, message, ..
        } = self
        {
            Some(ChatErrorTypeFileCancelRef { file_id, message })
        } else {
            None
        }
    }
    pub fn file_already_exists(&self) -> Option<&String> {
        if let Self::FileAlreadyExists { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn file_write(&self) -> Option<ChatErrorTypeFileWriteRef<'_>> {
        if let Self::FileWrite {
            file_path, message, ..
        } = self
        {
            Some(ChatErrorTypeFileWriteRef { file_path, message })
        } else {
            None
        }
    }
    pub fn file_send(&self) -> Option<ChatErrorTypeFileSendRef<'_>> {
        if let Self::FileSend {
            file_id,
            agent_error,
            ..
        } = self
        {
            Some(ChatErrorTypeFileSendRef {
                file_id,
                agent_error,
            })
        } else {
            None
        }
    }
    pub fn file_rcv_chunk(&self) -> Option<&String> {
        if let Self::FileRcvChunk { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_internal(&self) -> Option<&String> {
        if let Self::FileInternal { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn file_image_type(&self) -> Option<&String> {
        if let Self::FileImageType { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn file_image_size(&self) -> Option<&String> {
        if let Self::FileImageSize { file_path, .. } = self {
            Some(file_path)
        } else {
            None
        }
    }
    pub fn file_not_received(&self) -> Option<&i64> {
        if let Self::FileNotReceived { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn file_not_approved(&self) -> Option<ChatErrorTypeFileNotApprovedRef<'_>> {
        if let Self::FileNotApproved {
            file_id,
            unknown_servers,
            ..
        } = self
        {
            Some(ChatErrorTypeFileNotApprovedRef {
                file_id,
                unknown_servers,
            })
        } else {
            None
        }
    }
    pub fn fallback_to_smp_prohibited(&self) -> Option<&i64> {
        if let Self::FallbackToSmpProhibited { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn inline_file_prohibited(&self) -> Option<&i64> {
        if let Self::InlineFileProhibited { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn is_invalid_forward(&self) -> bool {
        matches!(self, Self::InvalidForward)
    }
    pub fn is_invalid_chat_item_update(&self) -> bool {
        matches!(self, Self::InvalidChatItemUpdate)
    }
    pub fn is_invalid_chat_item_delete(&self) -> bool {
        matches!(self, Self::InvalidChatItemDelete)
    }
    pub fn is_has_current_call(&self) -> bool {
        matches!(self, Self::HasCurrentCall)
    }
    pub fn is_no_current_call(&self) -> bool {
        matches!(self, Self::NoCurrentCall)
    }
    pub fn call_contact(&self) -> Option<&i64> {
        if let Self::CallContact { contact_id, .. } = self {
            Some(contact_id)
        } else {
            None
        }
    }
    pub fn direct_messages_prohibited(
        &self,
    ) -> Option<ChatErrorTypeDirectMessagesProhibitedRef<'_>> {
        if let Self::DirectMessagesProhibited {
            direction, contact, ..
        } = self
        {
            Some(ChatErrorTypeDirectMessagesProhibitedRef { direction, contact })
        } else {
            None
        }
    }
    pub fn is_agent_version(&self) -> bool {
        matches!(self, Self::AgentVersion)
    }
    pub fn agent_no_sub_result(&self) -> Option<&String> {
        if let Self::AgentNoSubResult { agent_conn_id, .. } = self {
            Some(agent_conn_id)
        } else {
            None
        }
    }
    pub fn command_error(&self) -> Option<&String> {
        if let Self::CommandError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn agent_command_error(&self) -> Option<&String> {
        if let Self::AgentCommandError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn invalid_file_description(&self) -> Option<&String> {
        if let Self::InvalidFileDescription { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn is_connection_incognito_change_prohibited(&self) -> bool {
        matches!(self, Self::ConnectionIncognitoChangeProhibited)
    }
    pub fn is_connection_user_change_prohibited(&self) -> bool {
        matches!(self, Self::ConnectionUserChangeProhibited)
    }
    pub fn is_peer_chat_v_range_incompatible(&self) -> bool {
        matches!(self, Self::PeerChatVRangeIncompatible)
    }
    pub fn relay_test_error(&self) -> Option<&String> {
        if let Self::RelayTestError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn internal_error(&self) -> Option<&String> {
        if let Self::InternalError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn exception(&self) -> Option<&String> {
        if let Self::Exception { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeDifferentActiveUserRef<'a> {
    pub command_user_id: &'a i64,
    pub active_user_id: &'a i64,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeInvalidDisplayNameRef<'a> {
    pub display_name: &'a String,
    pub valid_name: &'a String,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeGroupUserRoleRef<'a> {
    pub group_info: &'a GroupInfo,
    pub required_role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeGroupMemberInitialRoleRef<'a> {
    pub group_info: &'a GroupInfo,
    pub initial_role: &'a GroupMemberRole,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeCantBlockMemberForSelfRef<'a> {
    pub group_info: &'a GroupInfo,
    pub member: &'a GroupMember,
    pub set_show_messages: &'a bool,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeGroupCantResendInvitationRef<'a> {
    pub group_info: &'a GroupInfo,
    pub contact_name: &'a String,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeFileCancelRef<'a> {
    pub file_id: &'a i64,
    pub message: &'a String,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeFileWriteRef<'a> {
    pub file_path: &'a String,
    pub message: &'a String,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeFileSendRef<'a> {
    pub file_id: &'a i64,
    pub agent_error: &'a AgentErrorType,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeFileNotApprovedRef<'a> {
    pub file_id: &'a i64,
    pub unknown_servers: &'a Vec<String>,
}
#[derive(Clone, Copy)]
pub struct ChatErrorTypeDirectMessagesProhibitedRef<'a> {
    pub direction: &'a MsgDirection,
    pub contact: &'a Contact,
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

impl CommandError {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    pub fn is_syntax(&self) -> bool {
        matches!(self, Self::Syntax)
    }
    pub fn is_prohibited(&self) -> bool {
        matches!(self, Self::Prohibited)
    }
    pub fn is_no_auth(&self) -> bool {
        matches!(self, Self::NoAuth)
    }
    pub fn is_has_auth(&self) -> bool {
        matches!(self, Self::HasAuth)
    }
    pub fn is_no_entity(&self) -> bool {
        matches!(self, Self::NoEntity)
    }
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

impl CommandErrorType {
    pub fn is_prohibited(&self) -> bool {
        matches!(self, Self::Prohibited)
    }
    pub fn is_syntax(&self) -> bool {
        matches!(self, Self::Syntax)
    }
    pub fn is_no_conn(&self) -> bool {
        matches!(self, Self::NoConn)
    }
    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size)
    }
    pub fn is_large(&self) -> bool {
        matches!(self, Self::Large)
    }
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

impl ConnectionErrorType {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }
    pub fn is_duplicate(&self) -> bool {
        matches!(self, Self::Duplicate)
    }
    pub fn is_simplex(&self) -> bool {
        matches!(self, Self::Simplex)
    }
    pub fn is_not_accepted(&self) -> bool {
        matches!(self, Self::NotAccepted)
    }
    pub fn is_not_available(&self) -> bool {
        matches!(self, Self::NotAvailable)
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "PROXY")]
    Proxy {
        #[serde(rename = "proxyErr")]
        proxy_err: Arc<ProxyError>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BLOCKED")]
    Blocked {
        #[serde(rename = "blockInfo")]
        block_info: BlockingInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

impl ErrorType {
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block)
    }
    pub fn is_session(&self) -> bool {
        matches!(self, Self::Session)
    }
    pub fn cmd(&self) -> Option<&CommandError> {
        if let Self::Cmd { cmd_err, .. } = self {
            Some(cmd_err)
        } else {
            None
        }
    }
    pub fn proxy(&self) -> Option<&Arc<ProxyError>> {
        if let Self::Proxy { proxy_err, .. } = self {
            Some(proxy_err)
        } else {
            None
        }
    }
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth)
    }
    pub fn blocked(&self) -> Option<&BlockingInfo> {
        if let Self::Blocked { block_info, .. } = self {
            Some(block_info)
        } else {
            None
        }
    }
    pub fn is_service(&self) -> bool {
        matches!(self, Self::Service)
    }
    pub fn is_crypto(&self) -> bool {
        matches!(self, Self::Crypto)
    }
    pub fn is_quota(&self) -> bool {
        matches!(self, Self::Quota)
    }
    pub fn store(&self) -> Option<&String> {
        if let Self::Store { store_err, .. } = self {
            Some(store_err)
        } else {
            None
        }
    }
    pub fn is_no_msg(&self) -> bool {
        matches!(self, Self::NoMsg)
    }
    pub fn is_large_msg(&self) -> bool {
        matches!(self, Self::LargeMsg)
    }
    pub fn is_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }
    pub fn is_internal(&self) -> bool {
        matches!(self, Self::Internal)
    }
    pub fn is_duplicate(&self) -> bool {
        matches!(self, Self::Duplicate)
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "noFile")]
    NoFile,
    #[serde(rename = "relay")]
    Relay {
        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "other")]
    Other {
        #[serde(rename = "fileError")]
        file_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl FileError {
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth)
    }
    pub fn blocked(&self) -> Option<FileErrorBlockedRef<'_>> {
        if let Self::Blocked {
            server, block_info, ..
        } = self
        {
            Some(FileErrorBlockedRef { server, block_info })
        } else {
            None
        }
    }
    pub fn is_no_file(&self) -> bool {
        matches!(self, Self::NoFile)
    }
    pub fn relay(&self) -> Option<&SrvError> {
        if let Self::Relay { srv_error, .. } = self {
            Some(srv_error)
        } else {
            None
        }
    }
    pub fn other(&self) -> Option<&String> {
        if let Self::Other { file_error, .. } = self {
            Some(file_error)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct FileErrorBlockedRef<'a> {
    pub server: &'a String,
    pub block_info: &'a BlockingInfo,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "FILE_IO")]
    FileIo {
        #[serde(rename = "fileIOError")]
        file_io_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "NO_FILE")]
    NoFile,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl FileErrorType {
    pub fn is_not_approved(&self) -> bool {
        matches!(self, Self::NotApproved)
    }
    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size)
    }
    pub fn redirect(&self) -> Option<&String> {
        if let Self::Redirect { redirect_error, .. } = self {
            Some(redirect_error)
        } else {
            None
        }
    }
    pub fn file_io(&self) -> Option<&String> {
        if let Self::FileIo { file_io_error, .. } = self {
            Some(file_io_error)
        } else {
            None
        }
    }
    pub fn is_no_file(&self) -> bool {
        matches!(self, Self::NoFile)
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "msgBadId")]
    MsgBadId {
        #[serde(rename = "msgId", deserialize_with = "deserialize_number_from_string")]
        msg_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "msgBadHash")]
    MsgBadHash,
    #[serde(rename = "msgDuplicate")]
    MsgDuplicate,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl MsgErrorType {
    pub fn msg_skipped(&self) -> Option<MsgErrorTypeMsgSkippedRef<'_>> {
        if let Self::MsgSkipped {
            from_msg_id,
            to_msg_id,
            ..
        } = self
        {
            Some(MsgErrorTypeMsgSkippedRef {
                from_msg_id,
                to_msg_id,
            })
        } else {
            None
        }
    }
    pub fn msg_bad_id(&self) -> Option<&i64> {
        if let Self::MsgBadId { msg_id, .. } = self {
            Some(msg_id)
        } else {
            None
        }
    }
    pub fn is_msg_bad_hash(&self) -> bool {
        matches!(self, Self::MsgBadHash)
    }
    pub fn is_msg_duplicate(&self) -> bool {
        matches!(self, Self::MsgDuplicate)
    }
}
#[derive(Clone, Copy)]
pub struct MsgErrorTypeMsgSkippedRef<'a> {
    pub from_msg_id: &'a i64,
    pub to_msg_id: &'a i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum NetworkError {
    #[serde(rename = "connectError")]
    ConnectError {
        #[serde(rename = "connectError")]
        connect_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "tLSError")]
    TLsError {
        #[serde(rename = "tlsError")]
        tls_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl NetworkError {
    pub fn connect_error(&self) -> Option<&String> {
        if let Self::ConnectError { connect_error, .. } = self {
            Some(connect_error)
        } else {
            None
        }
    }
    pub fn t_ls_error(&self) -> Option<&String> {
        if let Self::TLsError { tls_error, .. } = self {
            Some(tls_error)
        } else {
            None
        }
    }
    pub fn is_unknown_ca_error(&self) -> bool {
        matches!(self, Self::UnknownCaError)
    }
    pub fn is_failed_error(&self) -> bool {
        matches!(self, Self::FailedError)
    }
    pub fn is_timeout_error(&self) -> bool {
        matches!(self, Self::TimeoutError)
    }
    pub fn subscribe_error(&self) -> Option<&String> {
        if let Self::SubscribeError {
            subscribe_error, ..
        } = self
        {
            Some(subscribe_error)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ProxyClientError {
    #[serde(rename = "protocolError")]
    ProtocolError {
        #[serde(rename = "protocolErr")]
        protocol_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "unexpectedResponse")]
    UnexpectedResponse {
        #[serde(rename = "responseStr")]
        response_str: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "responseError")]
    ResponseError {
        #[serde(rename = "responseErr")]
        response_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ProxyClientError {
    pub fn protocol_error(&self) -> Option<&ErrorType> {
        if let Self::ProtocolError { protocol_err, .. } = self {
            Some(protocol_err)
        } else {
            None
        }
    }
    pub fn unexpected_response(&self) -> Option<&String> {
        if let Self::UnexpectedResponse { response_str, .. } = self {
            Some(response_str)
        } else {
            None
        }
    }
    pub fn response_error(&self) -> Option<&ErrorType> {
        if let Self::ResponseError { response_err, .. } = self {
            Some(response_err)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ProxyError {
    #[serde(rename = "PROTOCOL")]
    Protocol {
        #[serde(rename = "protocolErr")]
        protocol_err: ErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "BROKER")]
    Broker {
        #[serde(rename = "brokerErr")]
        broker_err: BrokerErrorType,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "BASIC_AUTH")]
    BasicAuth,
    #[serde(rename = "NO_SESSION")]
    NoSession,
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl ProxyError {
    pub fn protocol(&self) -> Option<&ErrorType> {
        if let Self::Protocol { protocol_err, .. } = self {
            Some(protocol_err)
        } else {
            None
        }
    }
    pub fn broker(&self) -> Option<&BrokerErrorType> {
        if let Self::Broker { broker_err, .. } = self {
            Some(broker_err)
        } else {
            None
        }
    }
    pub fn is_basic_auth(&self) -> bool {
        matches!(self, Self::BasicAuth)
    }
    pub fn is_no_session(&self) -> bool {
        matches!(self, Self::NoSession)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RCErrorType {
    #[serde(rename = "internal")]
    Internal {
        #[serde(rename = "internalErr")]
        internal_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "ctrlAuth")]
    CtrlAuth,
    #[serde(rename = "ctrlNotFound")]
    CtrlNotFound,
    #[serde(rename = "ctrlError")]
    CtrlError {
        #[serde(rename = "ctrlErr")]
        ctrl_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RCErrorType {
    pub fn internal(&self) -> Option<&String> {
        if let Self::Internal { internal_err, .. } = self {
            Some(internal_err)
        } else {
            None
        }
    }
    pub fn is_identity(&self) -> bool {
        matches!(self, Self::Identity)
    }
    pub fn is_no_local_address(&self) -> bool {
        matches!(self, Self::NoLocalAddress)
    }
    pub fn is_new_controller(&self) -> bool {
        matches!(self, Self::NewController)
    }
    pub fn is_not_discovered(&self) -> bool {
        matches!(self, Self::NotDiscovered)
    }
    pub fn is_t_ls_start_failed(&self) -> bool {
        matches!(self, Self::TLsStartFailed)
    }
    pub fn exception(&self) -> Option<&String> {
        if let Self::Exception { exception, .. } = self {
            Some(exception)
        } else {
            None
        }
    }
    pub fn is_ctrl_auth(&self) -> bool {
        matches!(self, Self::CtrlAuth)
    }
    pub fn is_ctrl_not_found(&self) -> bool {
        matches!(self, Self::CtrlNotFound)
    }
    pub fn ctrl_error(&self) -> Option<&String> {
        if let Self::CtrlError { ctrl_err, .. } = self {
            Some(ctrl_err)
        } else {
            None
        }
    }
    pub fn is_invitation(&self) -> bool {
        matches!(self, Self::Invitation)
    }
    pub fn is_version(&self) -> bool {
        matches!(self, Self::Version)
    }
    pub fn is_encrypt(&self) -> bool {
        matches!(self, Self::Encrypt)
    }
    pub fn is_decrypt(&self) -> bool {
        matches!(self, Self::Decrypt)
    }
    pub fn is_block_size(&self) -> bool {
        matches!(self, Self::BlockSize)
    }
    pub fn syntax(&self) -> Option<&String> {
        if let Self::Syntax { syntax_err, .. } = self {
            Some(syntax_err)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum RcvMsgError {
    #[serde(rename = "dropped")]
    Dropped {
        #[serde(
            rename = "attempts",
            deserialize_with = "deserialize_number_from_string"
        )]
        attempts: i32,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "parseError")]
    ParseError {
        #[serde(rename = "parseError")]
        parse_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl RcvMsgError {
    pub fn dropped(&self) -> Option<&i32> {
        if let Self::Dropped { attempts, .. } = self {
            Some(attempts)
        } else {
            None
        }
    }
    pub fn parse_error(&self) -> Option<&String> {
        if let Self::ParseError { parse_error, .. } = self {
            Some(parse_error)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "A_VERSION")]
    AVersion,
    #[serde(rename = "A_LINK")]
    ALink {
        #[serde(rename = "linkErr")]
        link_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "A_CRYPTO")]
    ACrypto {
        #[serde(rename = "cryptoErr")]
        crypto_err: AgentCryptoError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "A_DUPLICATE")]
    ADuplicate {
        #[serde(rename = "droppedMsg_", skip_serializing_if = "Option::is_none")]
        dropped_msg: Option<DroppedMsg>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "A_QUEUE")]
    AQueue {
        #[serde(rename = "queueErr")]
        queue_err: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SMPAgentError {
    pub fn is_a_message(&self) -> bool {
        matches!(self, Self::AMessage)
    }
    pub fn a_prohibited(&self) -> Option<&String> {
        if let Self::AProhibited { prohibited_err, .. } = self {
            Some(prohibited_err)
        } else {
            None
        }
    }
    pub fn is_a_version(&self) -> bool {
        matches!(self, Self::AVersion)
    }
    pub fn a_link(&self) -> Option<&String> {
        if let Self::ALink { link_err, .. } = self {
            Some(link_err)
        } else {
            None
        }
    }
    pub fn a_crypto(&self) -> Option<&AgentCryptoError> {
        if let Self::ACrypto { crypto_err, .. } = self {
            Some(crypto_err)
        } else {
            None
        }
    }
    pub fn a_duplicate(&self) -> Option<&Option<DroppedMsg>> {
        if let Self::ADuplicate { dropped_msg, .. } = self {
            Some(dropped_msg)
        } else {
            None
        }
    }
    pub fn a_queue(&self) -> Option<&String> {
        if let Self::AQueue { queue_err, .. } = self {
            Some(queue_err)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "proxy")]
    Proxy {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "proxyRelay")]
    ProxyRelay {
        #[serde(rename = "proxyServer")]
        proxy_server: String,

        #[serde(rename = "srvError")]
        srv_error: SrvError,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "other")]
    Other {
        #[serde(rename = "sndError")]
        snd_error: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SndError {
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth)
    }
    pub fn is_quota(&self) -> bool {
        matches!(self, Self::Quota)
    }
    pub fn is_expired(&self) -> bool {
        matches!(self, Self::Expired)
    }
    pub fn relay(&self) -> Option<&SrvError> {
        if let Self::Relay { srv_error, .. } = self {
            Some(srv_error)
        } else {
            None
        }
    }
    pub fn proxy(&self) -> Option<SndErrorProxyRef<'_>> {
        if let Self::Proxy {
            proxy_server,
            srv_error,
            ..
        } = self
        {
            Some(SndErrorProxyRef {
                proxy_server,
                srv_error,
            })
        } else {
            None
        }
    }
    pub fn proxy_relay(&self) -> Option<SndErrorProxyRelayRef<'_>> {
        if let Self::ProxyRelay {
            proxy_server,
            srv_error,
            ..
        } = self
        {
            Some(SndErrorProxyRelayRef {
                proxy_server,
                srv_error,
            })
        } else {
            None
        }
    }
    pub fn other(&self) -> Option<&String> {
        if let Self::Other { snd_error, .. } = self {
            Some(snd_error)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct SndErrorProxyRef<'a> {
    pub proxy_server: &'a String,
    pub srv_error: &'a SrvError,
}
#[derive(Clone, Copy)]
pub struct SndErrorProxyRelayRef<'a> {
    pub proxy_server: &'a String,
    pub srv_error: &'a SrvError,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl SrvError {
    pub fn is_host(&self) -> bool {
        matches!(self, Self::Host)
    }
    pub fn is_version(&self) -> bool {
        matches!(self, Self::Version)
    }
    pub fn other(&self) -> Option<&String> {
        if let Self::Other { srv_error, .. } = self {
            Some(srv_error)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "relayUserNotFound")]
    RelayUserNotFound,
    #[serde(rename = "userNotFoundByName")]
    UserNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNotFoundByContactId")]
    UserNotFoundByContactId {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNotFoundByGroupId")]
    UserNotFoundByGroupId {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNotFoundByFileId")]
    UserNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNotFoundByContactRequestId")]
    UserNotFoundByContactRequestId {
        #[serde(
            rename = "contactRequestId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_request_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotFound")]
    ContactNotFound {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotFoundByName")]
    ContactNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotFoundByMemberId")]
    ContactNotFoundByMemberId {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotReady")]
    ContactNotReady {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactRequestNotFoundByName")]
    ContactRequestNotFoundByName {
        #[serde(rename = "contactName")]
        contact_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidContactRequestEntity")]
    InvalidContactRequestEntity {
        #[serde(
            rename = "contactRequestId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_request_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupNotFoundByName")]
    GroupNotFoundByName {
        #[serde(rename = "groupName")]
        group_name: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupMemberNotFound")]
    GroupMemberNotFound {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupMemberNotFoundByIndex")]
    GroupMemberNotFoundByIndex {
        #[serde(
            rename = "groupMemberIndex",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_index: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "memberRelationsVectorNotFound")]
    MemberRelationsVectorNotFound {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupHostMemberNotFound")]
    GroupHostMemberNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupMemberNotFoundByMemberId")]
    GroupMemberNotFoundByMemberId {
        #[serde(rename = "memberId")]
        member_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "memberContactGroupMemberNotFound")]
    MemberContactGroupMemberNotFound {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidMemberRelationUpdate")]
    InvalidMemberRelationUpdate,
    #[serde(rename = "groupWithoutUser")]
    GroupWithoutUser,
    #[serde(rename = "duplicateGroupMember")]
    DuplicateGroupMember,
    #[serde(rename = "duplicateMemberId")]
    DuplicateMemberId,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "noteFolderNotFound")]
    NoteFolderNotFound {
        #[serde(
            rename = "noteFolderId",
            deserialize_with = "deserialize_number_from_string"
        )]
        note_folder_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "userNoteFolderNotFound")]
    UserNoteFolderNotFound,
    #[serde(rename = "sndFileNotFound")]
    SndFileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndFileInvalid")]
    SndFileInvalid {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvFileNotFound")]
    RcvFileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvFileDescrNotFound")]
    RcvFileDescrNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileNotFound")]
    FileNotFound {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvFileInvalid")]
    RcvFileInvalid {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvFileInvalidDescrPart")]
    RcvFileInvalidDescrPart,
    #[serde(rename = "localFileNoTransfer")]
    LocalFileNoTransfer {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sharedMsgIdNotFoundByFileId")]
    SharedMsgIdNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "fileIdNotFoundBySharedMsgId")]
    FileIdNotFoundBySharedMsgId {
        #[serde(rename = "sharedMsgId")]
        shared_msg_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "sndFileNotFoundXFTP")]
    SndFileNotFoundXftp {
        #[serde(rename = "agentSndFileId")]
        agent_snd_file_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "rcvFileNotFoundXFTP")]
    RcvFileNotFoundXftp {
        #[serde(rename = "agentRcvFileId")]
        agent_rcv_file_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectionNotFound")]
    ConnectionNotFound {
        #[serde(rename = "agentConnId")]
        agent_conn_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectionNotFoundById")]
    ConnectionNotFoundById {
        #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
        conn_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "connectionNotFoundByMemberId")]
    ConnectionNotFoundByMemberId {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "pendingConnectionNotFound")]
    PendingConnectionNotFound {
        #[serde(rename = "connId", deserialize_with = "deserialize_number_from_string")]
        conn_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "uniqueID")]
    UniqueId,
    #[serde(rename = "largeMsg")]
    LargeMsg,
    #[serde(rename = "internalError")]
    InternalError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "dBException")]
    DBException {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "dBBusyError")]
    DBBusyError {
        #[serde(rename = "message")]
        message: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "badChatItem")]
    BadChatItem {
        #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
        item_id: i64,

        #[serde(rename = "itemTs", skip_serializing_if = "Option::is_none")]
        item_ts: Option<UtcTime>,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemNotFound")]
    ChatItemNotFound {
        #[serde(rename = "itemId", deserialize_with = "deserialize_number_from_string")]
        item_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemNotFoundByText")]
    ChatItemNotFoundByText {
        #[serde(rename = "text")]
        text: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemSharedMsgIdNotFound")]
    ChatItemSharedMsgIdNotFound {
        #[serde(rename = "sharedMsgId")]
        shared_msg_id: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemNotFoundByFileId")]
    ChatItemNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemNotFoundByContactId")]
    ChatItemNotFoundByContactId {
        #[serde(
            rename = "contactId",
            deserialize_with = "deserialize_number_from_string"
        )]
        contact_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "chatItemNotFoundByGroupId")]
    ChatItemNotFoundByGroupId {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "profileNotFound")]
    ProfileNotFound {
        #[serde(
            rename = "profileId",
            deserialize_with = "deserialize_number_from_string"
        )]
        profile_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "duplicateGroupLink")]
    DuplicateGroupLink {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupLinkNotFound")]
    GroupLinkNotFound {
        #[serde(rename = "groupInfo")]
        group_info: GroupInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "hostMemberIdNotFound")]
    HostMemberIdNotFound {
        #[serde(
            rename = "groupId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "contactNotFoundByFileId")]
    ContactNotFoundByFileId {
        #[serde(rename = "fileId", deserialize_with = "deserialize_number_from_string")]
        file_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "remoteHostNotFound")]
    RemoteHostNotFound {
        #[serde(
            rename = "remoteHostId",
            deserialize_with = "deserialize_number_from_string"
        )]
        remote_host_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "operatorNotFound")]
    OperatorNotFound {
        #[serde(
            rename = "serverOperatorId",
            deserialize_with = "deserialize_number_from_string"
        )]
        server_operator_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "usageConditionsNotFound")]
    UsageConditionsNotFound,
    #[serde(rename = "userChatRelayNotFound")]
    UserChatRelayNotFound {
        #[serde(
            rename = "chatRelayId",
            deserialize_with = "deserialize_number_from_string"
        )]
        chat_relay_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupRelayNotFound")]
    GroupRelayNotFound {
        #[serde(
            rename = "groupRelayId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_relay_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "groupRelayNotFoundByMemberId")]
    GroupRelayNotFoundByMemberId {
        #[serde(
            rename = "groupMemberId",
            deserialize_with = "deserialize_number_from_string"
        )]
        group_member_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidQuote")]
    InvalidQuote,
    #[serde(rename = "invalidMention")]
    InvalidMention,
    #[serde(rename = "invalidDeliveryTask")]
    InvalidDeliveryTask {
        #[serde(rename = "taskId", deserialize_with = "deserialize_number_from_string")]
        task_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "deliveryTaskNotFound")]
    DeliveryTaskNotFound {
        #[serde(rename = "taskId", deserialize_with = "deserialize_number_from_string")]
        task_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "invalidDeliveryJob")]
    InvalidDeliveryJob {
        #[serde(rename = "jobId", deserialize_with = "deserialize_number_from_string")]
        job_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "deliveryJobNotFound")]
    DeliveryJobNotFound {
        #[serde(rename = "jobId", deserialize_with = "deserialize_number_from_string")]
        job_id: i64,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "workItemError")]
    WorkItemError {
        #[serde(rename = "errContext")]
        err_context: String,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl StoreError {
    pub fn is_duplicate_name(&self) -> bool {
        matches!(self, Self::DuplicateName)
    }
    pub fn user_not_found(&self) -> Option<&i64> {
        if let Self::UserNotFound { user_id, .. } = self {
            Some(user_id)
        } else {
            None
        }
    }
    pub fn is_relay_user_not_found(&self) -> bool {
        matches!(self, Self::RelayUserNotFound)
    }
    pub fn user_not_found_by_name(&self) -> Option<&String> {
        if let Self::UserNotFoundByName { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn user_not_found_by_contact_id(&self) -> Option<&i64> {
        if let Self::UserNotFoundByContactId { contact_id, .. } = self {
            Some(contact_id)
        } else {
            None
        }
    }
    pub fn user_not_found_by_group_id(&self) -> Option<&i64> {
        if let Self::UserNotFoundByGroupId { group_id, .. } = self {
            Some(group_id)
        } else {
            None
        }
    }
    pub fn user_not_found_by_file_id(&self) -> Option<&i64> {
        if let Self::UserNotFoundByFileId { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn user_not_found_by_contact_request_id(&self) -> Option<&i64> {
        if let Self::UserNotFoundByContactRequestId {
            contact_request_id, ..
        } = self
        {
            Some(contact_request_id)
        } else {
            None
        }
    }
    pub fn contact_not_found(&self) -> Option<&i64> {
        if let Self::ContactNotFound { contact_id, .. } = self {
            Some(contact_id)
        } else {
            None
        }
    }
    pub fn contact_not_found_by_name(&self) -> Option<&String> {
        if let Self::ContactNotFoundByName { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn contact_not_found_by_member_id(&self) -> Option<&i64> {
        if let Self::ContactNotFoundByMemberId {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
    }
    pub fn contact_not_ready(&self) -> Option<&String> {
        if let Self::ContactNotReady { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn is_duplicate_contact_link(&self) -> bool {
        matches!(self, Self::DuplicateContactLink)
    }
    pub fn is_user_contact_link_not_found(&self) -> bool {
        matches!(self, Self::UserContactLinkNotFound)
    }
    pub fn contact_request_not_found(&self) -> Option<&i64> {
        if let Self::ContactRequestNotFound {
            contact_request_id, ..
        } = self
        {
            Some(contact_request_id)
        } else {
            None
        }
    }
    pub fn contact_request_not_found_by_name(&self) -> Option<&String> {
        if let Self::ContactRequestNotFoundByName { contact_name, .. } = self {
            Some(contact_name)
        } else {
            None
        }
    }
    pub fn invalid_contact_request_entity(&self) -> Option<&i64> {
        if let Self::InvalidContactRequestEntity {
            contact_request_id, ..
        } = self
        {
            Some(contact_request_id)
        } else {
            None
        }
    }
    pub fn is_invalid_business_chat_contact_request(&self) -> bool {
        matches!(self, Self::InvalidBusinessChatContactRequest)
    }
    pub fn group_not_found(&self) -> Option<&i64> {
        if let Self::GroupNotFound { group_id, .. } = self {
            Some(group_id)
        } else {
            None
        }
    }
    pub fn group_not_found_by_name(&self) -> Option<&String> {
        if let Self::GroupNotFoundByName { group_name, .. } = self {
            Some(group_name)
        } else {
            None
        }
    }
    pub fn group_member_name_not_found(&self) -> Option<StoreErrorGroupMemberNameNotFoundRef<'_>> {
        if let Self::GroupMemberNameNotFound {
            group_id,
            group_member_name,
            ..
        } = self
        {
            Some(StoreErrorGroupMemberNameNotFoundRef {
                group_id,
                group_member_name,
            })
        } else {
            None
        }
    }
    pub fn group_member_not_found(&self) -> Option<&i64> {
        if let Self::GroupMemberNotFound {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
    }
    pub fn group_member_not_found_by_index(&self) -> Option<&i64> {
        if let Self::GroupMemberNotFoundByIndex {
            group_member_index, ..
        } = self
        {
            Some(group_member_index)
        } else {
            None
        }
    }
    pub fn member_relations_vector_not_found(&self) -> Option<&i64> {
        if let Self::MemberRelationsVectorNotFound {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
    }
    pub fn group_host_member_not_found(&self) -> Option<&i64> {
        if let Self::GroupHostMemberNotFound { group_id, .. } = self {
            Some(group_id)
        } else {
            None
        }
    }
    pub fn group_member_not_found_by_member_id(&self) -> Option<&String> {
        if let Self::GroupMemberNotFoundByMemberId { member_id, .. } = self {
            Some(member_id)
        } else {
            None
        }
    }
    pub fn member_contact_group_member_not_found(&self) -> Option<&i64> {
        if let Self::MemberContactGroupMemberNotFound { contact_id, .. } = self {
            Some(contact_id)
        } else {
            None
        }
    }
    pub fn is_invalid_member_relation_update(&self) -> bool {
        matches!(self, Self::InvalidMemberRelationUpdate)
    }
    pub fn is_group_without_user(&self) -> bool {
        matches!(self, Self::GroupWithoutUser)
    }
    pub fn is_duplicate_group_member(&self) -> bool {
        matches!(self, Self::DuplicateGroupMember)
    }
    pub fn is_duplicate_member_id(&self) -> bool {
        matches!(self, Self::DuplicateMemberId)
    }
    pub fn is_group_already_joined(&self) -> bool {
        matches!(self, Self::GroupAlreadyJoined)
    }
    pub fn is_group_invitation_not_found(&self) -> bool {
        matches!(self, Self::GroupInvitationNotFound)
    }
    pub fn note_folder_already_exists(&self) -> Option<&i64> {
        if let Self::NoteFolderAlreadyExists { note_folder_id, .. } = self {
            Some(note_folder_id)
        } else {
            None
        }
    }
    pub fn note_folder_not_found(&self) -> Option<&i64> {
        if let Self::NoteFolderNotFound { note_folder_id, .. } = self {
            Some(note_folder_id)
        } else {
            None
        }
    }
    pub fn is_user_note_folder_not_found(&self) -> bool {
        matches!(self, Self::UserNoteFolderNotFound)
    }
    pub fn snd_file_not_found(&self) -> Option<&i64> {
        if let Self::SndFileNotFound { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn snd_file_invalid(&self) -> Option<&i64> {
        if let Self::SndFileInvalid { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn rcv_file_not_found(&self) -> Option<&i64> {
        if let Self::RcvFileNotFound { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn rcv_file_descr_not_found(&self) -> Option<&i64> {
        if let Self::RcvFileDescrNotFound { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn file_not_found(&self) -> Option<&i64> {
        if let Self::FileNotFound { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn rcv_file_invalid(&self) -> Option<&i64> {
        if let Self::RcvFileInvalid { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn is_rcv_file_invalid_descr_part(&self) -> bool {
        matches!(self, Self::RcvFileInvalidDescrPart)
    }
    pub fn local_file_no_transfer(&self) -> Option<&i64> {
        if let Self::LocalFileNoTransfer { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn shared_msg_id_not_found_by_file_id(&self) -> Option<&i64> {
        if let Self::SharedMsgIdNotFoundByFileId { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn file_id_not_found_by_shared_msg_id(&self) -> Option<&String> {
        if let Self::FileIdNotFoundBySharedMsgId { shared_msg_id, .. } = self {
            Some(shared_msg_id)
        } else {
            None
        }
    }
    pub fn snd_file_not_found_xftp(&self) -> Option<&String> {
        if let Self::SndFileNotFoundXftp {
            agent_snd_file_id, ..
        } = self
        {
            Some(agent_snd_file_id)
        } else {
            None
        }
    }
    pub fn rcv_file_not_found_xftp(&self) -> Option<&String> {
        if let Self::RcvFileNotFoundXftp {
            agent_rcv_file_id, ..
        } = self
        {
            Some(agent_rcv_file_id)
        } else {
            None
        }
    }
    pub fn connection_not_found(&self) -> Option<&String> {
        if let Self::ConnectionNotFound { agent_conn_id, .. } = self {
            Some(agent_conn_id)
        } else {
            None
        }
    }
    pub fn connection_not_found_by_id(&self) -> Option<&i64> {
        if let Self::ConnectionNotFoundById { conn_id, .. } = self {
            Some(conn_id)
        } else {
            None
        }
    }
    pub fn connection_not_found_by_member_id(&self) -> Option<&i64> {
        if let Self::ConnectionNotFoundByMemberId {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
    }
    pub fn pending_connection_not_found(&self) -> Option<&i64> {
        if let Self::PendingConnectionNotFound { conn_id, .. } = self {
            Some(conn_id)
        } else {
            None
        }
    }
    pub fn is_unique_id(&self) -> bool {
        matches!(self, Self::UniqueId)
    }
    pub fn is_large_msg(&self) -> bool {
        matches!(self, Self::LargeMsg)
    }
    pub fn internal_error(&self) -> Option<&String> {
        if let Self::InternalError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn db_exception(&self) -> Option<&String> {
        if let Self::DBException { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn db_busy_error(&self) -> Option<&String> {
        if let Self::DBBusyError { message, .. } = self {
            Some(message)
        } else {
            None
        }
    }
    pub fn bad_chat_item(&self) -> Option<StoreErrorBadChatItemRef<'_>> {
        if let Self::BadChatItem {
            item_id, item_ts, ..
        } = self
        {
            Some(StoreErrorBadChatItemRef { item_id, item_ts })
        } else {
            None
        }
    }
    pub fn chat_item_not_found(&self) -> Option<&i64> {
        if let Self::ChatItemNotFound { item_id, .. } = self {
            Some(item_id)
        } else {
            None
        }
    }
    pub fn chat_item_not_found_by_text(&self) -> Option<&String> {
        if let Self::ChatItemNotFoundByText { text, .. } = self {
            Some(text)
        } else {
            None
        }
    }
    pub fn chat_item_shared_msg_id_not_found(&self) -> Option<&String> {
        if let Self::ChatItemSharedMsgIdNotFound { shared_msg_id, .. } = self {
            Some(shared_msg_id)
        } else {
            None
        }
    }
    pub fn chat_item_not_found_by_file_id(&self) -> Option<&i64> {
        if let Self::ChatItemNotFoundByFileId { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn chat_item_not_found_by_contact_id(&self) -> Option<&i64> {
        if let Self::ChatItemNotFoundByContactId { contact_id, .. } = self {
            Some(contact_id)
        } else {
            None
        }
    }
    pub fn chat_item_not_found_by_group_id(&self) -> Option<&i64> {
        if let Self::ChatItemNotFoundByGroupId { group_id, .. } = self {
            Some(group_id)
        } else {
            None
        }
    }
    pub fn profile_not_found(&self) -> Option<&i64> {
        if let Self::ProfileNotFound { profile_id, .. } = self {
            Some(profile_id)
        } else {
            None
        }
    }
    pub fn duplicate_group_link(&self) -> Option<&GroupInfo> {
        if let Self::DuplicateGroupLink { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
    pub fn group_link_not_found(&self) -> Option<&GroupInfo> {
        if let Self::GroupLinkNotFound { group_info, .. } = self {
            Some(group_info)
        } else {
            None
        }
    }
    pub fn host_member_id_not_found(&self) -> Option<&i64> {
        if let Self::HostMemberIdNotFound { group_id, .. } = self {
            Some(group_id)
        } else {
            None
        }
    }
    pub fn contact_not_found_by_file_id(&self) -> Option<&i64> {
        if let Self::ContactNotFoundByFileId { file_id, .. } = self {
            Some(file_id)
        } else {
            None
        }
    }
    pub fn no_group_snd_status(&self) -> Option<StoreErrorNoGroupSndStatusRef<'_>> {
        if let Self::NoGroupSndStatus {
            item_id,
            group_member_id,
            ..
        } = self
        {
            Some(StoreErrorNoGroupSndStatusRef {
                item_id,
                group_member_id,
            })
        } else {
            None
        }
    }
    pub fn duplicate_group_message(&self) -> Option<StoreErrorDuplicateGroupMessageRef<'_>> {
        if let Self::DuplicateGroupMessage {
            group_id,
            shared_msg_id,
            author_group_member_id,
            forwarded_by_group_member_id,
            ..
        } = self
        {
            Some(StoreErrorDuplicateGroupMessageRef {
                group_id,
                shared_msg_id,
                author_group_member_id,
                forwarded_by_group_member_id,
            })
        } else {
            None
        }
    }
    pub fn remote_host_not_found(&self) -> Option<&i64> {
        if let Self::RemoteHostNotFound { remote_host_id, .. } = self {
            Some(remote_host_id)
        } else {
            None
        }
    }
    pub fn is_remote_host_unknown(&self) -> bool {
        matches!(self, Self::RemoteHostUnknown)
    }
    pub fn is_remote_host_duplicate_ca(&self) -> bool {
        matches!(self, Self::RemoteHostDuplicateCa)
    }
    pub fn remote_ctrl_not_found(&self) -> Option<&i64> {
        if let Self::RemoteCtrlNotFound { remote_ctrl_id, .. } = self {
            Some(remote_ctrl_id)
        } else {
            None
        }
    }
    pub fn is_remote_ctrl_duplicate_ca(&self) -> bool {
        matches!(self, Self::RemoteCtrlDuplicateCa)
    }
    pub fn prohibited_delete_user(&self) -> Option<StoreErrorProhibitedDeleteUserRef<'_>> {
        if let Self::ProhibitedDeleteUser {
            user_id,
            contact_id,
            ..
        } = self
        {
            Some(StoreErrorProhibitedDeleteUserRef {
                user_id,
                contact_id,
            })
        } else {
            None
        }
    }
    pub fn operator_not_found(&self) -> Option<&i64> {
        if let Self::OperatorNotFound {
            server_operator_id, ..
        } = self
        {
            Some(server_operator_id)
        } else {
            None
        }
    }
    pub fn is_usage_conditions_not_found(&self) -> bool {
        matches!(self, Self::UsageConditionsNotFound)
    }
    pub fn user_chat_relay_not_found(&self) -> Option<&i64> {
        if let Self::UserChatRelayNotFound { chat_relay_id, .. } = self {
            Some(chat_relay_id)
        } else {
            None
        }
    }
    pub fn group_relay_not_found(&self) -> Option<&i64> {
        if let Self::GroupRelayNotFound { group_relay_id, .. } = self {
            Some(group_relay_id)
        } else {
            None
        }
    }
    pub fn group_relay_not_found_by_member_id(&self) -> Option<&i64> {
        if let Self::GroupRelayNotFoundByMemberId {
            group_member_id, ..
        } = self
        {
            Some(group_member_id)
        } else {
            None
        }
    }
    pub fn is_invalid_quote(&self) -> bool {
        matches!(self, Self::InvalidQuote)
    }
    pub fn is_invalid_mention(&self) -> bool {
        matches!(self, Self::InvalidMention)
    }
    pub fn invalid_delivery_task(&self) -> Option<&i64> {
        if let Self::InvalidDeliveryTask { task_id, .. } = self {
            Some(task_id)
        } else {
            None
        }
    }
    pub fn delivery_task_not_found(&self) -> Option<&i64> {
        if let Self::DeliveryTaskNotFound { task_id, .. } = self {
            Some(task_id)
        } else {
            None
        }
    }
    pub fn invalid_delivery_job(&self) -> Option<&i64> {
        if let Self::InvalidDeliveryJob { job_id, .. } = self {
            Some(job_id)
        } else {
            None
        }
    }
    pub fn delivery_job_not_found(&self) -> Option<&i64> {
        if let Self::DeliveryJobNotFound { job_id, .. } = self {
            Some(job_id)
        } else {
            None
        }
    }
    pub fn work_item_error(&self) -> Option<&String> {
        if let Self::WorkItemError { err_context, .. } = self {
            Some(err_context)
        } else {
            None
        }
    }
}
#[derive(Clone, Copy)]
pub struct StoreErrorGroupMemberNameNotFoundRef<'a> {
    pub group_id: &'a i64,
    pub group_member_name: &'a String,
}
#[derive(Clone, Copy)]
pub struct StoreErrorBadChatItemRef<'a> {
    pub item_id: &'a i64,
    pub item_ts: &'a Option<UtcTime>,
}
#[derive(Clone, Copy)]
pub struct StoreErrorNoGroupSndStatusRef<'a> {
    pub item_id: &'a i64,
    pub group_member_id: &'a i64,
}
#[derive(Clone, Copy)]
pub struct StoreErrorDuplicateGroupMessageRef<'a> {
    pub group_id: &'a i64,
    pub shared_msg_id: &'a String,
    pub author_group_member_id: &'a Option<i64>,
    pub forwarded_by_group_member_id: &'a Option<i64>,
}
#[derive(Clone, Copy)]
pub struct StoreErrorProhibitedDeleteUserRef<'a> {
    pub user_id: &'a i64,
    pub contact_id: &'a i64,
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl TransportError {
    pub fn is_bad_block(&self) -> bool {
        matches!(self, Self::BadBlock)
    }
    pub fn is_version(&self) -> bool {
        matches!(self, Self::Version)
    }
    pub fn is_large_msg(&self) -> bool {
        matches!(self, Self::LargeMsg)
    }
    pub fn is_bad_session(&self) -> bool {
        matches!(self, Self::BadSession)
    }
    pub fn is_no_server_auth(&self) -> bool {
        matches!(self, Self::NoServerAuth)
    }
    pub fn handshake(&self) -> Option<&HandshakeError> {
        if let Self::Handshake { handshake_err, .. } = self {
            Some(handshake_err)
        } else {
            None
        }
    }
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

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
    },
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BLOCKED")]
    Blocked {
        #[serde(rename = "blockInfo")]
        block_info: BlockingInfo,

        #[serde(flatten, skip_serializing_if = "JsonObject::is_null")]
        undocumented: JsonObject,
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

impl XFTPErrorType {
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block)
    }
    pub fn is_session(&self) -> bool {
        matches!(self, Self::Session)
    }
    pub fn is_handshake(&self) -> bool {
        matches!(self, Self::Handshake)
    }
    pub fn cmd(&self) -> Option<&CommandError> {
        if let Self::Cmd { cmd_err, .. } = self {
            Some(cmd_err)
        } else {
            None
        }
    }
    pub fn is_auth(&self) -> bool {
        matches!(self, Self::Auth)
    }
    pub fn blocked(&self) -> Option<&BlockingInfo> {
        if let Self::Blocked { block_info, .. } = self {
            Some(block_info)
        } else {
            None
        }
    }
    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size)
    }
    pub fn is_quota(&self) -> bool {
        matches!(self, Self::Quota)
    }
    pub fn is_digest(&self) -> bool {
        matches!(self, Self::Digest)
    }
    pub fn is_crypto(&self) -> bool {
        matches!(self, Self::Crypto)
    }
    pub fn is_no_file(&self) -> bool {
        matches!(self, Self::NoFile)
    }
    pub fn is_has_file(&self) -> bool {
        matches!(self, Self::HasFile)
    }
    pub fn is_file_io(&self) -> bool {
        matches!(self, Self::FileIo)
    }
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
    pub fn is_internal(&self) -> bool {
        matches!(self, Self::Internal)
    }
    pub fn is_duplicate(&self) -> bool {
        matches!(self, Self::Duplicate)
    }
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
    RcvMsgError,
    SMPAgentError,
    SndError,
    SrvError,
    StoreError,
    TransportError,
    XFTPErrorType
);
