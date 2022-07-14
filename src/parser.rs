#![allow(warnings)]

use std::iter::Peekable;
use crate::ast::{Operator, Expr};
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
        self.parse_binary(0)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek().take()
    }

    fn parse_binary(&mut self, min_binding_power: u8) -> Result<Expr, String> {
        let mut lhs = self.parse_command()?;
        loop {
            let op;
            match self.peek_operator() {
                Ok(o) => op = o,
                Err(_) => break
            }

            let (left_bp, right_bp) = self.get_binding_power(&op);
            if min_binding_power > left_bp {
                break;
            }
            self.next();
            let rhs = self.parse_binary(right_bp)?;
            lhs = Expr::Binary(
                Box::new(lhs),
                op,
                Box::new(rhs),
            );
        }
        Ok(lhs)
    }

    fn peek_operator(&mut self) -> Result<Operator, ()> {
        match self.peek() {
            Some(Token::Semicolon) => Ok(Operator::Next),
            Some(Token::Pipe) => Ok(Operator::Pipe),
            _ => Err(())
        }
    }

    fn get_binding_power(&mut self, op: &Operator) -> (u8, u8) {
        match op {
            Operator::Next => (1, 2),
            Operator::Pipe => (2, 3)
        }
    }


    fn parse_command(&mut self) -> Result<Expr, String> {
        let command_type = match self.next() {
            Some(Token::Command(cmd)) => cmd,
            Some(x) => return Err(x.to_string() + " is not a valid command"),
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
                Token::Hyphen(s) => {
                    arguments.push("-".to_string() + s);
                    self.next();
                }
                Token::DoubleHyphen(s) => {
                    arguments.push("--".to_string() + s);
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

    // fn expect_token(&mut self, should: &Token) -> bool {
    //     match self.peek() {
    //         Some(is) => is == should,
    //         None => false
    //     }
    // }
    //
    // fn expect_tokens(&mut self, should: Vec<Token>) -> bool {
    //     match self.peek() {
    //         Some(is) => should.iter().any(|should| is == should),
    //         None => false
    //     }
    // }
}