use std::fmt;
use std::str::FromStr;
use crate::ast::CmdType::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CmdType {
    Cat,
    Pwd,
    Cd,
    Ls,
    Cp,
    Mv,
    Echo,
    Mkdir,
    Wc,
    Grep,
}

impl FromStr for CmdType {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, String> {
        match str.to_lowercase().as_str() {
            "cat" => Ok(Cat),
            "pwd" => Ok(Pwd),
            "wc" => Ok(Wc),
            "cd" => Ok(Cd),
            "ls" => Ok(Ls),
            "cp" => Ok(Cp),
            "mv" => Ok(Mv),
            "echo" => Ok(Echo),
            "mkdir" => Ok(Mkdir),
            "grep" => Ok(Grep),
            x => Err(x.to_string())
        }
    }
}

impl fmt::Display for CmdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Expr {
    Cmd {
        program: CmdType,
        arguments: Vec<String>,
    },
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}