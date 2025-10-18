use convert_case::{Case, Casing};

use super::{Field, FieldFmt, RecordType, TopLevelDocs};

pub struct DiscriminatedUnionType {
    pub name: String,
    pub variants: Vec<DiscriminatedUnionVariant>,
    pub doc_comments: Vec<String>,
    pub syntax: String,
}

impl DiscriminatedUnionType {
    pub fn new(name: String, variants: Vec<DiscriminatedUnionVariant>) -> Self {
        Self {
            name,
            variants,
            doc_comments: Vec::new(),
            syntax: String::new(),
        }
    }

    /// See [`DisjointedDiscriminatedUnion`]
    pub fn disjoin(self) -> (DiscriminatedUnionType, Vec<RecordType>) {
        let tmp: DisjointedDiscriminatedUnion =
            self.variants.into_iter().map(|v| v.disjoin()).collect();

        let (mut this, records) = tmp.into_types(self.name);
        this.doc_comments = self.doc_comments;
        this.syntax = self.syntax;
        (this, records)
    }
}

impl TopLevelDocs for DiscriminatedUnionType {
    fn doc_lines(&self) -> &Vec<String> {
        &self.doc_comments
    }

    fn syntax(&self) -> &str {
        self.syntax.as_str()
    }
}

impl std::fmt::Display for DiscriminatedUnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_docs_fmt(f)?;
        writeln!(
            f,
            "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]"
        )?;
        writeln!(f, "#[serde(tag = \"type\")]")?;
        writeln!(f, "#[non_exhaustive]")?;
        writeln!(f, "pub enum {} {{", self.name)?;

        for variant in &self.variants {
            for comment_line in &variant.doc_comments {
                writeln!(f, "    /// {}", comment_line)?;
            }

            writeln!(f, "    #[serde(rename = \"{}\")]", variant.api_name)?;

            if variant.fields.is_empty() {
                writeln!(f, "    {},", variant.rust_name)?;
                continue;
            } else if variant.fields[0].rust_name.is_empty() {
                writeln!(f, "    {}({}),", variant.rust_name, variant.fields[0].typ)?;
                continue;
            } else {
                writeln!(f, "    {} {{", variant.rust_name)?;
            }

            for field in &variant.fields {
                let field_fmt = FieldFmt::with_offset(field, 8);
                writeln!(f, "{field_fmt}")?;
            }

            // Capture undocumented fields
            writeln!(
                f,
                "        #[serde(flatten, skip_serializing_if = \"BTreeMap::is_empty\")]"
            )?;
            writeln!(f, "        undocumented: BTreeMap<String, JsonObject>")?;

            writeln!(f, "    }},")?;
        }

        writeln!(f, "    #[serde(untagged)]")?;
        writeln!(f, "    Undocumented(BTreeMap<String, JsonObject>)")?;

        writeln!(f, "}}")
    }
}

#[derive(Clone, PartialEq)]
pub struct DiscriminatedUnionVariant {
    pub api_name: String,
    pub rust_name: String,
    pub doc_comments: Vec<String>,

    pub fields: Vec<Field>,
}

impl DiscriminatedUnionVariant {
    pub fn from_api_name(api_name: String, fields: Vec<Field>) -> Self {
        Self {
            api_name: api_name.clone(),
            rust_name: api_name.to_case(Case::Pascal),
            doc_comments: Vec::new(),
            fields,
        }
    }
}

impl DiscriminatedUnionVariant {
    /// Turns `VariantName { f1: T, f2: U }` into a `VariantName(VariantName)` and a `struct
    /// VariantName { f1: T, f2: U }`
    pub fn disjoin(mut self) -> DisjointedDisriminatedUnionVariant {
        let record_name = self.rust_name.clone();

        let record_fields = std::mem::replace(
            &mut self.fields,
            vec![Field {
                api_name: "".to_owned(),
                rust_name: "".to_owned(),
                typ: record_name.clone(),
            }],
        );

        DisjointedDisriminatedUnionVariant {
            variant: self,
            record: RecordType::new(record_name, record_fields),
        }
    }
}

/// An alternative representation of the discriminated union that moves all variant fields into
/// separate structs. E.g a union of `VariantName(VariantName)` variants where nested VariantName
/// is a struct holding the variant fields.
pub struct DisjointedDiscriminatedUnion {
    pub variants: Vec<DisjointedDisriminatedUnionVariant>,
}

impl DisjointedDiscriminatedUnion {
    /// Construct the resulting disjointed discriminated union with the name `union_name` and get
    /// it complementing record types in the vec.
    pub fn into_types(self, union_name: String) -> (DiscriminatedUnionType, Vec<RecordType>) {
        let (variants, records) = self
            .variants
            .into_iter()
            .map(|var| (var.variant, var.record))
            .unzip();

        (DiscriminatedUnionType::new(union_name, variants), records)
    }
}

impl FromIterator<DisjointedDisriminatedUnionVariant> for DisjointedDiscriminatedUnion {
    fn from_iter<T: IntoIterator<Item = DisjointedDisriminatedUnionVariant>>(iter: T) -> Self {
        Self {
            variants: iter.into_iter().collect(),
        }
    }
}

pub struct DisjointedDisriminatedUnionVariant {
    pub variant: DiscriminatedUnionVariant,
    pub record: RecordType,
}
