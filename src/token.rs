#![allow(warnings)]

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}