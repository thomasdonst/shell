#![allow(warnings)]

use std::fmt;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Pipe,
    Next,
    And,
    InputRedirect,
    OutputRedirect,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Cmd {
        program: String,
        arguments: Vec<String>,
    },
    Argument(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
}