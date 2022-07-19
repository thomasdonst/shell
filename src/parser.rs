#![allow(warnings)]

use std::fs::File;
use std::iter::Peekable;
use std::path::Path;

use crate::ast::{Expr, Operator};
use crate::lexer::Lexer;
use crate::token::Token;
use crate::utils::parse;

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
        self.parse_expression(0)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek().take()
    }

    fn parse_expression(&mut self, min_binding_power: u8) -> Result<Expr, String> {
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
            let rhs = self.parse_expression(right_bp)?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(Token::Command(cmd_type)) => self.parse_command(&cmd_type),
            Some(Token::If) => {
                let cond = self.parse_expression(0)?;
                self.expect(vec![Token::Then])?;
                let then_expr = self.parse_expression(0)?;
                if self.expect(vec![Token::Else, Token::DoubleSemicolon, Token::EOL])? == Token::Else {
                    let else_expr = self.parse_expression(0)?;
                    self.expect(vec![Token::DoubleSemicolon, Token::EOL])?;
                    return Ok(Expr::IfElse(
                        Box::new(cond),
                        Box::new(then_expr),
                        Box::new(else_expr),
                    ));
                }
                Ok(Expr::If(Box::new(cond), Box::new(then_expr)))
            }
            Some(token) => Err(format!("Expected a command or if but found {}", token)),
            None => Err("Expected a command or if but found nothing".to_string()),
        }
    }

    fn peek_operator(&mut self) -> Result<Operator, ()> {
        match self.peek() {
            Some(Token::Semicolon) => Ok(Operator::Next),
            Some(Token::Pipe) => Ok(Operator::Pipe),
            Some(Token::Ampersand) => Ok(Operator::NextIfSuccess),
            Some(Token::DoubleAmpersand) => Ok(Operator::LogicAnd),
            Some(Token::DoublePipe) => Ok(Operator::LogicOr),
            _ => Err(())
        }
    }

    fn get_binding_power(&mut self, op: &Operator) -> (u8, u8) {
        match op {
            Operator::Next => (1, 2),
            Operator::NextIfSuccess => (3, 4),
            Operator::LogicAnd => (3, 4),
            Operator::LogicOr => (3, 4),
            Operator::Pipe => (4, 5),
        }
    }

    fn parse_command(&mut self, cmd_type: &str) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        let mut stdin_redirect = None;
        let mut stdout_redirect = None;

        while let Some(x) = self.peek() {
            match x {
                Token::Command(arg) | Token::Hyphen(arg) |
                Token::DoubleHyphen(arg) | Token::Argument(arg) => {
                    arguments.push(arg.to_string());
                    self.next();
                }
                Token::InputRedirect(filename) => {
                    if stdin_redirect.is_some() {
                        return Err("Only one input redirection per command is allowed".to_string());
                    }
                    stdin_redirect = Some(Self::parse_redirect(filename, true)?);
                    self.next();
                }
                Token::OutputRedirect(filename) => {
                    if stdout_redirect.is_some() {
                        return Err("Only one output redirection per command is allowed".to_string());
                    }
                    stdout_redirect = Some(Self::parse_redirect(filename, false)?);
                    self.next();
                }
                _ => break
            }
        }
        Ok(Expr::Cmd {
            name: cmd_type.to_string(),
            arguments,
            stdin_redirect,
            stdout_redirect,
        })
    }

    fn parse_redirect(filename: &str, should_exist: bool) -> Result<String, String> {
        if filename.is_empty() {
            return Err("Expected a file but found nothing".to_string());
        }
        if !Path::new(filename).is_file() && should_exist {
            return Err(format!("{} does not exists", filename));
        }
        Ok(filename.to_string())
    }

    fn expect(&mut self, should: Vec<Token>) -> Result<Token, String> {
        match self.peek() {
            Some(is) if should.iter().any(|should| is == should) => Ok(self.next().unwrap()),
            Some(is) => Err(format!("Expected one of: {:?} but got: {:?}", should, is)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}