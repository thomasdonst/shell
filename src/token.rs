use std::fmt;
use crate::parser::Cmd;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    Great,
    DoubleGreat,
    GreatAmpersand,
    Less,
    DoubleLess,
    Ampersand,
    Pipe,
    Equal,
    DoubleQuote,

    Command(Cmd),
    Argument(String),
    Option(String),
    EnvVariable(String),
    Whitespace,
    Eof
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
