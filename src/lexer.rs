use std::iter::Peekable;
use std::str::Chars;
use crate::ast::Cmd::*;
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

            Some('"') => Some(Token::DoubleQuote),

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
                match word.to_lowercase().as_str() {
                    "cat" => Some(Token::Command(Cat)),
                    "pwd" => Some(Token::Command(Pwd)),
                    "wc" => Some(Token::Command(Wc)),
                    "cd" => Some(Token::Command(Cd)),
                    "ls" => Some(Token::Command(Ls)),
                    "cp" => Some(Token::Command(Cp)),
                    "mv" => Some(Token::Command(Mv)),
                    "echo" => Some(Token::Command(Echo)),
                    "mkdir" => Some(Token::Command(Mkdir)),
                    "grep" => Some(Token::Command(Grep)),
                    str => Some(Token::Argument(str.to_string()))
                }
            }

            None => None
        };
        self.consume_whitespaces();
        token
    }
}