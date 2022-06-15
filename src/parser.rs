use std::iter::Peekable;
use crate::ast::{Arg, Expr};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'lexer> {
    lexer: Peekable<Lexer<'lexer>>,
    current: Option<Token>,
}

impl<'lexer> Parser<'lexer> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
            current: None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.current = self.lexer.next();
        self.current.clone()
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_binary()
    }

    fn peek(&mut self) -> Option<Token> {
        self.lexer.peek().cloned()
    }

    fn parse_binary(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_command()?;
        loop {
            if !self.expect_token(Token::Pipe) {
                break;
            }
            self.next();
            let rhs = self.parse_binary()?;
            lhs = Expr::Pipe {
                left: Box::new(lhs),
                right: Box::new(rhs),
            }
        }
        Ok(lhs)
    }


    fn parse_command(&mut self) -> Result<Expr, String> {
        let command_type = match self.next() {
            Some(Token::Command(cmd)) => cmd,
            Some(x) => return Err(x.to_string() + " is not a valid command"),
            // todo(): read input afterwards and then parse again
            None => return Err("Expected a command".to_string()),
        };

        let (options, arguments) = self.parse_options_and_args();

        Ok(
            Expr::Cmd {
                ty: command_type,
                options,
                arguments,
            }
        )
    }

    fn parse_options_and_args(&mut self) -> (Vec<String>, Vec<Arg>) {
        let mut options = Vec::new();
        let mut arguments = Vec::<Arg>::new();
        while let Some(x) = self.peek() {
            match x {
                Token::Command(cmd) => {
                    arguments.push(Arg::Cmd(cmd));
                    self.next();
                }
                Token::Argument(arg) => {
                    arguments.push(arg);
                    self.next();
                }
                Token::Hyphen(s) => {
                    s.chars().for_each(|c| options.push(c.to_string()));
                    self.next();
                }
                Token::DoubleHyphen(s) => {
                    options.push(s);
                    self.next();
                }
                _ => break
            }
        }

        (options, arguments)
    }

    fn expect_token(&mut self, should: Token) -> bool {
        match self.peek() {
            Some(Token::Hyphen(_)) => matches!(should, Token::Hyphen(_)),
            Some(Token::DoubleHyphen(_)) => matches!(should, Token::DoubleHyphen(_)),
            Some(Token::Argument(_)) => matches!(should, Token::Argument(_)),
            Some(Token::Command(_)) => matches!(should, Token::Command(_)),
            Some(Token::EnvVariable(_)) => matches!(should, Token::EnvVariable(_)),
            Some(is) => is == should,
            _ => false
        }
    }
}