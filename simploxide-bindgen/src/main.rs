use std::process::ExitCode;
use std::{collections::BTreeMap, error::Error};
use std::{collections::btree_map::Entry, io::Write as _};

use simploxide_bindgen::{
    commands::{self, CommandFmt, CommandResponse, CommandResponseTraitMethod, ResponseWrapperFmt},
    events,
    syntax::Interpretable,
    types::{
        self, ApiType, DiscriminatedUnionType, DisjointedDiscriminatedUnion, Field, RecordType,
        discriminated_union_type::DiscriminatedUnionVariant,
    },
};

const COMMANDS_MD: &str = "./simplex-chat/bots/api/COMMANDS.md";
const TYPES_MD: &str = "./simplex-chat/bots/api/TYPES.md";
const EVENTS_MD: &str = "./simplex-chat/bots/api/EVENTS.md";

const LIB_RS: &str = "./generated/lib.rs";
const ERRORS_RS: &str = "./generated/errors.rs";
const EVENTS_RS: &str = "./generated/events.rs";
const COMMANDS_RS: &str = "./generated/commands.rs";
const RESPONSES_RS: &str = "./generated/responses.rs";
const UTILS_RS: &str = "./generated/utils.rs";
const CLIENT_API_RS: &str = "./generated/client_api.rs";

fn main() -> ExitCode {
    force_create_dir("./generated").unwrap();
    let types_md = std::fs::read_to_string(TYPES_MD).unwrap();
    let events_md = std::fs::read_to_string(EVENTS_MD).unwrap();
    let commands_md = std::fs::read_to_string(COMMANDS_MD).unwrap();

    if let Err(e) = generate_types(&types_md) {
        eprintln!("Failed to bind TYPES.md\nError: {}", e);
        return ExitCode::from(1);
    }

    if let Err(e) = generate_events(&events_md) {
        eprintln!("Failed to bind EVENTS.md\nError: {}", e);
        return ExitCode::from(1);
    }

    if let Err(e) = generate_commands(&commands_md) {
        eprintln!("Failed to bind COMMANDS.md\nError: {}", e);
        return ExitCode::from(1);
    }

    if let Err(e) = generate_utils() {
        eprintln!("Failed to generate utils.rs\nError: {}", e);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn generate_types(types_md: &str) -> Result<(), Box<dyn Error>> {
    let mut lib_rs = std::fs::File::create(LIB_RS)?;
    let mut errors_rs = std::fs::File::create(ERRORS_RS)?;

    writeln!(lib_rs, "//! This crate is auto-generated\n")?;
    writeln!(lib_rs, "#![allow(clippy::large_enum_variant)]")?;
    writeln!(lib_rs, "#![allow(clippy::should_implement_trait)]")?;
    writeln!(lib_rs, "#![allow(clippy::unnecessary_to_owned)]")?;
    writeln!(lib_rs)?;
    writeln!(lib_rs, "pub mod errors;")?;
    writeln!(lib_rs, "pub mod events;")?;
    writeln!(lib_rs, "pub mod commands;")?;
    writeln!(lib_rs, "pub mod utils;")?;
    writeln!(lib_rs, "pub mod responses;")?;
    writeln!(lib_rs, "pub mod client_api;")?;
    writeln!(lib_rs)?;
    writeln!(lib_rs, "use serde::{{Serialize, Deserialize}};")?;
    writeln!(
        lib_rs,
        "use serde_aux::field_attributes::{{deserialize_number_from_string, deserialize_option_number_from_string}};"
    )?;
    writeln!(
        lib_rs,
        "use std::{{collections::{{HashMap, BTreeMap}}, sync::Arc}};"
    )?;
    writeln!(lib_rs, "use errors::*;")?;
    writeln!(lib_rs, "use utils::CommandSyntax;")?;
    writeln!(lib_rs)?;
    writeln!(lib_rs, "pub type UtcTime = String;")?;
    writeln!(lib_rs, "pub type JsonObject = serde_json::Value;")?;
    writeln!(lib_rs)?;

    writeln!(errors_rs, "use super::*;")?;
    writeln!(errors_rs)?;

    let mut error_types = Vec::with_capacity(32);

    for api_type in types::parse(types_md) {
        let mut api_type = api_type?;

        let mut syntax = None;
        if let ApiType::DiscriminatedUnion(ref mut t) = api_type {
            if t.name == "ErrorType" {
                for var in &mut t.variants {
                    for field in &mut var.fields {
                        if field.typ == "ProxyError" {
                            field.typ = format!("Arc<{}>", field.typ)
                        }
                    }
                }
            }

            syntax = hack_discriminated_union_syntax(t)
        }

        if api_type.is_error() {
            writeln!(errors_rs, "{}", api_type)?;
            error_types.push(api_type.name().to_owned());
        } else {
            writeln!(lib_rs, "{}", api_type)?;
            if let Some(syntax) = syntax.or(api_type.command_syntax_impl()?) {
                writeln!(lib_rs, "{syntax}\n")?;
            }
        }
    }

    writeln!(
        errors_rs,
        r##"
macro_rules! impl_error {{
    ($($t:ty),+ $(,)?) => (
        $(
        impl std::fmt::Display for $t {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(f, "{{:#?}}", self)
            }}
        }}

        impl std::error::Error for $t {{}}
        )+
    );
}}
"##
    )?;

    writeln!(errors_rs, "impl_error!({});", error_types.join(", "))?;

    Ok(())
}

fn generate_events(events_md: &str) -> Result<(), Box<dyn Error>> {
    let discriminated_records: DisjointedDiscriminatedUnion = events::parse(events_md)
        .map(|result| {
            result.map(|mut part| {
                if part.record.is_error() {
                    // Use errors from `errors` module for nested error types
                    //
                    // FIXME: The markdown type links should be properly parsed and resolved
                    // someday in the future.
                    for field in &mut part.record.fields {
                        if let Some(ix) = field.typ.find("Error") {
                            let typ_start = field.typ[..ix]
                                .char_indices()
                                .rev()
                                .take_while(|(_, ch)| ch.is_alphanumeric())
                                .map(|(ix, _)| ix)
                                .last()
                                .unwrap_or(0);

                            field.typ.insert_str(typ_start, "errors::");
                        }
                    }
                }

                part
            })
        })
        .collect::<Result<_, _>>()?;

    let mut events_rs = std::fs::File::create(EVENTS_RS)?;
    let (top_level_enum, records) = discriminated_records.into_types("Event".to_owned());

    writeln!(events_rs, "use crate::{{*, errors::*}};")?;
    writeln!(events_rs)?;
    writeln!(events_rs, "{top_level_enum}\n")?;

    for record in records {
        writeln!(events_rs, "{record}\n")?;
    }

    Ok(())
}

fn generate_commands(commands_md: &str) -> Result<(), Box<dyn Error>> {
    let mut commands_rs = std::fs::File::create(COMMANDS_RS)?;
    let mut responses_rs = std::fs::File::create(RESPONSES_RS)?;
    let mut client_api_rs = std::fs::File::create(CLIENT_API_RS)?;

    writeln!(commands_rs, "use super::*;")?;
    writeln!(commands_rs, "use crate::utils::CommandSyntax;")?;
    writeln!(commands_rs)?;

    writeln!(responses_rs, "use super::{{*, errors::*}};")?;
    writeln!(responses_rs)?;

    writeln!(
        client_api_rs,
        "use crate::{{*, responses::*, commands::*, utils::CommandSyntax}};"
    )?;
    writeln!(client_api_rs, "use std::future::Future;")?;
    writeln!(client_api_rs, "use std::sync::Arc;")?;
    writeln!(client_api_rs)?;
    writeln!(client_api_rs, "{}\n", CLIENT_API_ERROR_TRAIT)?;

    writeln!(client_api_rs, "pub trait ClientApi: Sync {{")?;
    writeln!(client_api_rs, "    type Error: ClientApiError;")?;
    writeln!(client_api_rs)?;
    writeln!(
        client_api_rs,
        "    fn send_raw(&self, command: String) -> impl Future<Output = Result<JsonObject, Self::Error>> + Send;"
    )?;
    writeln!(client_api_rs)?;

    let mut unique_response_shapes: BTreeMap<String, RecordType> = BTreeMap::new();
    let mut helper_api_wrappers: Vec<ResponseWrapperFmt> = Vec::with_capacity(40);

    let chat_cmd_error = DiscriminatedUnionVariant::from_api_name(
        "chatCmdError".to_owned(),
        vec![Field::from_api_name(
            "chatError".to_owned(),
            "ChatError".to_owned(),
        )],
    );

    for command_response in commands::parse(commands_md) {
        // ========== Process command ==============
        let CommandResponse {
            command,
            mut response,
        } = command_response?;

        writeln!(commands_rs, "{}\n", CommandFmt(&command))?;

        let syntax_interpreter = command
            .command_syntax_impl()?
            .ok_or_else(|| format!("The command {} doesn't have a syntax", command.name))?;

        writeln!(commands_rs, "{syntax_interpreter}\n")?;

        // ========== Process response ==============
        if !response
            .variants
            .iter()
            .any(|v| v.api_name == chat_cmd_error.api_name)
        {
            response.variants.push(chat_cmd_error.clone());
        }

        let (mut response, shapes) = response.disjoin();

        for mut shape in shapes {
            shape.name.push_str("Response");
            match unique_response_shapes.entry(shape.name.clone()) {
                Entry::Occupied(occupied) => {
                    if *occupied.get() != shape {
                        return Err(format!(
                            "Found a response with the same type name but different shape\nOld: {}\nNew: {}",
                            shape,
                            occupied.get()
                        ).into());
                    }
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(shape);
                }
            }
        }

        for var in response.variants.iter_mut() {
            var.fields[0].typ.push_str("Response");
        }

        writeln!(responses_rs, "{response}\n")?;

        // ========== Process trait method ==============
        let method = CommandResponseTraitMethod::new(&command, &response);
        writeln!(client_api_rs, "{method}\n")?;

        if let Some(wrapper) = method.response_wrapper() {
            helper_api_wrappers.push(wrapper);
        }
    }

    // ========== Process response structs ==============
    for record in unique_response_shapes.into_values() {
        writeln!(responses_rs, "{record}")?;
    }

    // ========== Process helper API types ==============
    writeln!(client_api_rs, "}}\n")?;

    for wrapper in helper_api_wrappers {
        writeln!(client_api_rs, "{wrapper}")?;
    }

    writeln!(client_api_rs, "{}", BAD_RESPONSE_SHENINGANS)?;

    Ok(())
}

fn generate_utils() -> Result<(), Box<dyn Error>> {
    let mut utils_rs = std::fs::File::create(UTILS_RS)?;
    writeln!(utils_rs, "{}", COMMAND_SYNTAX_TRAIT)?;
    Ok(())
}

/// DANGEROUS: Creates a dir at path vanishing all its contents if dir already existed.
fn force_create_dir<P: AsRef<std::path::Path>>(path: P) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }

    std::fs::create_dir_all(path)
}

/// TODO: Right now there are only 2 discriminated unions with the syntax support so the generic
/// discriminated union interpreter generator is not yet implemented and precoded impls are being
/// used instead. This should be replaced with a proper interpreter generation in the future
fn hack_discriminated_union_syntax(du: &DiscriminatedUnionType) -> Option<String> {
    if du.name == "GroupChatScope" {
        Some(
            r#"
impl CommandSyntax for GroupChatScope {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        buf.push_str("(_support");
        match self {
            Self::MemberSupport { group_member_id, .. } => {
                if let Some(group_member_id) = group_member_id {
                    buf.push(':');
                    buf.push_str(&group_member_id.to_string());
                }
            }
            Self::Undocumented(_) => {}
        }
        buf.push(')');
        buf
    }
}
"#
            .to_owned(),
        )
    } else if du.name == "ChatDeleteMode" {
        Some(
            r#"
impl CommandSyntax for ChatDeleteMode {
    fn interpret(&self) -> String {
        let mut buf = String::with_capacity(64);
        match self {
            Self::Full { notify, .. } => {
                buf.push_str("full");
                if !notify {
                    buf.push_str(" notify=off");
                }
            },
            Self::Entity { notify, .. } => {
                buf.push_str("entity");
                if !notify {
                    buf.push_str(" notify=off");
                }
            }
            Self::Messages | Self::Undocumented(_) => {}
        }
        buf
    }
}
"#
            .to_owned(),
        )
    } else {
        None
    }
}

const COMMAND_SYNTAX_TRAIT: &str = r#"
pub trait CommandSyntax {
    /// Generate a SimpleX command string from self
    fn interpret(&self) -> String;
}

// TODO: This is a workaround for some syntaxes that don't use optional values in square brackets.
impl<T: CommandSyntax> CommandSyntax for Option<T> {
    fn interpret(&self) -> String {
        match self {
            Some(c) => c.interpret(),
            None => String::new(),
        }
    }
}
"#;

const BAD_RESPONSE_SHENINGANS: &str = r#"
#[derive(Debug)]
pub enum BadResponseError {
    ChatCmdError(Arc<ChatCmdErrorResponse>),
    Undocumented(BTreeMap<String, JsonObject>),
}

impl std::error::Error for BadResponseError {}

impl std::fmt::Display for BadResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatCmdError(resp) => writeln!(
                f,
                "Bad server response:\n{}",
                serde_json::to_string_pretty(resp).unwrap()
            ),
            Self::Undocumented(resp) => writeln!(
                f,
                "Unexpected server response:\n{}",
                serde_json::to_string_pretty(resp).unwrap()
            ),
        }
    }
}

pub enum UndocumentedResponse<T> {
    Documented(T),
    Undocumented(BTreeMap<String, JsonObject>),
}

/// If you want to ~~suffer~~ handle undocumented responses you can use this extension trait
/// on client API return values which moves Undocumented from `Err` to `Ok` variant.
///
/// Example:
///
/// ```ignore
///     match client
///         .api_create_my_address(1)
///         .await
///         .allow_undocumented()?
///     {
///         UndocumentedResponse::Documented(resp) => {
///              // Process expected response...
///         }
///         UndocumentedResponse::Undocumented(resp) => {
///             // Do something with the unexpected response...
///         }
///     }
/// }
/// ```
pub trait AllowUndocumentedResponses<T, E> {
    fn allow_undocumented(self) -> Result<UndocumentedResponse<T>, E>;
}

impl<T, E> AllowUndocumentedResponses<T, E> for Result<T, E>
where
    E: ClientApiError,
{
    fn allow_undocumented(self) -> Result<UndocumentedResponse<T>, E> {
        match self {
            Ok(resp) => Ok(UndocumentedResponse::Documented(resp)),
            Err(mut e) => match e.bad_response_mut() {
                Some(BadResponseError::Undocumented(btree_map)) => Ok(
                    UndocumentedResponse::Undocumented(std::mem::take(btree_map)),
                ),
                _ => Err(e),
            },
        }
    }
}
"#;

const CLIENT_API_ERROR_TRAIT: &str = r#"
pub trait ClientApiError: From<BadResponseError> + std::error::Error {
    /// If current error is a bad response error return a mut reference to it!
    ///
    /// Required for [`AllowUndocumentedResponses`] impl.
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError>;
}
"#;
