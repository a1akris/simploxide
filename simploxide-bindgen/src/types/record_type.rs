use super::{Field, FieldFmt, TopLevelDocs};

#[derive(Clone, PartialEq)]
pub struct RecordType {
    pub name: String,
    pub fields: Vec<Field>,
    pub doc_comments: Vec<String>,
    pub syntax: String,
}

impl RecordType {
    pub fn new(name: String, fields: Vec<Field>) -> Self {
        Self {
            name,
            fields,
            doc_comments: Vec::new(),
            syntax: String::new(),
        }
    }

    pub fn with_comments(name: String, fields: Vec<Field>, doc_comments: Vec<String>) -> Self {
        Self {
            name,
            fields,
            doc_comments,
            syntax: String::new(),
        }
    }

    pub fn is_error(&self) -> bool {
        self.name.contains("Error")
    }
}

impl TopLevelDocs for RecordType {
    fn doc_lines(&self) -> &Vec<String> {
        &self.doc_comments
    }

    fn syntax(&self) -> &str {
        self.syntax.as_str()
    }
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_docs_fmt(f)?;

        writeln!(
            f,
            "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
        )?;
        writeln!(f, "#[cfg_attr(feature = \"bon\", derive(::bon::Builder))]")?;
        writeln!(
            f,
            "#[cfg_attr(feature = \"bon\", builder(on(String, into)))]"
        )?;
        writeln!(f, "pub struct {} {{", self.name)?;

        for field in self.fields.iter() {
            let mut field_fmt = FieldFmt::with_offset(field, 4);
            field_fmt.set_pub(true);
            writeln!(f, "{field_fmt}")?;
        }

        // Capture undocumented fields
        writeln!(
            f,
            "    #[serde(flatten, skip_serializing_if = \"BTreeMap::is_empty\")]"
        )?;
        writeln!(f, "    #[cfg_attr(feature = \"bon\", builder(default))]")?;
        writeln!(f, "    pub undocumented: BTreeMap<String, JsonObject>")?;

        writeln!(f, "}}")
    }
}
