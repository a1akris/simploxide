use std::process::ExitCode;
use std::{collections::BTreeMap, error::Error};
use std::{collections::btree_map::Entry, io::Write as _};

use convert_case::{Case, Casing};
use simploxide_bindgen::{
    commands::{self, CommandFmt, CommandResponse, CommandResponseTraitMethod, ResponseFmt},
    events,
    syntax::Interpretable,
    types::{
        self, ApiType, DiscriminatedUnionType, DisjointedDiscriminatedUnion, Field, RecordType,
        Source, discriminated_union_type::DiscriminatedUnionVariant,
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
    writeln!(lib_rs, "#![allow(clippy::new_without_default)]")?;
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
        "use std::{{collections::BTreeMap, sync::Arc, fmt::Write as _}};"
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

            if let ApiType::DiscriminatedUnion(ref x) = api_type {
                writeln!(lib_rs, "{}", DiscriminatedUnionConstructors(x))?;
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
                for field in &mut part.record.fields {
                    if field.source == Some(Source::Types) {
                        let prefix = if field.is_error() {
                            "errors::"
                        } else {
                            "crate::"
                        };
                        field.typ.insert_str(field.base_type_offset(), prefix);
                    }
                }

                part
            })
        })
        .collect::<Result<_, _>>()?;

    let mut events_rs = std::fs::File::create(EVENTS_RS)?;
    let (mut top_level_enum, records) = discriminated_records.into_types("Event".to_owned());

    for field in top_level_enum
        .variants
        .iter_mut()
        .flat_map(|v| v.fields.iter_mut())
    {
        field.typ = format!("Arc<{}>", field.typ);
    }

    writeln!(events_rs, "use crate::*;")?;
    // Needed to resolve intra-doc links to command types in doc comments
    writeln!(events_rs, "#[allow(unused_imports)]")?;
    writeln!(events_rs, "use crate::commands::*;")?;
    writeln!(events_rs)?;

    writeln!(events_rs, "{top_level_enum}\n")?;
    writeln!(events_rs, "{}\n", EventKindFmt(&top_level_enum))?;

    writeln!(events_rs, "{EVENT_TRAITS}")?;
    for record in records {
        writeln!(events_rs, "{record}\n")?;
        writeln!(events_rs, "{}\n", EventRecordTraitsFmt(&record))?;
    }

    Ok(())
}

fn generate_commands(commands_md: &str) -> Result<(), Box<dyn Error>> {
    let mut commands_rs = std::fs::File::create(COMMANDS_RS)?;
    let mut responses_rs = std::fs::File::create(RESPONSES_RS)?;
    let mut client_api_rs = std::fs::File::create(CLIENT_API_RS)?;

    writeln!(
        commands_rs,
        "use {{crate::*, crate::utils::CommandSyntax}};"
    )?;
    writeln!(commands_rs)?;

    writeln!(responses_rs, "use crate::*;")?;
    writeln!(responses_rs)?;

    writeln!(
        client_api_rs,
        "use crate::{{*, responses::*, commands::*, utils::CommandSyntax}};"
    )?;
    writeln!(client_api_rs, "use std::future::Future;")?;
    writeln!(client_api_rs, "use std::sync::Arc;")?;
    writeln!(commands_rs, "use std::fmt::Write;")?;
    writeln!(client_api_rs)?;
    writeln!(client_api_rs, "{}\n", CLIENT_API_TRAITS)?;

    writeln!(client_api_rs, "pub trait ClientApi: Sync {{")?;
    writeln!(
        client_api_rs,
        "    type ResponseShape<'de, T>: ExtractResponse<'de, T> where T: 'de + Deserialize<'de>;"
    )?;
    writeln!(client_api_rs, "    type Error: ClientApiError;")?;
    writeln!(client_api_rs)?;
    writeln!(
        client_api_rs,
        "    fn send_raw(&self, command: String) -> impl Future<Output = Result<String, Self::Error>> + Send;"
    )?;
    writeln!(client_api_rs)?;

    let mut unique_response_shapes: BTreeMap<String, RecordType> = BTreeMap::new();

    let chat_cmd_error = DiscriminatedUnionVariant::from_api_name(
        "chatCmdError".to_owned(),
        vec![Field::from_api_name(
            "chatError".to_owned(),
            "Arc<errors::ChatError>".to_owned(),
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
        response
            .variants
            .retain(|v| v.api_name != chat_cmd_error.api_name);

        let (mut response, shapes) = response.disjoin();

        for mut shape in shapes.iter().cloned() {
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

        writeln!(responses_rs, "{}\n", ResponseFmt(&response))?;

        // ========== Process trait method ==============
        let method = CommandResponseTraitMethod::new(&command, &response, &shapes);
        writeln!(client_api_rs, "{method}\n")?;
    }

    // ========== Process chat cmd error==============
    let mut error_response =
        DiscriminatedUnionType::new("ChatCmdError".to_owned(), vec![chat_cmd_error]);

    for var in error_response.variants.iter_mut() {
        var.rust_name.push_str("Response");
    }

    let (response, shapes) = error_response.disjoin();

    writeln!(responses_rs, "{}\n", ResponseFmt(&response))?;

    for shape in shapes {
        writeln!(responses_rs, "{shape}")?;
    }

    // ========== Process response structs ==============
    for record in unique_response_shapes.into_values() {
        writeln!(responses_rs, "{record}")?;
    }

    // ========== Process helper API types ==============
    writeln!(client_api_rs, "}}\n")?;

    writeln!(client_api_rs, "{}", RESPONSE_EXTRACTORS)?;

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
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
        buf.push_str("(_support");
        match self {
            Self::MemberSupport { group_member_id, .. } => {
                if let Some(group_member_id) = group_member_id {
                    buf.push(':');
                    write!(buf, "{}", group_member_id).unwrap();
                }
            }
            Self::Undocumented(_) => {}
        }
        buf.push(')');
    }
}
"#
            .to_owned(),
        )
    } else if du.name == "ChatDeleteMode" {
        Some(
            r#"
impl CommandSyntax for ChatDeleteMode {
    const COMMAND_BUF_SIZE: usize = 64;

    fn append_command_syntax(&self, buf: &mut String) {
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
    const COMMAND_BUF_SIZE: usize;

    /// Generate a SimpleX command string from self
    fn to_command_string(&self) -> String {
        let mut buf = String::with_capacity(Self::COMMAND_BUF_SIZE);
        self.append_command_syntax(&mut buf);
        buf
    }

    fn append_command_syntax(&self, buf: &mut String);
}

// TODO: This is a workaround for some syntaxes that don't use optional values in square brackets.
impl<T: CommandSyntax> CommandSyntax for Option<T> {
    const COMMAND_BUF_SIZE: usize = T::COMMAND_BUF_SIZE;

    fn append_command_syntax(&self, buf: &mut String) {
        if let Some(command) = self {
            command.append_command_syntax(buf);
        }
    }
}
"#;

const EVENT_TRAITS: &str = r#"
    /// Generalization of event data
    pub trait EventData {
        const KIND: EventKind;

        fn from_event(event: Event) -> Result<Arc<Self>, Event>;

        fn into_event(self: Arc<Self>) -> Event;
    }
"#;

const CLIENT_API_TRAITS: &str = r#"
/// A helper trait to handle different response wrappers
pub trait ExtractResponse<'de, T>: Deserialize<'de> {
    fn extract_response(self) -> Result<T, BadResponseError>;
}


pub trait ClientApiError: From<BadResponseError> + std::error::Error {
    /// If current error is a bad response error return a mut reference to it!
    ///
    /// Required for [`AllowUndocumentedResponses`] impl.
    fn bad_response_mut(&mut self) -> Option<&mut BadResponseError>;
}
"#;

const RESPONSE_EXTRACTORS: &str = r#"
/// Use this as [`ClientApi::ResponseShape`] to extract web socket responses
#[derive(Serialize, Deserialize)]
pub struct WebSocketResponseShape<T> {
    pub resp: WebSocketResponseShapeInner<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketResponseShapeInner<T> {
    Response(T),
    Error(ChatCmdError),
    Undocumented(JsonObject),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for WebSocketResponseShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        self.resp.extract_response()
    }
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for WebSocketResponseShapeInner<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::Response(resp) => Ok(resp),
            Self::Error(err) => Err(BadResponseError::ChatError(
                err.into_inner().chat_error.clone(),
            )),
            Self::Undocumented(json) => Err(BadResponseError::Undocumented(json)),
        }
    }
}


/// Use this as [`ClientApi::ResponseShape`] to extract FFI responses
#[derive(Serialize, Deserialize)]
pub enum FfiResponseShape<T> {
    #[serde(rename = "result")]
    Result(T),

    #[serde(rename = "error")]
    Error(Arc<ChatError>),

    #[serde(untagged)]
    Undocumented(JsonObject),
}

impl<'de, T: 'de + Deserialize<'de>> ExtractResponse<'de, T> for FfiResponseShape<T> {
    fn extract_response(self) -> Result<T, BadResponseError> {
        match self {
            Self::Result(resp) => Ok(resp),
            Self::Error(err) => Err(BadResponseError::ChatError(err)),
            Self::Undocumented(json) => Err(BadResponseError::Undocumented(json)),
        }
    }
}
"#;

const BAD_RESPONSE_SHENINGANS: &str = r#"
#[derive(Debug)]
pub enum BadResponseError {
    ChatError(Arc<ChatError>),
    InvalidJson(serde_json::Error),
    Undocumented(JsonObject),
}

impl std::error::Error for BadResponseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ChatError(error) => Some(error.as_ref()),
            Self::InvalidJson(error) => Some(error),
            Self::Undocumented(_) => None,
        }
    }
}

impl std::fmt::Display for BadResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatError(resp) => writeln!(f, "Bad response:\n{resp:#}"),
            Self::Undocumented(resp) => writeln!(f, "Unexpected response:\n{resp:#}"),
            Self::InvalidJson(err) => writeln!(f, "Invalid JSON:\n{err:#}"),
        }
    }
}


pub enum UndocumentedResponse<T> {
    Documented(T),
    Undocumented(JsonObject),
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

struct EventKindFmt<'a>(&'a DiscriminatedUnionType);

impl std::fmt::Display for EventKindFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "impl {} {{", self.0.name)?;
        writeln!(f, "   pub fn kind(&self) -> EventKind {{")?;
        writeln!(f, "       match self {{")?;
        for variant in &self.0.variants {
            writeln!(
                f,
                "           Self::{variant}(_) => EventKind::{variant},",
                variant = variant.rust_name
            )?;
        }
        writeln!(
            f,
            "           Self::Undocumented(_) => EventKind::Undocumented,"
        )?;

        writeln!(f, "       }}")?;
        writeln!(f, "   }}")?;
        writeln!(f, "}}")?;

        writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
        writeln!(f, "#[non_exhaustive]")?;
        writeln!(f, "pub enum EventKind {{")?;
        for variant in &self.0.variants {
            writeln!(f, "    {},", variant.rust_name)?;
        }
        writeln!(f, "    Undocumented,")?;
        writeln!(f, "}}\n")?;

        let count = self.0.variants.len() + 1;
        writeln!(f, "impl EventKind {{",)?;
        writeln!(f, "   pub const COUNT: usize = {count};")?;
        writeln!(f)?;
        writeln!(f, "   pub fn as_usize(&self) -> usize  {{")?;
        writeln!(f, "       match self {{")?;
        for (ix, variant) in self.0.variants.iter().enumerate() {
            writeln!(
                f,
                "           Self::{variant} => {ix},",
                variant = variant.rust_name
            )?;
        }
        writeln!(
            f,
            "           Self::Undocumented => {},",
            self.0.variants.len()
        )?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;

        writeln!(f, "   pub fn from_type_str(type_str: &str) -> Self  {{")?;
        writeln!(f, "       match type_str {{")?;
        for variant in &self.0.variants {
            writeln!(
                f,
                "           \"{api_name}\" => Self::{rust_name},",
                api_name = variant.api_name,
                rust_name = variant.rust_name,
            )?;
        }
        writeln!(f, "           _ => Self::Undocumented,",)?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;

        writeln!(f, "}}")
    }
}

struct EventRecordTraitsFmt<'a>(&'a RecordType);

impl std::fmt::Display for EventRecordTraitsFmt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "impl EventData for {} {{", self.0.name)?;
        writeln!(f, "   const KIND: EventKind = EventKind::{};", self.0.name)?;
        writeln!(
            f,
            "   fn from_event(ev: Event) -> Result<Arc<Self>, Event> {{"
        )?;
        writeln!(
            f,
            "       if let Event::{}(data) = ev {{ Ok(data) }} else {{ Err(ev) }}",
            self.0.name
        )?;
        writeln!(f, "   }}")?;
        writeln!(f, "   fn into_event(self: Arc<Self>) -> Event {{")?;
        writeln!(f, "       Event::{}(self)", self.0.name)?;
        writeln!(f, "   }}")?;
        writeln!(f, "}}")
    }
}

struct DiscriminatedUnionConstructors<'a>(&'a DiscriminatedUnionType);

impl std::fmt::Display for DiscriminatedUnionConstructors<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "impl {} {{", self.0.name)?;

        for variant in &self.0.variants {
            write!(f, "pub fn {}(", variant.rust_name.to_case(Case::Snake))?;
            for field in &variant.fields {
                write!(f, "{}: {},", field.rust_name, field.typ)?;
            }

            writeln!(f, ") -> Self {{")?;

            write!(f, "Self::{}", variant.rust_name)?;

            if !variant.fields.is_empty() {
                write!(f, " {{ ")?;
                for field in &variant.fields {
                    write!(f, "{},", field.rust_name)?;
                }
                write!(f, " undocumented: Default::default() }}")?;
            }

            writeln!(f)?;
            writeln!(f, "}}")?;
            writeln!(f)?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}
