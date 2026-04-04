use serde::Deserialize;

use std::{
    ffi::{CStr, CString, NulError, c_char, c_int, c_void},
    sync::Once,
};

/// TODO: expose more methods on demand
#[allow(unused)]
#[allow(non_camel_case_types)]
mod bindings;

static HASKELL_RUNTIME: Once = Once::new();

type Handle = bindings::chat_ctrl;

pub struct SimpleXChat(Handle);

impl SimpleXChat {
    pub fn init(
        db_path: String,
        db_key: String,
        migration: MigrationConfirmation,
    ) -> Result<Self, InitError> {
        HASKELL_RUNTIME.call_once(haskell_init);

        let mut handle: Handle = std::ptr::null_mut();
        let db_path = CString::new(db_path).map_err(CallError::NullByteInput)?;
        let db_key = CString::new(db_key).map_err(CallError::NullByteInput)?;
        let string = Self::init_raw(&db_path, &db_key, migration.as_cstr(), &mut handle)?;

        #[derive(Deserialize)]
        struct Response<'a> {
            #[serde(borrow, rename = "type")]
            type_: &'a str,
        }

        let response: Response<'_> =
            serde_json::from_str(&string).map_err(CallError::InvalidJson)?;

        if response.type_ == "ok" {
            Ok(Self(handle))
        } else {
            let error = serde_json::from_str(&string).map_err(CallError::InvalidJson)?;
            Err(InitError::DbError(error))
        }
    }

    pub fn send_cmd(&mut self, cmd: String) -> Result<String, CallError> {
        let ccmd = CString::new(cmd)?;
        let mut c_res = unsafe { bindings::chat_send_cmd(self.0, ccmd.as_ptr()) };
        drop(ccmd);
        c_res_to_string(&mut c_res)
    }

    pub fn recv_msg_wait(&mut self, wait: std::time::Duration) -> Result<String, CallError> {
        let clamped = std::cmp::min(wait, std::time::Duration::from_mins(30));

        // SAFETY: clamped to fit into i32 without overflows
        let cwait: c_int = clamped.as_micros() as i32;
        let mut c_res = unsafe { bindings::chat_recv_msg_wait(self.0, cwait) };

        c_res_to_string(&mut c_res)
    }

    fn init_raw(
        db_path: &CStr,
        db_key: &CStr,
        migration: &'static CStr,
        handle: &mut Handle,
    ) -> Result<String, CallError> {
        let mut c_res = unsafe {
            bindings::chat_migrate_init(
                db_path.as_ptr(),
                db_key.as_ptr(),
                migration.as_ptr(),
                handle,
            )
        };

        c_res_to_string(&mut c_res)
    }
}

impl Drop for SimpleXChat {
    fn drop(&mut self) {
        unsafe {
            bindings::chat_close_store(self.0);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MigrationConfirmation {
    YesUp,
    YesUpDown,
    Console,
    Error,
}

impl MigrationConfirmation {
    fn as_cstr(&self) -> &'static CStr {
        match self {
            Self::YesUp => c"yesUp",
            Self::YesUpDown => c"yesUpDown",
            Self::Console => c"console",
            Self::Error => c"error",
        }
    }
}

fn haskell_init() {
    #[cfg(target_os = "windows")]
    let args = Box::new([
        c"simplex".as_ptr() as *mut c_char,
        c"+RTS".as_ptr() as *mut c_char,
        c"-A64m".as_ptr() as *mut c_char,
        c"-H64m".as_ptr() as *mut c_char,
        c"--install-signal-handlers=no".as_ptr() as *mut c_char,
        std::ptr::null_mut(),
    ]);

    #[cfg(not(target_os = "windows"))]
    let args = Box::new([
        c"simplex".as_ptr() as *mut c_char,
        c"+RTS".as_ptr() as *mut c_char,
        c"-A64m".as_ptr() as *mut c_char,
        c"-H64m".as_ptr() as *mut c_char,
        c"-xn".as_ptr() as *mut c_char,
        c"--install-signal-handlers=no".as_ptr() as *mut c_char,
        std::ptr::null_mut(),
    ]);

    let mut argc: c_int = (args.len() - 1) as c_int;
    let mut pargv: *mut *mut c_char = Box::leak(args).as_mut_ptr();

    unsafe {
        bindings::hs_init_with_rtsopts(&mut argc, &mut pargv);
    }
}

fn c_res_to_string(c_res: &mut *mut c_char) -> Result<String, CallError> {
    fn try_parse_c_res(c_res: *mut c_char) -> Result<String, CallError> {
        if c_res.is_null() {
            return Err(CallError::Failure);
        }

        // SAFETY:
        // * SimpleX-Core-FFI functions should return valid null-terminated C strings
        // * c_res ptr is not null(checked above)
        // * c_res memory is not mutating and is hold exclusively while CStr::from_ptr borrow is
        //   active(ensured by &mut in the outer method)
        let string = unsafe { CStr::from_ptr(c_res).to_str()?.to_owned() };
        Ok(string)
    }

    let parsed = try_parse_c_res(*c_res);

    unsafe {
        libc::free(*c_res as *mut c_void);
    }
    *c_res = std::ptr::null_mut();

    parsed
}

#[derive(Debug)]
pub enum InitError {
    CallError(CallError),
    DbError(serde_json::Value),
}

impl From<CallError> for InitError {
    fn from(value: CallError) -> Self {
        Self::CallError(value)
    }
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitError::CallError(call_error) => call_error.fmt(f),
            InitError::DbError(value) => {
                write!(f, "cannot create DB connection:\n{value:#}")
            }
        }
    }
}

impl std::error::Error for InitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CallError(call_error) => Some(call_error),
            Self::DbError(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum CallError {
    NullByteInput(NulError),
    Failure,
    NotUtf8(std::str::Utf8Error),
    InvalidJson(serde_json::Error),
}

impl From<NulError> for CallError {
    fn from(value: NulError) -> Self {
        Self::NullByteInput(value)
    }
}

impl From<std::str::Utf8Error> for CallError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::NotUtf8(value)
    }
}

impl From<serde_json::Error> for CallError {
    fn from(value: serde_json::Error) -> Self {
        Self::InvalidJson(value)
    }
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::NullByteInput(error) => {
                write!(f, "null byte injection in one of the input strings {error}")
            }
            CallError::Failure => {
                write!(f, "ffi call returned nullptr instead of string")
            }
            CallError::NotUtf8(utf8_error) => {
                write!(f, "ffi call returned non-utf8 string {utf8_error}")
            }
            CallError::InvalidJson(serde_error) => {
                write!(f, "ffi call returned invalid JSON {serde_error}")
            }
        }
    }
}

impl std::error::Error for CallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CallError::NullByteInput(error) => Some(error),
            CallError::Failure => None,
            CallError::NotUtf8(error) => Some(error),
            CallError::InvalidJson(error) => Some(error),
        }
    }
}
