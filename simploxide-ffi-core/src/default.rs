pub const MAX_COMMAND_LEN: usize = 64 * 1024;

pub const MAX_CMDS_PER_ITER: usize = 4;

pub const MAX_EVENTS_PER_ITER: usize = 8;

pub const MAX_EVENT_LATENCY: std::time::Duration = std::time::Duration::from_secs(1);

// Maximum number of concurrent chat_ctrl instances.
pub const MAX_CHAT_INSTANCES: usize = 20;

pub const MAX_DISPLAY_NAME_LEN: usize = 80;

pub const MAX_DB_PREFIX_LEN: usize = 256;
