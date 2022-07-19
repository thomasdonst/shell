#![allow(warnings)]

use std::fs::File;
use std::process::Stdio;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Pipe,
    Next,
    NextIfSuccess,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Cmd {
        name: String,
        arguments: Vec<String>,
        stdin_redirect: Option<String>,
        stdout_redirect: Option<String>,
    },
    Binary(Box<Expr>, Operator, Box<Expr>),
}