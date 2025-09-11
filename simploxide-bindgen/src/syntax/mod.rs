//! A module with a simple greedy lexer for the SimpleX api command syntax and a binding submodule
//! that generates actual interpreters.

pub mod binding;

pub use binding::Interpretable;

#[derive(Debug, Clone, Copy)]
pub enum SyntaxElement<'a> {
    /// /_command
    Literal(&'a str),
    /// @|#|*|
    EnumSubstitutions(EnumSubstitutions<'a>),
    /// on, off, on|off
    MaybeBool(MaybeBool),
    /// <member_name>
    TrivialMemberSubstitution { member_name: &'a str },
    /// <str(member_name)>
    DelegateMemberSubstitution { member_name: &'a str },
    /// <json(member_name)>
    JsonMemberSubstitution { member_name: &'a str },
    /// <member_name[0]>[,<member_name[1]>...]
    VecMemberSubstitution {
        member_name: &'a str,
        delim: &'a str,
    },
    /// [whatever]
    Optional { unparsed: &'a str },
}

impl<'a> SyntaxElement<'a> {
    fn new_enum_substitutions(subs: &'a str) -> Self {
        Self::EnumSubstitutions(EnumSubstitutions(subs))
    }

    #[cfg(test)]
    fn optional(&self) -> Option<&'a str> {
        if let Self::Optional { unparsed } = self {
            Some(unparsed)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumSubstitutions<'a>(&'a str);

impl<'a> EnumSubstitutions<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        self.0.split('|').map(str::trim).filter(|s| !s.is_empty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaybeBool {
    On,
    Off,
    Either,
}

pub fn lex(s: &str) -> Lexer<'_> {
    Lexer::new(s)
}

/// An iterator over syntax elements
pub struct Lexer<'a> {
    syntax: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(syntax: &'a str) -> Self {
        Self { syntax }
    }
}

// TODO: This impl needs some love. Refactor it to remove unwraps and handle errors uniformly
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<SyntaxElement<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        const SIGNIFICANT_CHARS: &[char] = &[' ', '=', '<', '['];

        enum NaughtyLiteral<'a> {
            Literal(&'a str),
            EnumSubs(&'a str),
            Bool(MaybeBool),
        }

        // Need to distinguish literals to know how to deal with the trailing spaces. Normal
        // literals include them while enum substitutions exclude them so spaces must become
        // separate literals before/after EnumSubs
        fn deduce_literal(s: &str) -> Result<NaughtyLiteral<'_>, String> {
            if s.contains('|') {
                if s.ends_with('=') {
                    return Err(format!(
                        "Unexpected `=` operator after enum substitution {s}"
                    ));
                }
                let s = s.trim();

                match s {
                    "on|off" => Ok(NaughtyLiteral::Bool(MaybeBool::Either)),
                    _ => Ok(NaughtyLiteral::EnumSubs(s)),
                }
            } else {
                match s {
                    "on" => Ok(NaughtyLiteral::Bool(MaybeBool::On)),
                    "off" => Ok(NaughtyLiteral::Bool(MaybeBool::Off)),
                    _ => Ok(NaughtyLiteral::Literal(s)),
                }
            }
        }

        if self.syntax.is_empty() {
            return None;
        }

        let mut pos = match self.syntax.find(SIGNIFICANT_CHARS) {
            Some(pos) => pos,
            None => {
                let s = std::mem::take(&mut self.syntax);
                // No extra handling is required because Space is a significant character so it
                // cannot be present in the None branch.
                match deduce_literal(s) {
                    Ok(NaughtyLiteral::Literal(lit)) => {
                        return Some(Ok(SyntaxElement::Literal(lit)));
                    }
                    Ok(NaughtyLiteral::EnumSubs(enum_subs)) => {
                        return Some(Ok(SyntaxElement::new_enum_substitutions(enum_subs)));
                    }
                    Ok(NaughtyLiteral::Bool(b)) => {
                        return Some(Ok(SyntaxElement::MaybeBool(b)));
                    }
                    Err(e) => return Some(Err(e)),
                }
            }
        };

        if pos != 0 {
            let mut ends_with_space = false;

            let space_offset = ' '.len_utf8();
            if self.syntax[pos..].starts_with(' ') || self.syntax[pos..].starts_with('=') {
                pos += space_offset;
                ends_with_space = true;
            }

            match deduce_literal(&self.syntax[..pos]) {
                Ok(NaughtyLiteral::Literal(literal)) => {
                    let (_, unparsed) = self.syntax.split_at(pos);
                    self.syntax = unparsed;

                    Some(Ok(SyntaxElement::Literal(literal)))
                }
                Ok(special) => {
                    if ends_with_space {
                        pos -= space_offset;
                    }

                    let (_, unparsed) = self.syntax.split_at(pos);
                    self.syntax = unparsed;

                    match special {
                        NaughtyLiteral::EnumSubs(enum_subs) => {
                            Some(Ok(SyntaxElement::new_enum_substitutions(enum_subs)))
                        }
                        NaughtyLiteral::Bool(b) => Some(Ok(SyntaxElement::MaybeBool(b))),
                        NaughtyLiteral::Literal(_) => unreachable!(),
                    }
                }
                Err(e) => {
                    self.syntax = "";
                    Some(Err(e))
                }
            }
        } else if self.syntax.starts_with(' ') {
            let end = self.syntax.find(|c: char| !c.is_whitespace())?;
            let (spaces, unparsed) = self.syntax.split_at(end);
            self.syntax = unparsed;

            Some(Ok(SyntaxElement::Literal(spaces)))
        } else if self.syntax.starts_with('=') {
            let err = Err(format!(
                "'=' is expected to be captured by the literals. Got uncaptured '=' at `{}`",
                self.syntax
            ));
            self.syntax = "";
            Some(err)
        } else if self.syntax.starts_with('[') {
            let end = self.syntax.find(']').unwrap();
            let inner = &self.syntax['['.len_utf8()..end];
            self.syntax = &self.syntax[end + ']'.len_utf8()..];

            Some(Ok(SyntaxElement::Optional { unparsed: inner }))
        } else if self.syntax.starts_with('<') {
            let end = self.syntax.find('>').unwrap();
            let inner = &self.syntax['<'.len_utf8()..end];
            self.syntax = &self.syntax[end + '>'.len_utf8()..];

            if let Some(name) = inner.strip_prefix("str(") {
                let member_name = name.strip_suffix(')').unwrap();
                Some(Ok(SyntaxElement::DelegateMemberSubstitution {
                    member_name,
                }))
            } else if let Some(name) = inner.strip_prefix("json(") {
                let member_name = name.strip_suffix(')').unwrap();
                Some(Ok(SyntaxElement::JsonMemberSubstitution { member_name }))
            } else if let Some(member_name) = inner.strip_suffix("[0]") {
                let Some(mut delim_beg) = self.syntax.find('[') else {
                    return Some(Err(format!(
                        "Failed to find array continuation(`[<delim><member_name[1]>...]`) by looking ahead for '[' at {}",
                        self.syntax
                    )));
                };
                let Some(delim_end) = self.syntax[delim_beg..].find('<') else {
                    return Some(Err(format!(
                        "Failed to delimiter boundary in array conitnuation (`[<delim><member_name[1]>...]`) by looking ahead for '<' at {}",
                        self.syntax
                    )));
                };

                delim_beg += '['.len_utf8();

                let delim = &self.syntax[delim_beg..delim_end];
                let mut new_end = self.syntax.find("...]").unwrap();
                new_end += "...]".len();
                self.syntax = &self.syntax[new_end..];

                Some(Ok(SyntaxElement::VecMemberSubstitution {
                    member_name,
                    delim,
                }))
            } else {
                Some(Ok(SyntaxElement::TrivialMemberSubstitution {
                    member_name: inner,
                }))
            }
        } else {
            let err = Err(format!("Unexpected syntax at: {}", self.syntax));
            self.syntax = "";
            Some(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn lexer_simple1() {
        let syntax = "/_address <userId>";
        let tokens: Vec<_> = lex(syntax).collect();

        expect![[r#"
            [
                Ok(
                    Literal(
                        "/_address ",
                    ),
                ),
                Ok(
                    TrivialMemberSubstitution {
                        member_name: "userId",
                    },
                ),
            ]
        "#]]
        .assert_debug_eq(&tokens);
    }

    #[test]
    fn lexer_messy() {
        let syntax = "(_support[:<groupMemberId_>])";
        let tokens: Vec<_> = lex(syntax).collect();
        expect![[r#"
            [
                Ok(
                    Literal(
                        "(_support",
                    ),
                ),
                Ok(
                    Optional {
                        unparsed: ":<groupMemberId_>",
                    },
                ),
                Ok(
                    Literal(
                        ")",
                    ),
                ),
            ]
        "#]]
        .assert_debug_eq(&tokens);

        let nested: Vec<_> = tokens
            .iter()
            .filter_map(|tok| tok.as_ref().unwrap().optional())
            .map(|subsyntax| lex(subsyntax).collect::<Vec<_>>())
            .collect();

        expect![[r#"
            [
                [
                    Ok(
                        Literal(
                            ":",
                        ),
                    ),
                    Ok(
                        TrivialMemberSubstitution {
                            member_name: "groupMemberId_",
                        },
                    ),
                ],
            ]
        "#]]
        .assert_debug_eq(&nested);
    }

    #[test]
    fn lexer_moderate1() {
        let syntax = "full|entity|messages[ notify=off]";

        let tokens: Vec<_> = lex(syntax).collect();
        expect![[r#"
            [
                Ok(
                    EnumSubstitutions(
                        EnumSubstitutions(
                            "full|entity|messages",
                        ),
                    ),
                ),
                Ok(
                    Optional {
                        unparsed: " notify=off",
                    },
                ),
            ]
        "#]]
        .assert_debug_eq(&tokens);

        let nested: Vec<_> = tokens
            .iter()
            .filter_map(|tok| tok.as_ref().unwrap().optional())
            .map(|subsyntax| lex(subsyntax).collect::<Vec<_>>())
            .collect();

        expect![[r#"
            [
                [
                    Ok(
                        Literal(
                            " ",
                        ),
                    ),
                    Ok(
                        Literal(
                            "notify=",
                        ),
                    ),
                    Ok(
                        MaybeBool(
                            Off,
                        ),
                    ),
                ],
            ]
        "#]]
        .assert_debug_eq(&nested);
    }

    #[test]
    fn lexer_moderate2() {
        let syntax = "/_send <str(sendRef)>[ live=on][ ttl=<ttl>] json <json(composedMessages)>";
        let tokens: Vec<_> = lex(syntax).collect();

        expect![[r#"
            [
                Ok(
                    Literal(
                        "/_send ",
                    ),
                ),
                Ok(
                    DelegateMemberSubstitution {
                        member_name: "sendRef",
                    },
                ),
                Ok(
                    Optional {
                        unparsed: " live=on",
                    },
                ),
                Ok(
                    Optional {
                        unparsed: " ttl=<ttl>",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    Literal(
                        "json ",
                    ),
                ),
                Ok(
                    JsonMemberSubstitution {
                        member_name: "composedMessages",
                    },
                ),
            ]
        "#]]
        .assert_debug_eq(&tokens);

        let nested: Vec<_> = tokens
            .iter()
            .filter_map(|tok| tok.as_ref().unwrap().optional())
            .map(|s| lex(s).collect::<Vec<_>>())
            .collect();

        expect![[r#"
            [
                [
                    Ok(
                        Literal(
                            " ",
                        ),
                    ),
                    Ok(
                        Literal(
                            "live=",
                        ),
                    ),
                    Ok(
                        MaybeBool(
                            On,
                        ),
                    ),
                ],
                [
                    Ok(
                        Literal(
                            " ",
                        ),
                    ),
                    Ok(
                        Literal(
                            "ttl=",
                        ),
                    ),
                    Ok(
                        TrivialMemberSubstitution {
                            member_name: "ttl",
                        },
                    ),
                ],
            ]
        "#]]
        .assert_debug_eq(&nested);
    }

    #[test]
    fn lexer_complex1() {
        let syntax1 = "/_member role #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] observer|author|member|moderator|admin|owner";
        let syntax2 =
            "/_block #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...] blocked=on|off";

        let tokens1: Vec<_> = lex(syntax1).collect();
        let tokens2: Vec<_> = lex(syntax2).collect();

        expect![[r##"
            [
                Ok(
                    Literal(
                        "/_member ",
                    ),
                ),
                Ok(
                    Literal(
                        "role ",
                    ),
                ),
                Ok(
                    Literal(
                        "#",
                    ),
                ),
                Ok(
                    TrivialMemberSubstitution {
                        member_name: "groupId",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    VecMemberSubstitution {
                        member_name: "groupMemberIds",
                        delim: ",",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    EnumSubstitutions(
                        EnumSubstitutions(
                            "observer|author|member|moderator|admin|owner",
                        ),
                    ),
                ),
            ]
        "##]]
        .assert_debug_eq(&tokens1);

        expect![[r##"
            [
                Ok(
                    Literal(
                        "/_block ",
                    ),
                ),
                Ok(
                    Literal(
                        "#",
                    ),
                ),
                Ok(
                    TrivialMemberSubstitution {
                        member_name: "groupId",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    VecMemberSubstitution {
                        member_name: "groupMemberIds",
                        delim: ",",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    Literal(
                        "blocked=",
                    ),
                ),
                Ok(
                    MaybeBool(
                        Either,
                    ),
                ),
            ]
        "##]]
        .assert_debug_eq(&tokens2);
    }

    #[test]
    fn lexer_complex2() {
        let syntax =
            "/_remove #<groupId> <groupMemberIds[0]>[,<groupMemberIds[1]>...][ messages=on]";

        let tokens: Vec<_> = lex(syntax).collect();

        expect![[r##"
            [
                Ok(
                    Literal(
                        "/_remove ",
                    ),
                ),
                Ok(
                    Literal(
                        "#",
                    ),
                ),
                Ok(
                    TrivialMemberSubstitution {
                        member_name: "groupId",
                    },
                ),
                Ok(
                    Literal(
                        " ",
                    ),
                ),
                Ok(
                    VecMemberSubstitution {
                        member_name: "groupMemberIds",
                        delim: ",",
                    },
                ),
                Ok(
                    Optional {
                        unparsed: " messages=on",
                    },
                ),
            ]
        "##]]
        .assert_debug_eq(&tokens);

        let nested: Vec<_> = tokens
            .iter()
            .filter_map(|tok| tok.as_ref().unwrap().optional())
            .map(|s| lex(s).collect::<Vec<_>>())
            .collect();

        expect![[r#"
            [
                [
                    Ok(
                        Literal(
                            " ",
                        ),
                    ),
                    Ok(
                        Literal(
                            "messages=",
                        ),
                    ),
                    Ok(
                        MaybeBool(
                            On,
                        ),
                    ),
                ],
            ]
        "#]]
        .assert_debug_eq(&nested);
    }
}
