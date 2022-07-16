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
        let mut lhs = self.parse_atom()?;
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

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(Token::Command(cmd_type)) => {
                Ok(
                    Expr::Cmd {
                        program: cmd_type,
                        arguments: self.parse_args(),
                    }
                )
            }
            Some(Token::Argument(arg)) => Ok(Expr::Argument(arg.to_string())),
            _ => Err("Expected a command".to_string()),
        }
    }

    fn peek_operator(&mut self) -> Result<Operator, ()> {
        match self.peek() {
            Some(Token::Semicolon) => Ok(Operator::Next),
            Some(Token::Pipe) => Ok(Operator::Pipe),
            Some(Token::DoubleAmpersand) => Ok(Operator::And),
            Some(Token::Great) => Ok(Operator::OutputRedirect),
            Some(Token::Less) => Ok(Operator::InputRedirect),
            _ => Err(())
        }
    }

    fn get_binding_power(&mut self, op: &Operator) -> (u8, u8) {
        match op {
            Operator::Next => (1, 2),
            Operator::And => (3, 4),
            Operator::Pipe => (4, 5),
            Operator::OutputRedirect => (5, 6),
            Operator::InputRedirect => (5, 6),
        }
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
}