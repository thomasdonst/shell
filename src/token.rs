use std::fmt;
use crate::ast::{Cmd};

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
    Hyphen(String),
    DoubleHyphen(String),
    EnvVariable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}