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
