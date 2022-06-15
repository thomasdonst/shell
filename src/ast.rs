use std::fmt;
use std::path::Path;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cmd {
    Cat,
    Pwd,
    Cd,
    Ls,
    Cp,
    Mv,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Arg {
    Cmd(Cmd),
    File(String),
    Dir(String),
    String(String),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Arg {
    fn is_file(file: &str) -> bool {
        let path = Path::new(file);
        if !path.exists() {
            return false;
        }
        path.is_file()
    }

    fn is_dir(dir: &str) -> bool {
        let path = Path::new(dir);
        if !path.exists() {
            return false;
        }
        path.is_dir()
    }

    pub fn from_string(s: &str) -> Arg {
        match s {
            file if Arg::is_file(file) => Arg::File(file.to_string()),
            dir if Arg::is_dir(dir) => Arg::Dir(dir.to_string()),
            str => Arg::File(str.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self.clone() {
            Arg::Cmd(cmd) => cmd.to_string(),
            Arg::File(file) => file,
            Arg::Dir(dir) => dir,
            Arg::String(str) => str
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Cmd {
        ty: Cmd,
        options: Vec<String>,
        arguments: Vec<Arg>,
    },
    Pipe {
        left: Box<Expr>,
        right: Box<Expr>,
    },
}