//! This module defines both data types that represent different typekinds in the SimpleX API docs
//! as well as the parser that turns TYPES.md into an iterator that yields these data types. The
//! data types' std::fmt::Display impl renders them as the Rust code.

pub mod discriminated_union_type;
pub mod enum_type;
pub mod record_type;

pub use discriminated_union_type::{
    DiscriminatedUnionType, DisjointedDiscriminatedUnion, DisjointedDisriminatedUnionVariant,
};
pub use enum_type::EnumType;
pub use record_type::RecordType;

use convert_case::{Case, Casing as _};
use std::str::FromStr;

use crate::parse_utils;

pub fn parse(types_md: &str) -> impl Iterator<Item = Result<ApiType, String>> {
    types_md.split("---").skip(1).map(ApiType::from_str)
}

pub enum ApiType {
    Record(RecordType),
    DiscriminatedUnion(DiscriminatedUnionType),
    Enum(EnumType),
}

impl ApiType {
    pub fn is_error(&self) -> bool {
        self.name().contains("Error")
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Record(r) => r.name.as_str(),
            Self::Enum(e) => e.name.as_str(),
            Self::DiscriminatedUnion(du) => du.name.as_str(),
        }
    }
}

impl std::str::FromStr for ApiType {
    type Err = String;

    fn from_str(md_block: &str) -> Result<Self, Self::Err> {
        fn parser<'a>(mut lines: impl Iterator<Item = &'a str>) -> Result<ApiType, String> {
            const TYPENAME_PAT: &str = parse_utils::H2;
            const TYPEKIND_PAT: &str = parse_utils::BOLD;

            let typename = parse_utils::skip_empty(&mut lines)
                .and_then(|s| s.strip_prefix(TYPENAME_PAT))
                .ok_or_else(|| format!("Failed to find a type name by pattern {TYPENAME_PAT:?}"))?;

            let mut doc_comments = Vec::new();

            let typekind = parse_utils::parse_doc_lines(&mut lines, &mut doc_comments, |s| {
                s.starts_with(TYPEKIND_PAT)
            })
            .map(|s| s.strip_prefix(TYPEKIND_PAT).unwrap())
            .ok_or_else(|| format!("Failed to find a type kind by pattern {TYPEKIND_PAT:?}"))?;

            let mut syntax = String::new();
            let breaker = |s: &str| s.starts_with("**Syntax");

            if typekind.starts_with("Record") {
                let mut fields = Vec::new();

                let syntax_block =
                    parse_utils::parse_record_fields(&mut lines, &mut fields, breaker)?;

                if syntax_block.is_some() {
                    parse_utils::parse_syntax(&mut lines, &mut syntax)?;
                }

                Ok(ApiType::Record(RecordType {
                    name: typename.to_owned(),
                    fields,
                    doc_comments,
                    syntax,
                }))
            } else if typekind.starts_with("Enum") {
                let mut variants = Vec::new();

                let syntax_block =
                    parse_utils::parse_enum_variants(&mut lines, &mut variants, breaker)?;

                if syntax_block.is_some() {
                    parse_utils::parse_syntax(&mut lines, &mut syntax)?;
                }

                Ok(ApiType::Enum(EnumType {
                    name: typename.to_owned(),
                    variants,
                    doc_comments,
                    syntax,
                }))
            } else if typekind.starts_with("Discriminated") {
                let mut variants = Vec::new();

                let syntax_block = parse_utils::parse_discriminated_union_variants(
                    &mut lines,
                    &mut variants,
                    breaker,
                )?;

                if syntax_block.is_some() {
                    parse_utils::parse_syntax(&mut lines, &mut syntax)?;
                }

                Ok(ApiType::DiscriminatedUnion(DiscriminatedUnionType {
                    name: typename.to_owned(),
                    variants,
                    doc_comments,
                    syntax,
                }))
            } else {
                Err(format!("Unknown type kind: {typekind:?}"))
            }
        }

        parser(md_block.lines().map(str::trim))
            .map_err(|e| format!("{e} in md block\n```\n{md_block}\n```"))
    }
}

impl std::fmt::Display for ApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Record(r) => r.fmt(f),
            Self::Enum(e) => e.fmt(f),
            Self::DiscriminatedUnion(du) => du.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub api_name: String,
    pub rust_name: String,
    pub typ: String,
}

impl Field {
    pub fn from_api_name(api_name: String, typ: String) -> Self {
        Self {
            api_name: api_name.clone(),
            rust_name: api_name.to_case(Case::Snake),
            typ,
        }
    }
    pub fn is_optional(&self) -> bool {
        is_optional_type(self.typ.as_str())
    }

    pub fn is_vec(&self) -> bool {
        is_vec_type(self.typ.as_str())
    }

    pub fn is_map(&self) -> bool {
        is_map_type(self.typ.as_str())
    }

    pub fn is_numeric(&self) -> bool {
        is_numeric_type(self.typ.as_str())
    }

    pub fn is_bool(&self) -> bool {
        is_bool_type(self.typ.as_str())
    }

    pub fn is_string(&self) -> bool {
        is_string_type(self.typ.as_str())
    }

    pub fn is_compound(&self) -> bool {
        is_compound_type(self.typ.as_str())
    }

    /// Retrieves the inner type of Option<_> or Vec<_>
    /// Returns None if the field type is not Option or Vec.
    pub fn inner_type(&self) -> Option<&str> {
        inner_type(self.typ.as_str())
    }

    // /// Produces the field with the same name but with the type
    // /// of the [`inner_type`]
    // pub fn to_inner(&self) -> Option<Field> {
    //     self.inner_type().map(|t| Field {
    //         typ: t.to_owned(),
    //         rust_name: self.rust_name.clone(),
    //         api_name: self.api_name.clone(),
    //     })
    // }
}

impl FromStr for Field {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, typ) = line
            .trim()
            .split_once(':')
            .ok_or_else(|| format!("Failed to parse field at line: '{line}'"))?;

        let api_name = name.trim().to_owned();
        let rust_name = api_name.to_case(Case::Snake);
        let typ = resolve_type(typ.trim())?;

        Ok(Field {
            api_name,
            rust_name,
            typ,
        })
    }
}

pub fn is_optional_type(typ: &str) -> bool {
    typ.starts_with("Option<")
}

pub fn is_vec_type(typ: &str) -> bool {
    typ.starts_with("Vec<")
}

pub fn is_map_type(typ: &str) -> bool {
    typ.starts_with("HashMap<")
}

pub fn is_numeric_type(typ: &str) -> bool {
    typ.starts_with("i")
        || typ.starts_with("f")
        || typ.starts_with("u")
        || typ.starts_with("Option<i")
        || typ.starts_with("Option<f")
        || typ.starts_with("Option<u")
}

pub fn is_bool_type(typ: &str) -> bool {
    typ == "bool"
}

pub fn is_string_type(typ: &str) -> bool {
    typ == "String" || typ == "UtcTime"
}

pub fn is_compound_type(typ: &str) -> bool {
    !is_optional_type(typ)
        && !is_vec_type(typ)
        && !is_map_type(typ)
        && !is_numeric_type(typ)
        && !is_bool_type(typ)
        && !is_string_type(typ)
}

/// Retrieves the inner type of Option<_> or Vec<_>
/// Returns None if the field type is not Option or Vec.
pub fn inner_type(typ: &str) -> Option<&str> {
    if let Some(opt) = typ.strip_prefix("Option<") {
        let end = opt.rfind('>').unwrap();
        Some(&opt[..end])
    } else if let Some(vec) = typ.strip_prefix("Vec<") {
        let end = vec.rfind('>').unwrap();
        Some(&vec[..end])
    } else {
        None
    }
}

fn resolve_type(t: &str) -> Result<String, String> {
    if let Some(t) = t.strip_suffix('}') {
        let t = t.strip_prefix('{').unwrap().trim();
        let (lhs, rhs) = t.split_once(':').unwrap();

        let key = resolve_type(lhs.trim())?;
        let val = resolve_type(rhs.trim())?;

        return Ok(format!("HashMap<{key}, {val}>"));
    }

    if let Some(t) = t.strip_suffix(']') {
        let resolved = resolve_type(t.strip_prefix('[').unwrap())?;
        return Ok(format!("Vec<{resolved}>"));
    }

    if let Some(t) = t.strip_suffix('?') {
        let resolved = resolve_type(t)?;
        return Ok(format!("Option<{resolved}>"));
    }

    let resolved = match t {
        "bool" => "bool".to_owned(),
        "int" => "i32".to_owned(),
        "int64" => "i64".to_owned(),
        "word32" => "u32".to_owned(),
        "double" => "f64".to_owned(),
        "string" => "String".to_owned(),
        // These types map into themselves to preserve semantics.
        // The generated module MUST have the typedefs like these:
        //  - type UtcTime = String;
        //  - type JsonObject = serde_json::Value;
        "UTCTime" => "UtcTime".to_owned(),
        "JSONObject" => "JsonObject".to_owned(),

        compound if compound.starts_with('[') => {
            let end = compound.find(']').unwrap();
            compound['['.len_utf8()..end].to_owned()
        }

        _ => return Err(format!("Failed to resolve type: `{t}`")),
    };

    Ok(resolved)
}

/// A helper that allows to configure how the field should be rendred.
pub struct FieldFmt<'a> {
    field: &'a Field,
    offset: usize,
    is_pub: bool,
}

impl<'a> FieldFmt<'a> {
    pub fn new(field: &'a Field) -> Self {
        Self::with_offset(field, 0)
    }

    pub fn with_offset(field: &'a Field, offset: usize) -> Self {
        Self {
            field,
            offset,
            is_pub: false,
        }
    }

    pub fn set_pub(&mut self, new: bool) {
        self.is_pub = new;
    }
}

impl<'a> std::fmt::Display for FieldFmt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset = " ".repeat(self.offset);
        let pub_ = if self.is_pub { "pub " } else { "" };
        let is_numeric = self.field.is_numeric();
        let is_optional = self.field.is_optional();

        write!(f, "{offset}#[serde(rename = \"{}\"", self.field.api_name)?;

        if is_optional {
            write!(f, ", skip_serializing_if = \"Option::is_none\"")?;
        }

        if is_numeric {
            if is_optional {
                write!(
                    f,
                    ", deserialize_with=\"deserialize_option_number_from_string\", default"
                )?;
            } else {
                write!(f, ", deserialize_with=\"deserialize_number_from_string\"")?;
            }
        }

        writeln!(f, ")]")?;
        writeln!(
            f,
            "{offset}{pub_}{}: {},",
            self.field.rust_name, self.field.typ
        )?;

        Ok(())
    }
}

/// A common impl for outer docs rendering shared by all type kinds.
pub(crate) trait TopLevelDocs {
    fn doc_lines(&self) -> &Vec<String>;

    fn syntax(&self) -> &str;

    fn write_docs_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.doc_lines() {
            writeln!(f, "/// {line}")?;
        }

        if !self.syntax().is_empty() {
            if !self.doc_lines().is_empty() {
                writeln!(f, "///")?;
            }

            writeln!(f, "/// *Syntax:*")?;
            writeln!(f, "///")?;
            writeln!(f, "/// ```")?;
            writeln!(f, "/// {}", self.syntax())?;
            writeln!(f, "/// ```")?;
        }

        Ok(())
    }
}
