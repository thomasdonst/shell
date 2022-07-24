#![allow(warnings)]

use std::fs::File;
use std::process::Stdio;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Pipe,
    Next,
    NextIfSuccess,
    LogicAnd,
    LogicOr,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Cmd {
        name: String,
        arguments: Vec<String>,
        redirect: Redirect,
    },
    Binary(Box<Expr>, Operator, Box<Expr>),
    If(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Redirect {
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Redirect {
    pub fn new(stdin: Option<String>, stdout: Option<String>, stderr: Option<String>) -> Redirect {
        Redirect { stdin, stdout, stderr }
    }
}
