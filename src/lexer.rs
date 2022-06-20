use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::ast::CmdType;
use crate::token::Token;

pub struct Lexer<'input> {
    input: Peekable<Chars<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Lexer {
        Lexer {
            input: input.chars().peekable()
        }
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        self.for_each(|token| tokens.push(token));
        tokens
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn next_word(&mut self, init: String) -> String {
        let mut res = init;
        while let Some(c) = self.peek() {
            if self.is_word_member(c) {
                res.push(self.next_char().unwrap())
            } else {
                break;
            }
        }
        res
    }

    fn is_word_member(&self, c: char) -> bool {
        !matches!(c, ' ' | '>' | '<' | '&' | '|' | '=' | '"' | '$' | '-')
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().cloned()
    }

    fn consume_whitespaces(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.next_char() {
            Some('>') =>
                if self.peek() == Some('>') {
                    self.input.next();
                    Some(Token::DoubleGreat)
                } else if self.peek() == Some('&') {
                    self.input.next();
                    Some(Token::GreatAmpersand)
                } else {
                    Some(Token::Great)
                },

            Some('<') =>
                if self.peek() == Some('<') {
                    self.input.next();
                    Some(Token::DoubleLess)
                } else {
                    Some(Token::Less)
                }

            Some('&') => Some(Token::Ampersand),

            Some('|') => Some(Token::Pipe),

            Some('=') => Some(Token::Equal),

            Some('"') => Some(Token::Quote),

            Some('$') => {
                let res = self.next_word("".to_string());
                Some(Token::EnvVariable(res))
            }

            Some('-') => {
                if self.peek() == Some('-') {
                    self.next_char();
                    let option = self.next_word("".to_string());
                    Some(Token::DoubleHyphen(option))
                } else {
                    let option = self.next_word("".to_string());
                    Some(Token::Hyphen(option))
                }
            }

            Some(c) => {
                let word = self.next_word(c.to_string());
                match CmdType::from_str(&word) {
                    Ok(cmd) => Some(Token::Command(cmd)),
                    Err(_) => Some(Token::Argument(word))
                }
            }

            None => None
        };
        self.consume_whitespaces();
        token
    }
}