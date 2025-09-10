use crate::types::{
    Field, discriminated_union_type::DiscriminatedUnionVariant, enum_type::EnumVariant,
};

// pub const H1: &str = "# ";
pub const H2: &str = "## ";
pub const H3: &str = "### ";
pub const BOLD: &str = "**";
pub const LIST_ITEM: &str = "- ";

const FIELD_PAT: &str = LIST_ITEM;

#[must_use]
pub fn skip_while<'i, 's, F: Fn(&str) -> bool>(
    lines: &'i mut impl Iterator<Item = &'s str>,
    cond: F,
) -> Option<&'s str> {
    lines.find(|&s| !cond(s))
}

#[must_use]
pub fn skip_empty<'i, 's>(lines: &'i mut impl Iterator<Item = &'s str>) -> Option<&'s str> {
    skip_while(lines, str::is_empty)
}

#[must_use]
pub fn parse_doc_lines<'a, 'b, B: Fn(&str) -> bool>(
    lines: &'a mut impl Iterator<Item = &'b str>,
    out_buffer: &mut Vec<String>,
    breaker: B,
) -> Option<&'b str> {
    let mut ret_line = None;
    for line in lines {
        if breaker(line) {
            ret_line = Some(line);
            break;
        }

        // skip trailing empty lines
        if line.is_empty() && out_buffer.is_empty() {
            continue;
        }

        out_buffer.push(line.to_owned());
    }

    // Remove trailing empty lines from end
    while out_buffer
        .last()
        .map(|s| s.trim().is_empty())
        .unwrap_or(false)
    {
        out_buffer.pop();
    }

    ret_line
}

pub fn parse_syntax<'a, 'b>(
    lines: &'a mut impl Iterator<Item = &'b str>,
    out_buffer: &mut String,
) -> Result<Option<&'b str>, String> {
    let next = skip_empty(lines).map(|s| s.trim());

    if next != Some("```") {
        return Err(format!(
            "Expected opening syntax quotes but found line {next:?}"
        ));
    }

    let syntax = lines
        .next()
        .map(|s| s.trim())
        .ok_or_else(|| "Expected syntax line but found nothing".to_owned())?;

    out_buffer.push_str(syntax);

    let next = lines.next().map(|s| s.trim());

    if next != Some("```") {
        return Err(format!(
            "Expected closing syntax quotes but found line {next:?}"
        ));
    }

    Ok(lines.next())
}

pub fn parse_enum_variants<'a, 'b, B: Fn(&str) -> bool>(
    lines: &'a mut impl Iterator<Item = &'b str>,
    out_buffer: &mut Vec<EnumVariant>,
    breaker: B,
) -> Result<Option<&'b str>, String> {
    for line in lines {
        if breaker(line) {
            return Ok(Some(line));
        }

        if let Some(name) = line.strip_prefix(FIELD_PAT) {
            out_buffer.push(name.parse()?);
        }
    }

    Ok(None)
}

pub fn parse_record_fields<'a, 'b, B: Fn(&str) -> bool>(
    lines: &'a mut impl Iterator<Item = &'b str>,
    out_buffer: &mut Vec<Field>,
    breaker: B,
) -> Result<Option<&'b str>, String> {
    for line in lines {
        if breaker(line) {
            return Ok(Some(line));
        }

        if let Some(field) = line.strip_prefix(FIELD_PAT) {
            out_buffer.push(field.parse()?);
        }
    }

    Ok(None)
}

pub fn parse_discriminated_union_variant<'a, 'b>(
    lines: &'a mut impl Iterator<Item = &'b str>,
) -> Result<(DiscriminatedUnionVariant, Option<&'b str>), String> {
    let enum_variant: EnumVariant = lines
        .next()
        .and_then(|s| s.strip_prefix(FIELD_PAT))
        .and_then(|s| s.strip_prefix("type: "))
        .ok_or_else(|| {
            "Haven't found the \"- type:\" field. It should come before any other discriminated variant fields"
                .to_owned()
        })
        .and_then(|s| s.parse())?;

    let mut fields = Vec::new();

    for line in lines {
        if !line.starts_with(FIELD_PAT) {
            let ret = DiscriminatedUnionVariant {
                api_name: enum_variant.api_name,
                rust_name: enum_variant.rust_name,
                doc_comments: Vec::new(),
                fields,
            };

            return Ok((ret, Some(line)));
        }

        let field = line.strip_prefix(FIELD_PAT).unwrap();
        fields.push(field.parse()?);
    }

    let ret = DiscriminatedUnionVariant {
        api_name: enum_variant.api_name,
        rust_name: enum_variant.rust_name,
        doc_comments: Vec::new(),
        fields,
    };

    Ok((ret, None))
}

pub fn parse_discriminated_union_variants<'a, 'b, B: Fn(&str) -> bool>(
    lines: &'a mut impl Iterator<Item = &'b str>,
    out_buffer: &mut Vec<DiscriminatedUnionVariant>,
    breaker: B,
) -> Result<Option<&'b str>, String> {
    let mut enum_variant: Option<EnumVariant> = None;
    let mut fields: Vec<Field> = Vec::new();

    for line in lines {
        if breaker(line) {
            if let Some(var) = enum_variant {
                out_buffer.push(DiscriminatedUnionVariant {
                    api_name: var.api_name,
                    rust_name: var.rust_name,
                    doc_comments: Vec::new(),
                    fields: fields.clone(),
                });
            }

            return Ok(Some(line));
        }

        if let Some(field) = line.strip_prefix(FIELD_PAT) {
            if let Some(name) = field.strip_prefix("type: ") {
                let var: EnumVariant = name.parse()?;

                if let Some(old_var) = enum_variant.replace(var) {
                    out_buffer.push(DiscriminatedUnionVariant {
                        api_name: old_var.api_name,
                        rust_name: old_var.rust_name,
                        doc_comments: Vec::new(),
                        fields: fields.clone(),
                    });

                    fields.clear();
                }
            } else {
                fields.push(field.parse()?);
            }
        }
    }

    if let Some(var) = enum_variant {
        out_buffer.push(DiscriminatedUnionVariant {
            api_name: var.api_name,
            rust_name: var.rust_name,
            doc_comments: Vec::new(),
            fields: fields.clone(),
        });
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enum_type::EnumVariant;

    const HELLO: &str = r#"

Hello!
World!

## Title

"#;
    const CHAT_TYPE: &str = r#"

## ChatType

**Enum type**:
- "direct"
- "group"
- "local"

**Syntax**:

```
@|#|*|
```

```javascript
self == 'contact' ? '@' : self == 'group' ? '#' : self == 'local' ? '*' : '' // JavaScript
```

```python
'@' if str(self) == 'contact' else '#' if str(self) == 'group' else '*' if str(self) == 'local' else '' # Python
```



---
"#;

    fn to_end(_: &str) -> bool {
        false
    }

    #[test]
    fn lines_skipper() {
        let mut lines = HELLO.lines();

        let line = skip_empty(&mut lines);
        assert_eq!(line, Some("Hello!"));
        assert_eq!(lines.next(), Some("World!"));
    }

    #[test]
    fn doc_parser() {
        let mut lines = HELLO.lines();
        let mut docs = Vec::new();

        let next = parse_doc_lines(&mut lines, &mut docs, |s| s.starts_with("##")).unwrap();

        assert_eq!(docs, vec!["Hello!", "World!"]);
        assert_eq!(next, "## Title");

        docs.clear();
        let mut lines = HELLO.lines();

        let next = parse_doc_lines(&mut lines, &mut docs, to_end);
        assert!(next.is_none());
        assert_eq!(docs, vec!["Hello!", "World!", "", "## Title"]);
    }

    #[test]
    fn syntax_parser() {
        let mut lines = CHAT_TYPE.lines();

        skip_while(&mut lines, |s| s != "**Syntax**:").unwrap();

        let mut syntax = String::new();

        let _ = parse_syntax(&mut lines, &mut syntax).unwrap();

        assert_eq!(syntax, "@|#|*|");
    }

    #[test]
    fn enum_variants() {
        let mut lines = CHAT_TYPE.lines();

        skip_while(&mut lines, |s| s != "**Enum type**:").unwrap();

        let mut variants: Vec<EnumVariant> = Vec::new();
        let next = parse_enum_variants(&mut lines, &mut variants, |s| s.starts_with("**")).unwrap();

        assert_eq!(next, Some("**Syntax**:"));

        let api_names: Vec<_> = variants.iter().map(|var| var.api_name.as_str()).collect();
        let rust_names: Vec<_> = variants.iter().map(|var| var.rust_name.as_str()).collect();

        assert_eq!(api_names, vec!["direct", "group", "local"]);
        assert_eq!(rust_names, vec!["Direct", "Group", "Local"]);
    }
}
