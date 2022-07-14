#![allow(warnings)]

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Great,
    DoubleGreat,
    GreatAmpersand,
    Less,
    DoubleLess,
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
    EnvVariable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Token {
    fn compare_type(&self, other: &Self) -> bool {
        match self {
            Token::Hyphen(_) => matches!(other, Token::Hyphen(_)),
            Token::DoubleHyphen(_) => matches!(other, Token::DoubleHyphen(_)),
            Token::Command(_) => matches!(other, Token::Command(_)),
            Token::Argument(_) => matches!(other, Token::Argument(_)),
            Token::EnvVariable(_) => matches!(other, Token::EnvVariable(_)),
            _ => self == other
        }
    }
}