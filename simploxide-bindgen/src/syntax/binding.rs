use crate::{
    syntax::{EnumSubstitutions, MaybeBool, lex},
    types::{ApiType, EnumType, Field, RecordType, enum_type::EnumVariant},
};
use std::{borrow::Cow, fmt::Write as _};

use super::SyntaxElement;

macro_rules! bufwriteln {
    ($dst:expr, :>$offset:expr, $($args:tt)*) => {
        write!($dst, "{0:>1$}", " ", $offset).unwrap();
        writeln!($dst, $($args)*).unwrap();
    };
    ($dst:expr, $($args:tt)*) => {
        writeln!($dst, $($args)*).unwrap();
    };
}

pub trait Interpretable {
    fn command_syntax_impl(&self) -> Result<Option<String>, String>;
}

impl Interpretable for ApiType {
    fn command_syntax_impl(&self) -> Result<Option<String>, String> {
        match self {
            ApiType::Record(record_type) => record_type.command_syntax_impl(),
            ApiType::DiscriminatedUnion(_) => Ok(None),
            ApiType::Enum(enum_type) => enum_type.command_syntax_impl(),
        }
    }
}

impl Interpretable for EnumType {
    fn command_syntax_impl(&self) -> Result<Option<String>, String> {
        let binder = Binder { typ: self };
        binder.command_syntax_impl()
    }
}

impl Interpretable for RecordType {
    fn command_syntax_impl(&self) -> Result<Option<String>, String> {
        let binder = Binder { typ: self };
        binder.command_syntax_impl()
    }
}

pub struct Binder<'a, T> {
    typ: &'a T,
}

pub type VariantName<'a> = &'a str;
pub type VariantLiteral<'a> = &'a str;

impl<'a> Binder<'a, EnumType> {
    pub fn command_syntax_impl(&self) -> Result<Option<String>, String> {
        if self.typ.syntax.is_empty() {
            return Ok(None);
        }

        let mut code_buffer = String::with_capacity(1024);

        bufwriteln!(code_buffer, "impl CommandSyntax for {} {{", self.typ.name);
        bufwriteln!(code_buffer, :>4, "fn interpret(&self) -> String {{",);
        bufwriteln!(code_buffer, :>8, "let mut buf = String::new();");

        self.interpreter_body(&mut code_buffer).map_err(|e| {
            format!(
                "{e} in syntax `{}` of the enum type\n{}",
                self.typ.syntax, self.typ
            )
        })?;

        bufwriteln!(code_buffer, :>8, "buf");
        bufwriteln!(code_buffer, :>4, "}}");
        bufwriteln!(code_buffer, "}}");

        Ok(Some(code_buffer))
    }

    fn interpreter_body(&self, code_buffer: &mut String) -> Result<(), String> {
        for tok in lex(self.typ.syntax.as_str()) {
            match tok {
                Ok(element) => match element {
                    SyntaxElement::Literal(lit) => interpret_literal(lit, 8, code_buffer),
                    SyntaxElement::EnumSubstitutions(enum_substitutions) => {
                        self.interpret_subs(&enum_substitutions, None, 8, code_buffer)?;
                    }
                    syn => {
                        return Err(format!("Unsupported syntax element: {syn:?}"));
                    }
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn interpret_subs(
        &self,
        subs: &EnumSubstitutions,
        from_field: Option<&Field>,
        offset: usize,
        code_buffer: &mut String,
    ) -> Result<(), String> {
        let (accessor, enum_ref) = match from_field {
            Some(field) => (format!(".{}", field.rust_name), self.typ.name.as_str()),
            None => (String::new(), "Self"),
        };

        bufwriteln!(code_buffer, :>offset, "match self{accessor} {{");
        for (var_name, literal) in self.variant_substitutions(subs)? {
            bufwriteln!(code_buffer, :>offset + 4, "{enum_ref}::{var_name} => {{");
            interpret_literal(literal, offset + 8, code_buffer);
            bufwriteln!(code_buffer, :>offset + 4, "}}");
        }
        bufwriteln!(code_buffer, :>offset, "}}");
        Ok(())
    }

    fn variant_substitutions(
        &self,
        enum_subs: &'a EnumSubstitutions<'a>,
    ) -> Result<impl Iterator<Item = (VariantName<'a>, VariantLiteral<'a>)>, String> {
        let literals_count = enum_subs.iter().count();

        if self.typ.variants.len() != enum_subs.iter().count() {
            return Err(format!(
                "Bad enum syntax: `{}`. Variants count mismatch.\n\
                 The enum {} has {} variants while the syntax defines {literals_count} literals",
                self.typ.syntax,
                self.typ.name,
                self.typ.variants.len()
            ));
        }

        Ok(self
            .typ
            .variants
            .iter()
            .map(|var| var.rust_name.as_str())
            .zip(enum_subs.iter()))
    }
}

impl<'a> Binder<'a, RecordType> {
    pub fn command_syntax_impl(&self) -> Result<Option<String>, String> {
        if self.typ.syntax.is_empty() {
            return Ok(None);
        }

        let mut code_buffer = String::with_capacity(4096);

        bufwriteln!(code_buffer, "impl CommandSyntax for {} {{", self.typ.name);
        bufwriteln!(code_buffer, :>4, "fn interpret(&self) -> String {{",);

        if self.typ.syntax.contains("json(") {
            bufwriteln!(code_buffer, :>8, "let mut buf = String::with_capacity(1024);");
        } else if self.typ.fields.len() > 2 || self.typ.syntax.contains("[0]>") {
            bufwriteln!(code_buffer, :>8, "let mut buf = String::with_capacity(256);");
        } else {
            bufwriteln!(code_buffer, :>8, "let mut buf = String::with_capacity(64);");
        }

        self.interpreter_body(&mut code_buffer).map_err(|e| {
            format!(
                "{e} in syntax `{}` of the record type\n{}",
                self.typ.syntax, self.typ
            )
        })?;

        bufwriteln!(code_buffer, :>8, "buf");
        bufwriteln!(code_buffer, :>4, "}}");
        bufwriteln!(code_buffer, "}}");

        Ok(Some(code_buffer))
    }

    fn interpreter_body(&self, code_buffer: &mut String) -> Result<(), String> {
        let mut curr_field_ix = 0;

        for tok in lex(self.typ.syntax.as_str()) {
            match tok {
                Ok(element) => match element {
                    SyntaxElement::Literal(lit) => {
                        interpret_literal(lit, 8, code_buffer);
                    }
                    el @ SyntaxElement::EnumSubstitutions(enum_substitutions) => {
                        let field = self.field_by_ix(el, &mut curr_field_ix)?;
                        self.interpret_enum(&enum_substitutions, field, 8, code_buffer)?;
                    }
                    el @ SyntaxElement::MaybeBool(maybe_bool) => match maybe_bool {
                        MaybeBool::On | MaybeBool::Off => {
                            return Err(format!(
                                "Unexpexted non-optional {maybe_bool:?} literal that doesn't provide a choice(expected: on|off)"
                            ));
                        }
                        MaybeBool::Either => {
                            let field = self.field_by_ix(el, &mut curr_field_ix)?;
                            self.interpret_bool(InterpretField::Root(field), 8, code_buffer)?;
                        }
                    },
                    el @ SyntaxElement::TrivialMemberSubstitution { member_name } => {
                        let field = self.field_by_api_name(member_name, el, &mut curr_field_ix)?;
                        self.interpret_trivial_sub(InterpretField::Root(field), 8, code_buffer);
                    }
                    el @ SyntaxElement::DelegateMemberSubstitution { member_name } => {
                        let field = self.field_by_api_name(member_name, el, &mut curr_field_ix)?;
                        self.interpret_delegate_sub(InterpretField::Root(field), 8, code_buffer);
                    }
                    el @ SyntaxElement::JsonMemberSubstitution { member_name } => {
                        let field = self.field_by_api_name(member_name, el, &mut curr_field_ix)?;
                        self.interpret_json_sub(InterpretField::Root(field), 8, code_buffer);
                    }
                    el @ SyntaxElement::VecMemberSubstitution { member_name, delim } => {
                        let field = self.field_by_api_name(member_name, el, &mut curr_field_ix)?;
                        self.interpret_vec_sub(InterpretField::Root(field), delim, 8, code_buffer);
                    }
                    el @ SyntaxElement::Optional { unparsed } => {
                        let field = self.field_by_ix(el, &mut curr_field_ix)?;
                        self.interpret_optional(field, unparsed, 8, code_buffer)?;
                    }
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn interpret_optional(
        &self,
        field: &Field,
        unparsed: &str,
        offset: usize,
        code_buffer: &mut String,
    ) -> Result<(), String> {
        if !field.is_optional() && !field.is_bool() {
            return Err(format!(
                "Expected an optional or a bool field for optional element [{unparsed}] but got {field:?}"
            ));
        }

        if field.is_optional() {
            bufwriteln!(
                code_buffer,
                :>offset, "if let Some(ref {0}) = self.{0} {{",
                field.rust_name
            );
        } else {
            bufwriteln!(code_buffer, :>offset, "if self.{} {{", field.rust_name);
        }

        for tok in lex(unparsed) {
            match tok {
                Ok(element) => match element {
                    SyntaxElement::Literal(lit) => interpret_literal(lit, offset + 4, code_buffer),
                    SyntaxElement::EnumSubstitutions(_) => {
                        // Add on demand
                        return Err(format!(
                            "Enum substitutions are unsupported in optional contexts. Got [{unparsed}]",
                        ));
                    }
                    SyntaxElement::MaybeBool(maybe_bool) => match maybe_bool {
                        MaybeBool::On => {
                            interpret_literal("on", offset + 4, code_buffer);
                        }
                        MaybeBool::Off => {
                            // This could be a bit tricky to add because Off requires a negation of
                            // the if statement written above.
                            return Err(format!(
                                "\"=off\" is unsupported for booleans in optional contexts. Got [{unparsed}]"
                            ));
                        }
                        MaybeBool::Either => {
                            self.interpret_bool(
                                InterpretField::Unwrapped(field),
                                offset + 4,
                                code_buffer,
                            )?;
                        }
                    },
                    el @ SyntaxElement::TrivialMemberSubstitution { member_name } => {
                        self.ensure_field(el, field, member_name)?;
                        self.interpret_trivial_sub(
                            InterpretField::Unwrapped(field),
                            offset + 4,
                            code_buffer,
                        );
                    }
                    el @ SyntaxElement::DelegateMemberSubstitution { member_name } => {
                        self.ensure_field(el, field, member_name)?;
                        self.interpret_delegate_sub(
                            InterpretField::Unwrapped(field),
                            offset + 4,
                            code_buffer,
                        );
                    }
                    el @ SyntaxElement::JsonMemberSubstitution { member_name } => {
                        self.ensure_field(el, field, member_name)?;
                        self.interpret_json_sub(
                            InterpretField::Unwrapped(field),
                            offset + 4,
                            code_buffer,
                        );
                    }
                    el @ SyntaxElement::VecMemberSubstitution { member_name, delim } => {
                        self.ensure_field(el, field, member_name)?;
                        self.interpret_vec_sub(
                            InterpretField::Unwrapped(field),
                            delim,
                            offset + 4,
                            code_buffer,
                        );
                    }
                    SyntaxElement::Optional { unparsed } => {
                        // Add on demand
                        return Err(format!(
                            "Enum substitutions are unsupported in optional contexts. Got [{unparsed}]",
                        ));
                    }
                },
                Err(e) => return Err(e),
            }
        }

        bufwriteln!(code_buffer, :>offset, "}}");
        Ok(())
    }

    fn interpret_enum(
        &self,
        enum_subs: &EnumSubstitutions<'_>,
        field: &Field,
        offset: usize,
        code_buffer: &mut String,
    ) -> Result<(), String> {
        if !field.is_compound() {
            return Err(format!(
                "Expected a enum field but got a {:?} field while processing substitutions {enum_subs:?}",
                field.typ,
            ));
        }

        let ad_hoc_enum = EnumType::new(
            field.typ.clone(),
            enum_subs.iter().map(EnumVariant::from_api_name).collect(),
        );
        let binder = Binder { typ: &ad_hoc_enum };
        binder.interpret_subs(enum_subs, Some(field), offset, code_buffer)?;

        Ok(())
    }

    fn interpret_bool(
        &self,
        field: InterpretField<'_>,
        offset: usize,
        code_buffer: &mut String,
    ) -> Result<(), String> {
        let star = if field.is_unwrapped() { "*" } else { "" };
        let (field, self_str) = field.interpret_vals();

        if !field.is_bool() {
            return Err(format!(
                "The current field must be a bool, but instead it's a {:?}",
                field
            ));
        }

        bufwriteln!(code_buffer, :>offset,  "if {star}{self_str}{} {{", field.rust_name);
        bufwriteln!(code_buffer, :>(offset + 4), "buf.push_str(\"on\");");
        bufwriteln!(code_buffer, :>offset,  "}} else {{");
        bufwriteln!(code_buffer, :>(offset + 4), "buf.push_str(\"off\");");
        bufwriteln!(code_buffer, :>offset,  "}}");

        Ok(())
    }

    fn interpret_trivial_sub(
        &self,
        field: InterpretField<'_>,
        offset: usize,
        code_buffer: &mut String,
    ) {
        let (field, self_str) = field.interpret_vals();
        bufwriteln!(
            code_buffer,
            :>offset, "buf.push_str(&{self_str}{}{}.to_string());",
            field.rust_name,
            maybe_unwrap(&field)
        );
    }

    fn interpret_delegate_sub(
        &self,
        field: InterpretField<'_>,
        offset: usize,
        code_buffer: &mut String,
    ) {
        let (field, self_str) = field.interpret_vals();
        bufwriteln!(
            code_buffer,
            :>offset, "buf.push_str(&{self_str}{}{}.interpret());",
            field.rust_name,
            maybe_unwrap(&field)
        );
    }

    fn interpret_json_sub(
        &self,
        field: InterpretField<'_>,
        offset: usize,
        code_buffer: &mut String,
    ) {
        let (field, self_str) = field.interpret_vals();
        bufwriteln!(
            code_buffer,
            :>offset, "buf.push_str(&serde_json::to_string(&{self_str}{}{}).unwrap());",
            field.rust_name,
            maybe_unwrap(&field)
        );
    }

    fn interpret_vec_sub(
        &self,
        field: InterpretField<'_>,
        delim: &str,
        offset: usize,
        code_buffer: &mut String,
    ) {
        let (field, self_str) = field.interpret_vals();
        bufwriteln!(
            code_buffer,
            :>offset, "buf.push_str(&{self_str}{}{}.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(\"{delim}\"));",
            field.rust_name,
            maybe_unwrap(&field)
        );
    }

    fn ensure_field(
        &self,
        el: SyntaxElement<'_>,
        field: &Field,
        expected: &str,
    ) -> Result<(), String> {
        if expected != field.api_name {
            Err(format!(
                "Optional field mismatch. Expected field {expected:?} but got field {:?} while processing an element {el:?}",
                field.api_name
            ))
        } else {
            Ok(())
        }
    }

    fn field_by_ix(&self, el: SyntaxElement<'_>, ix: &mut usize) -> Result<&Field, String> {
        let field = self
                .typ
                .fields
                .get(*ix)
                .ok_or_else(|| {
                    format!("Expected a field while processing an element {el:?} but found None(current_field_ix={ix})")
                })?;

        *ix += 1;
        Ok(field)
    }

    fn field_by_api_name(
        &self,
        api_name: &str,
        el: SyntaxElement<'_>,
        ix: &mut usize,
    ) -> Result<&Field, String> {
        if let Some(field) = self.typ.fields.get(*ix)
            && field.api_name == api_name
        {
            *ix += 1;
            Ok(field)
        } else {
            let (new_ix, field) = self.typ
                    .fields
                    .iter()
                    .enumerate()
                    .find(|(_, field)| field.api_name == api_name)
                    .ok_or_else(|| format!("Failed to find a struct field with the name: {api_name} while trying to match element {el:?}"))?;

            *ix = new_ix + 1;
            Ok(field)
        }
    }
}

#[derive(Copy, Clone)]
enum InterpretField<'a> {
    /// Normal processing of the self.<field>
    Root(&'a Field),
    /// The field was unwrapped by some statement(e.g. if let Some(_)) in the code above.
    Unwrapped(&'a Field),
}

impl<'a> InterpretField<'a> {
    fn interpret_vals(&self) -> (Cow<'a, Field>, &'static str) {
        fn unwrap_field<'a>(mut x: Cow<'a, Field>) -> Cow<'a, Field> {
            x.to_mut().typ = x.inner_type().unwrap_or_default().to_owned();
            x
        }

        match self {
            InterpretField::Root(field) => (Cow::Borrowed(field), "self."),
            InterpretField::Unwrapped(field) => (unwrap_field(Cow::Borrowed(field)), ""),
        }
    }

    fn is_unwrapped(&self) -> bool {
        matches!(self, Self::Unwrapped(_))
    }
}

fn interpret_literal(literal: &str, offset: usize, code_buffer: &mut String) {
    if literal.len() == 1 {
        bufwriteln!(code_buffer, :>offset, "buf.push('{literal}');");
    } else {
        bufwriteln!(code_buffer, :>offset, "buf.push_str(\"{literal}\");");
    }
}

fn maybe_unwrap(field: &Field) -> &'static str {
    if field.is_optional() {
        let unwrapped = field.to_inner().unwrap();
        if unwrapped.is_compound() {
            ""
        } else if !unwrapped.is_numeric() {
            ".as_deref().unwrap_or_default()"
        } else {
            ".unwrap_or_default()"
        }
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn enum_interpreter() {
        let mut test_enum = EnumType::new(
            "Greeting".to_owned(),
            vec!["\"hi\"".parse().unwrap(), "\"bye\"".parse().unwrap()],
        );

        test_enum.syntax = "Hello,|Goodbye,| World!".to_owned();

        let interpreter_impl = test_enum.command_syntax_impl().unwrap().unwrap();

        expect![[r#"
            impl CommandSyntax for Greeting {
                fn interpret(&self) -> String {
                    let mut buf = String::with_capacity(64);
                    match self {
                        Self::Hi => { buf.push_str("Hello,"); }
                        Self::Bye => { buf.push_str("Goodbye,"); }
                    }
                    buf.push_str(" ");
                    buf.push_str("World!");
                    buf
                }
             }
        "#]]
        .assert_eq(&interpreter_impl);
    }

    trait CommandSyntax {
        fn interpret(&self) -> String;
    }

    enum Greeting {
        Hi,
        Bye,
    }

    impl CommandSyntax for Greeting {
        fn interpret(&self) -> String {
            let mut buf = String::with_capacity(64);
            match self {
                Self::Hi => {
                    buf.push_str("Hello,");
                }
                Self::Bye => {
                    buf.push_str("Goodbye,");
                }
            }
            buf.push(' ');
            buf.push_str("World!");
            buf
        }
    }

    #[test]
    fn real_greeting() {
        assert_eq!(Greeting::Hi.interpret(), "Hello, World!");
        assert_eq!(Greeting::Bye.interpret(), "Goodbye, World!");
    }
}
