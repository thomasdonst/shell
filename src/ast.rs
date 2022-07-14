#![allow(warnings)]

use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Operator {
    Pipe,
    Next,
    // And,
}

#[derive(Debug)]
pub enum Expr {
    Cmd {
        program: String,
        arguments: Vec<String>,
    },

    Binary(Box<Expr>, Operator, Box<Expr>),
}