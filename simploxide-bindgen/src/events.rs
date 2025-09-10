use crate::{parse_utils, types::DisjointedDisriminatedUnionVariant};

pub fn parse(
    events_md: &str,
) -> impl Iterator<Item = Result<DisjointedDisriminatedUnionVariant, String>> {
    let mut parser = Parser::default();

    events_md
        .split("---")
        .skip(1)
        .filter_map(|s| {
            let trimmed = s.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .map(move |blk| parser.parse_block(blk))
}

#[derive(Default)]
struct Parser {
    current_doc_section: Option<DocSection>,
}

impl Parser {
    pub fn parse_block(
        &mut self,
        block: &str,
    ) -> Result<DisjointedDisriminatedUnionVariant, String> {
        self.parser(block.lines().map(str::trim))
            .map_err(|e| format!("{e} in block\n```\n{block}\n```"))
    }

    fn parser<'a>(
        &mut self,
        mut lines: impl Iterator<Item = &'a str>,
    ) -> Result<DisjointedDisriminatedUnionVariant, String> {
        const DOC_SECTION_PAT: &str = parse_utils::H2;
        const TYPENAME_PAT: &str = parse_utils::H3;
        const TYPEKIND_PAT: &str = parse_utils::BOLD;

        let mut next =
            parse_utils::skip_empty(&mut lines).ok_or_else(|| "Got an empty block".to_owned())?;

        let mut inner_docs: Vec<String> = Vec::new();

        loop {
            if let Some(section_name) = next.strip_prefix(DOC_SECTION_PAT) {
                let mut doc_section = DocSection::new(section_name.to_owned());

                next = parse_utils::parse_doc_lines(&mut lines, &mut doc_section.contents, |s| {
                    s.starts_with(TYPENAME_PAT)
                })
                .ok_or_else(|| format!("Failed to find a typename by pattern {TYPENAME_PAT:?} after the doc section"))?;

                self.current_doc_section.replace(doc_section);
            } else if let Some(_name) = next.strip_prefix(TYPENAME_PAT) {
                parse_utils::parse_doc_lines(&mut lines, &mut inner_docs, |s| {
                    s.starts_with(TYPEKIND_PAT)
                }).ok_or_else(|| format!("Failed to find a typekind by pattern {TYPEKIND_PAT:?} after the inner docs "))?;

                break;
            }
        }

        let (variant, _) = parse_utils::parse_discriminated_union_variant(&mut lines)?;
        let mut disjointed = variant.disjoin();

        // Spread documentation between enum and structs
        //
        if let Some(ref outer_docs) = self.current_doc_section {
            disjointed
                .variant
                .doc_comments
                .push(outer_docs.header.clone());

            disjointed
                .record
                .doc_comments
                .push(format!("### {}", outer_docs.header.clone()));

            disjointed.record.doc_comments.push(String::new());

            disjointed
                .record
                .doc_comments
                .extend(outer_docs.contents.iter().cloned());

            disjointed.record.doc_comments.push(String::new());
            disjointed.record.doc_comments.push("----".to_owned());
            disjointed.record.doc_comments.push(String::new());
        }

        disjointed.record.doc_comments.extend(inner_docs);
        Ok(disjointed)
    }
}

#[derive(Default, Clone)]
struct DocSection {
    header: String,
    contents: Vec<String>,
}

impl DocSection {
    fn new(header: String) -> Self {
        Self {
            header,
            contents: Vec::new(),
        }
    }
}
