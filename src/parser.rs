use std::iter::Peekable;
use crate::ast::{Expr};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'lexer> {
    lexer: Peekable<Lexer<'lexer>>,
}

impl<'lexer> Parser<'lexer> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_binary()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek().take()
    }

    fn parse_binary(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_command()?;
        loop {
            if !self.expect_token(&Token::Pipe) {
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
            // todo: read input afterwards and then parse again
            None => return Err("Expected a command".to_string()),
        };

        Ok(
            Expr::Cmd {
                program: command_type,
                arguments: self.parse_args(),
            }
        )
    }

    fn parse_args(&mut self) -> Vec<String> {
        let mut arguments = Vec::new();
        while let Some(x) = self.peek() {
            match x {
                Token::Command(cmd) => {
                    arguments.push(cmd.to_string());
                    self.next();
                }
                Token::Hyphen(s) | Token::DoubleHyphen(s) => {
                    arguments.push("-".to_string() + s);
                    self.next();
                }
                Token::Argument(s) => {
                    arguments.push(s.to_string());
                    self.next();
                }
                _ => break
            }
        }

        arguments
    }

    fn expect_token(&mut self, should: &Token) -> bool {
        match self.peek() {
            Some(is) => is == should,
            None => false
        }
    }
}