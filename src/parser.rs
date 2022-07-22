#![allow(warnings)]

use std::env;
use std::env::VarError;
use std::fs::File;
use std::iter::Peekable;
use std::ops::Deref;
use std::path::Path;

use crate::ast::{Expr, Operator, Redirect};
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
        self.parse_expr(0)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek().take()
    }

    fn parse_expr(&mut self, min_binding_power: u8) -> Result<Expr, String> {
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
            let rhs = self.parse_expr(right_bp)?;
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(Token::Command(cmd_type)) => self.parse_command(&cmd_type),
            Some(Token::If) => self.parse_if(),
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
            Operator::LogicOr => (3, 4),
            Operator::LogicAnd => (4, 5),
            Operator::Pipe => (5, 6),
        }
    }

    fn parse_command(&mut self, cmd_type: &str) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        let mut redirect = Redirect::new(None, None, None);

        while let Some(x) = self.peek() {
            match x {
                Token::Command(arg) |
                Token::Hyphen(arg) |
                Token::DoubleHyphen(arg) |
                Token::Argument(arg) => {
                    arguments.push(arg.to_string());
                    self.next();
                }
                Token::EnvVariable(env) => {
                    match env::var(env) {
                        Ok(arg) => { arguments.push(arg); }
                        Err(_) => return Err(format!("{} is not a valid environment variable", env))
                    }
                    self.next();
                }
                Token::InputRedirect(filename) => {
                    if redirect.stdin.is_some() {
                        return Err("Only one input redirection per command is allowed".to_string());
                    }
                    redirect.stdin = Some(Self::parse_redirect(filename)?);
                    self.next();
                }
                Token::OutputRedirect(filename) => {
                    if redirect.stdout.is_some() {
                        return Err("Only one output redirection per command is allowed".to_string());
                    }
                    redirect.stdout = Some(Self::parse_redirect(filename)?);
                    self.next();
                }
                Token::ErrorRedirect(filename) => {
                    if redirect.stderr.is_some() {
                        return Err("Only one error redirection per command is allowed".to_string());
                    }
                    redirect.stderr = Some(Self::parse_redirect(filename)?);
                    self.next();
                }
                _ => break
            }
        }
        Ok(Expr::Cmd {
            name: cmd_type.to_string(),
            arguments,
            redirect,
        })
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        let cond = self.parse_expr(0)?;
        self.expect(vec![Token::Then])?;
        let then_expr = self.parse_expr(0)?;

        if self.expect(vec![Token::Else, Token::DoubleSemicolon, Token::EOL])? == Token::Else {
            let else_expr = self.parse_expr(0)?;
            self.expect(vec![Token::DoubleSemicolon, Token::EOL])?;
            Ok(Expr::IfElse(
                Box::new(cond),
                Box::new(then_expr),
                Box::new(else_expr),
            ))
        } else {
            Ok(Expr::If(Box::new(cond), Box::new(then_expr)))
        }
    }

    fn parse_redirect(filename: &str) -> Result<String, String> {
        if filename.is_empty() {
            return Err("Expected a file but found nothing".to_string());
        }
        Ok(filename.to_string())
    }

    fn expect(&mut self, should: Vec<Token>) -> Result<Token, String> {
        match self.peek() {
            Some(is) if should.iter().any(|should| is == should) =>
                if is == &Token::EOL { Ok(Token::EOL) } else { Ok(self.next().unwrap()) }
            Some(is) => Err(format!("Expected one of: {:?} but found: {:?}", should, is)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}