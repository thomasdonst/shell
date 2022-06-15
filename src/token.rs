use std::fmt;
use crate::ast::{Arg, Cmd};

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
    Argument(Arg),
    Hyphen(String),
    DoubleHyphen(String),
    EnvVariable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}