use std::process::{Command};
use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use crate::parser::{Cmd, Expr};

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

    pub fn eval(&mut self, node: &Expr) -> String {
        match node {
            Expr::Pipe { left, right } => {
                self.eval(left);
                self.pipe();
                self.eval(right)
            }
            Expr::Cmd { ty, options, arguments } => {
                match self.execute(ty, options, arguments) {
                    Ok(s) => {
                        self.stdout = Some(s);
                        self.stderr = None;
                    }
                    Err(e) => {
                        self.stdout = None;
                        self.stderr = Some(e.to_string());
                    }
                }
                String::from("")
            }
        };

        self.get_result()
    }

    fn get_result(&self) -> String {
        match (self.stdout.clone(), self.stderr.clone()) {
            (Some(stdout), None) => stdout,
            (None, Some(stderr)) => stderr,
            (Some(stdout), Some(stderr)) => format!("{}\r\n{}", stdout, stderr),
            (None, None) => String::from("stdout and stderr are none")
        }
    }

    fn pipe(&mut self) {
        self.stdin = self.stdout.clone();
        self.stdout = None;
    }

    fn execute(&self, ty: &Cmd, options: &Vec<char>, arguments: &Vec<String>) -> Result<String, Error> {
        match ty {
            Cmd::Cat => self.execute_cat(options, arguments),
            Cmd::Pwd => self.execute_pwd(options),
            Cmd::Cd => self.execute_cd(options, arguments),
            Cmd::Ls => self.execute_ls(options, arguments),
            Cmd::Cp => self.execute_cp(options, arguments),
            Cmd::Mv => self.execute_mv(options, arguments)
        }
    }

    fn execute_cat(&self, _options: &Vec<char>, arguments: &Vec<String>) -> Result<String, Error> {
        let arguments = &self.extend_arguments(arguments.clone());

        let mut result = String::from("");
        let mut args_iter = arguments.iter();
        let current_dir = env::current_dir().unwrap().display().to_string();

        while let Some(filename) = args_iter.next() {
            let mut file = File::open(current_dir.clone() + "\\" + filename)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            result.push_str(format!("{}\n", contents).as_str());
        }

        if result.ends_with("\n") {
            result = result[0..result.len() - 1].to_string();
        }

        Ok(result)
    }

    fn execute_pwd(&self, _options: &Vec<char>) -> Result<String, Error> {
        Ok(env::current_dir().unwrap().display().to_string())
    }

    fn execute_cd(&self, _options: &Vec<char>, arguments: &Vec<String>) -> Result<String, Error> {
        let arguments = &self.extend_arguments(arguments.clone());
        let dir = match arguments.last() {
            Some(s) => Path::new(s),
            _ => Path::new(".")
        };

        env::set_current_dir(dir)?;
        Ok(String::from(""))
    }

    fn execute_ls(&self, _options: &Vec<char>, arguments: &Vec<String>) -> Result<String, Error> {
        let cmd = vec!["/c".to_string(), "dir /b".to_string()];
        let args = [cmd, arguments.clone()].concat();
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

    fn execute_cp(&self, _options: &Vec<char>, _arguments: &Vec<String>) -> Result<String, Error> {
        unimplemented!()
    }
    fn execute_mv(&self, _options: &Vec<char>, _arguments: &Vec<String>) -> Result<String, Error> {
        unimplemented!()
    }

    fn extend_arguments(&self, arguments: Vec<String>) -> Vec<String> {
        match self.stdin.clone() {
            Some(stdin) => {
                let tmp = vec![stdin];
                let result = [tmp, arguments].concat();
                result
            }
            None => arguments
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}