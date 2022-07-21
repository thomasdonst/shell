#![allow(warnings)]

use std::env;
use std::fs::OpenOptions;
use std::iter::Peekable;
use std::path::Path;
use std::str::{Chars, FromStr};

use crate::token::Token;

pub struct Lexer<'input> {
    input: Peekable<Chars<'input>>,

    program_dir: String,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str, program_dir: &str) -> Lexer<'input> {
        Lexer {
            input: input.chars().peekable(),

            program_dir: program_dir.to_string(),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
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
        !matches!(c, ' ' | '>' | '<' | '&' | '|' | '=' | '"' | '$' | '-' | ';' | '\r' | '\n')
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().cloned()
    }

    fn consume_whitespaces(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' {
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
            Some('>') => {
                self.consume_whitespaces();
                let res = self.next_word("".to_string());
                Some(Token::OutputRedirect(res))
            }

            Some('<') => {
                self.consume_whitespaces();
                let res = self.next_word("".to_string());
                Some(Token::InputRedirect(res))
            }

            Some('&') =>
                if self.peek() == Some('&') {
                    self.input.next();
                    Some(Token::DoubleAmpersand)
                } else {
                    Some(Token::Ampersand)
                }

            Some('|') =>
                if self.peek() == Some('|') {
                    self.input.next();
                    Some(Token::DoublePipe)
                } else {
                    Some(Token::Pipe)
                }

            Some(';') =>
                if self.peek() == Some(';') {
                    self.input.next();
                    Some(Token::DoubleSemicolon)
                } else {
                    Some(Token::Semicolon)
                },

            Some('-') => {
                if self.peek() == Some('-') {
                    self.next_char();
                    let option = self.next_word("--".to_string());
                    Some(Token::DoubleHyphen(option))
                } else {
                    let option = self.next_word("-".to_string());
                    Some(Token::Hyphen(option))
                }
            }

            Some('\n') => Some(Token::EOL),

            Some('\r') => {
                if self.peek() == Some('\n') {
                    self.next_char();
                    Some(Token::EOL)
                } else {
                    Some(Token::EOL)
                }
            }

            Some(c) => Some({
                let word = self.next_word(c.to_string()).to_lowercase();

                if word == "if" {
                    self.consume_whitespaces();
                    return Some(Token::If);
                }
                if word == "then" {
                    self.consume_whitespaces();
                    return Some(Token::Then);
                }
                if word == "else" {
                    self.consume_whitespaces();
                    return Some(Token::Else);
                }

                let program_path = self.program_dir.clone() + &word + ".exe";
                let built_in_shell = ["cd", "exit"].contains(&word.as_str());
                let found_program = Path::new(&program_path).is_file();
                let program_exists = found_program || built_in_shell;

                if program_exists { Token::Command(word) } else { Token::Argument(word) }
            }),

            None => None
        };
        self.consume_whitespaces();
        token
    }
}