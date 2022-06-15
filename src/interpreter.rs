use std::process::{Command};
use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use crate::ast::{Arg, Cmd, Expr};

pub struct Interpreter {
    stdin: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn eval(&mut self, ast: &Expr) -> String {
        self.eval_binary(ast);
        self.get_result()
    }

    fn eval_binary(&mut self, node: &Expr) {
        match node {
            Expr::Pipe { left, right } => {
                self.eval_binary(left);
                self.pipe();
                self.eval_binary(right);
            }
            Expr::Cmd { ty, options, arguments } => {
                self.execute(ty, options, arguments);
            }
        }
    }

    fn get_result(&self) -> String {
        match (self.stdout.clone(), self.stderr.clone()) {
            (Some(stdout), None) => stdout,
            (None, Some(stderr)) => stderr,
            (Some(stdout), Some(stderr)) => stdout + "\r\n" + &stderr,
            (None, None) => String::from("stdout and stderr are none")
        }
    }

    fn pipe(&mut self) {
        self.stdin = self.stdout.clone();
        self.stdout = None;
    }

    fn execute(&mut self, ty: &Cmd, options: &Vec<String>, arguments: &Vec<Arg>) {
        let result = match ty {
            Cmd::Cat => self.execute_cat(options, arguments),
            Cmd::Pwd => self.execute_pwd(options),
            Cmd::Cd => self.execute_cd(options, arguments),
            Cmd::Ls => self.execute_ls(options, arguments),
            Cmd::Cp => self.execute_cp(options, arguments),
            Cmd::Mv => self.execute_mv(options, arguments)
        };

        match &result {
            Ok(s) => {
                self.stdout = Some(s.clone());
                self.stderr = None;
            }
            Err(e) => {
                self.stdout = None;
                self.stderr = Some(e.to_string());
            }
        }
    }

    fn execute_cat(&self, _options: &Vec<String>, arguments: &Vec<Arg>) -> Result<String, Error> {
        let arguments = self.replace_args_with_stdin(arguments);

        let mut result = String::from("");
        let mut args_iter = arguments.iter();

        while let Some(Arg::File(filename)) = args_iter.next() {
            let mut file = File::open(".\\".to_string() + filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            result.push_str(format!("{}\n", contents).as_str());
        }

        if result.ends_with("\n") {
            result = result[0..result.len() - 1].to_string();
        }

        Ok(result)
    }

    fn execute_pwd(&self, _options: &Vec<String>) -> Result<String, Error> {
        Ok(env::current_dir().unwrap().display().to_string())
    }

    fn execute_cd(&self, _options: &Vec<String>, arguments: &Vec<Arg>) -> Result<String, Error> {
        let arguments = self.replace_args_with_stdin(arguments);
        let dir = match arguments.last() {
            Some(Arg::Dir(d)) => Path::new(d),
            _ => Path::new(".")
        };

        env::set_current_dir(dir)?;
        Ok(String::from(""))
    }

    fn execute_ls(&self, _options: &Vec<String>, arguments: &Vec<Arg>) -> Result<String, Error> {
        let cmd = vec!["/c".to_string(), "dir /b".to_string()];
        let tmp = arguments.clone().iter().map(|x| x.to_string()).collect();
        let args = [cmd, tmp].concat();
        let output = Command::new("cmd")
            .args(args)
            .output();

        match output {
            Ok(mut x) => {
                let a = String::from_utf8_lossy(&mut x.stdout).to_string();
                Ok(a)
            }
            Err(e) => Err(e),
        }
    }

    fn execute_cp(&self, _options: &Vec<String>, _arguments: &Vec<Arg>) -> Result<String, Error> {
        unimplemented!()
    }
    fn execute_mv(&self, _options: &Vec<String>, _arguments: &Vec<Arg>) -> Result<String, Error> {
        unimplemented!()
    }

    fn replace_args_with_stdin(&self, arguments: &Vec<Arg>) -> Vec<Arg> {
        match self.stdin.clone() {
            Some(stdin) => {
                vec![Arg::from_string(&stdin)]
            }
            None => arguments.clone()
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}