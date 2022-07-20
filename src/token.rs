#![allow(warnings)]

use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Ampersand,
    DoubleAmpersand,
    Pipe,
    DoublePipe,
    Semicolon,
    DoubleSemicolon,
    If,
    Then,
    Else,
    EOL,

    InputRedirect(String),
    OutputRedirect(String),
    Command(String),
    Argument(String),
    Hyphen(String),
    DoubleHyphen(String),
    EnvVariable(String), // todo: implementation worth/ meaningful?
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}