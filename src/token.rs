#![allow(warnings)]

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ampersand,
    DoubleAmpersand,
    Pipe,
    Equal,
    Quote,
    Semicolon,

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