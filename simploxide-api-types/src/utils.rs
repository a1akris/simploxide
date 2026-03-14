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
