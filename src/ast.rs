use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cmd {
    Cat,
    Pwd,
    Cd,
    Ls,
    Cp,
    Mv,
    Echo,
    Mkdir,
    Wc,
    Grep
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Expr {
    Cmd {
        ty: Cmd,
        options: Vec<String>,
        arguments: Vec<String>,
    },
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}