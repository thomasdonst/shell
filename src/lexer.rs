use std::iter::Peekable;
use std::str::Chars;
use log::debug;
use crate::token::Token;

pub struct Lexer<'input> {
    input: Peekable<Chars<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input: input.chars().peekable()
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().cloned()
    }

    pub fn get_all_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        loop {
            let token = self.next();
            if token == Token::EOF {
                break;
            }
            tokens.push(token.clone());
        }
        tokens
    }

    fn next(&mut self) -> Token {
        let token = match self.next_char() {
            None => Token::EOF,

            Some(' ') => {
                while self.peek() == Some(' ') {
                    self.input.next();
                }
                Token::Whitespace
            }

            Some('>') =>
                if self.peek() == Some('>') {
                    self.input.next();
                    Token::DoubleGreat
                } else if self.peek() == Some('&') {
                    self.input.next();
                    Token::GreatAmpersand
                } else {
                    Token::Great
                },

            Some('<') =>
                if self.peek() == Some('<') {
                    self.input.next();
                    Token::DoubleLess
                } else {
                    Token::Less
                }

            Some('&') => Token::Ampersand,

            Some('|') => Token::Pipe,

            Some('-') => {
                let mut res = String::from("");
                while let Some(c) = self.peek() {
                    if is_ident_member(c) {
                        res.push(self.next_char().unwrap())
                    } else {
                        break;
                    }
                }
                Token::Option(res)
            }

            Some(c) => {
                let mut res = c.to_string();
                while let Some(c) = self.peek() {
                    if !c.is_whitespace() {
                        res.push(self.next_char().unwrap())
                    } else {
                        break;
                    }
                }
                match res.as_str() {
                    "pwd" => Token::Pwd,
                    "cd" => Token::Cd,
                    "ls" => Token::Ls,
                    "cp" => Token::Cp,
                    "mv" => Token::Mv,
                    "mkdir" => Token::Mkdir,
                    "rmdir" => Token::Rmdir,
                    "rm" => Token::Rm,
                    "touch" => Token::Touch,
                    "locate" => Token::Locate,
                    "find" => Token::Find,
                    "grep" => Token::Grep,
                    "kill" => Token::Kill,
                    "ping" => Token::Ping,
                    "history" => Token::History,
                    "man" => Token::Man,
                    "echo" => Token::Echo,
                    _ => Token::Argument(res)
                }
            }
        };

        debug!("token: {:?}", token);
        token
    }
}

fn is_ident_member(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        '_' => true,
        _ => false,
    }
}











