use std::fmt;
use crate::ast::{CmdType};

#[derive(Debug)]
pub enum Token {
    Great,
    DoubleGreat,
    GreatAmpersand,
    Less,
    DoubleLess,
    Ampersand,
    Pipe,
    Equal,
    Quote,

    Command(CmdType),
    Argument(String),
    Hyphen(String),
    DoubleHyphen(String),
    EnvVariable(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl PartialEq<Self> for Token {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Token::Great => matches!(other, Token::Great),
            Token::DoubleGreat => matches!(other, Token::DoubleGreat),
            Token::GreatAmpersand => matches!(other, Token::GreatAmpersand),
            Token::Less => matches!(other, Token::Less),
            Token::DoubleLess => matches!(other, Token::DoubleLess),
            Token::Ampersand => matches!(other, Token::Ampersand),
            Token::Pipe => matches!(other, Token::Pipe),
            Token::Equal => matches!(other, Token::Equal),
            Token::Quote => matches!(other, Token::Equal),
            Token::Hyphen(_) => matches!(other, Token::Hyphen(_)),
            Token::DoubleHyphen(_) => matches!(other, Token::DoubleHyphen(_)),
            Token::Command(_) => matches!(other, Token::Command(_)),
            Token::Argument(_) => matches!(other, Token::Argument(_)),
            Token::EnvVariable(_) => matches!(other, Token::EnvVariable(_)),
        }
    }
}