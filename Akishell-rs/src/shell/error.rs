use std::fmt;

#[derive(Debug)]
pub enum ShellError {
    // Lex(LexError),
    // Parse(ParseError),
    // Expand(ExpandError),
    Io(std::io::Error),
    CommandNotFound(String),
    Builtin(String),
}

impl ShellError {
    pub fn exit_status(&self) -> i32 {
        match self {
            ShellError::CommandNotFound(_) => 127,
            // ShellError::Parse(_) => 2,
            _ => 1,
        }
    }
}

impl From<std::io::Error> for ShellError {
    fn from(error: std::io::Error) -> Self {
        ShellError::Io(error)
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShellError::Io(error) => write!(f, "{error}"),
            ShellError::CommandNotFound(command) => write!(f, "{command}: command not found"),
            ShellError::Builtin(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ShellError {}
