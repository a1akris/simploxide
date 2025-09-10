use super::TopLevelDocs;

use convert_case::{Case, Casing as _};
use std::str::FromStr;

pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub doc_comments: Vec<String>,
    pub syntax: String,
}

impl EnumType {
    pub fn new(name: String, variants: Vec<EnumVariant>) -> Self {
        Self {
            name,
            variants,
            doc_comments: Vec::new(),
            syntax: String::new(),
        }
    }
}

impl TopLevelDocs for EnumType {
    fn doc_lines(&self) -> &Vec<String> {
        &self.doc_comments
    }

    fn syntax(&self) -> &str {
        self.syntax.as_str()
    }
}

impl std::fmt::Display for EnumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_docs_fmt(f)?;

        writeln!(
            f,
            "#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]"
        )?;
        writeln!(f, "#[non_exhaustive]")?;
        writeln!(f, "pub enum {} {{", self.name)?;
        writeln!(f, "    #[default]")?;

        for var in &self.variants {
            writeln!(f, "    #[serde(rename = \"{}\")]", var.api_name)?;
            writeln!(f, "    {},", var.rust_name)?;
        }

        writeln!(f, "}}")
    }
}

pub struct EnumVariant {
    pub api_name: String,
    pub rust_name: String,
}

impl EnumVariant {
    pub fn from_api_name(api_name: &str) -> Self {
        Self {
            api_name: api_name.to_owned(),
            rust_name: api_name.to_case(Case::Pascal),
        }
    }
}

impl FromStr for EnumVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('"') || !s.ends_with('"') {
            return Err(format!("'{s}' is not an enum variant name"));
        }

        let api_name = s.trim_matches('"').to_owned();
        let rust_name = api_name.to_case(Case::Pascal);

        Ok(Self {
            api_name,
            rust_name,
        })
    }
}
