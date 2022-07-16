#![allow(warnings)]

use std::fmt;
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Great,
    Less,
    Ampersand,
    DoubleAmpersand,
    Pipe,
    Equal,
    Quote,
    Semicolon,

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