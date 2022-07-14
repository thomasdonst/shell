#![allow(warnings)]


use std::{env, io, thread};
use std::fs::File;
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use std::time::Duration;
use crate::ast::{Operator, Expr};
use crate::token::Token;

pub struct Interpreter {
    stderr: Option<ChildStderr>,
    stdout: Option<ChildStdout>,
    is_piped: bool,
    successful_command: bool,

    program_dir: String,
}

impl Interpreter {
    pub fn new(program_path: &str) -> Interpreter {
        Interpreter {
            stderr: None,
            stdout: None,
            is_piped: false,
            successful_command: false,

            program_dir: program_path.to_string(),
        }
    }

    pub fn eval(&mut self, ast: &Expr) {
        self.eval_binary(ast);
        self.print_result()
    }


    fn eval_binary(&mut self, node: &Expr) {
        match node {
            Expr::Binary(lhs, Operator::Pipe, rhs) => {
                self.eval_binary(lhs);
                self.is_piped = true;
                self.eval_binary(rhs);
                self.is_piped = false;
            }
            Expr::Binary(lhs, Operator::Next, rhs) => {
                self.eval_binary(lhs);
                self.print_result();
                self.eval_binary(rhs);
                self.print_result();
            }
            // todo: implement and
            // Expr::Binary(lhs, Operator::And, rhs) => {
            //     self.eval_binary(lhs);
            //     if self.successful_command {
            //         self.eval_binary(rhs);
            //     }
            // }
            Expr::Cmd { program: cmd_type, arguments } => {
                self.execute(cmd_type, arguments);
            }
        }
    }

    fn execute(&mut self, cmd_type: &str, arguments: &Vec<String>) {
        match cmd_type {
            "cd" => self.execute_cd(arguments),
            "clear" => self.execute_clear(),
            _ => self.execute_program(cmd_type, arguments)
        }
    }

    fn execute_cd(&mut self, arguments: &Vec<String>) {
        if self.is_piped {
            self.stdout = None;
            self.stderr = None;
            return;
        }
        // todo: implement options
        let directory = match arguments.last() {
            Some(dir) => dir,
            _ => {
                eprintln!("Cd has no argument");
                return;
            }
        };

        if !is_dir(directory) {
            eprintln!("{} is not a valid directory", directory);
            return;
        }

        let path = Path::new(directory);
        match env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => eprintln!("Could set working directory")
        }

        self.stdout = None;
        self.stderr = None;
    }

    fn execute_clear(&mut self) {
        // todo: fix bug
        print!("\x1b[2J\x1b[1;1H")
    }

    fn execute_program(&mut self, program_name: &str, arguments: &Vec<String>) {
        let program_path = self.program_dir.clone() + program_name;
        let mut command = Command::new(&program_path);

        let program = command
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if self.is_piped {
            let stdout = Stdio::from(self.stdout.take().unwrap());
            program.stdin(stdout);
        }

        match program.spawn() {
            Ok(c) => {
                self.stderr = c.stderr;
                self.stdout = c.stdout;
            }
            Err(e) => eprintln!("{}\r\n{}", e.to_string(), &program_path)
        };
    }

    fn print_result(&mut self) {
        if let Some(mut stderr) = self.stderr.take() {
            self.print_fd_buffer(&mut stderr as &mut dyn Read)
        }
        if let Some(mut stdout) = self.stdout.take() {
            self.print_fd_buffer(&mut stdout as &mut dyn Read)
        }
    }

    fn print_fd_buffer(&mut self, fd: &mut dyn Read) {
        let mut buffer = String::new();
        let _ = fd.read_to_string(&mut buffer);
        if !buffer.is_empty() {
            if !buffer.ends_with("\n") {
                buffer.push_str("\n")
            }
            print!("{}", buffer)
        }
    }
}

pub fn read_stdin() -> String {
    let mut result = String::new();
    let lines = io::stdin().lines();
    lines.for_each(|l| result.push_str(&*format!("{}\r\n", l.unwrap())));
    result[0..result.len() - 1].to_string()
}

pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn read_file(path: &str) -> Result<String, String> {
    match File::open("./".to_string() + path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                Ok(contents)
            } else {
                Err(format!("Can't read file: {}", path))
            }
        }
        Err(_) => Err(format!("Can't open file: {}", path))
    }
}

pub fn read_files(files: Vec<String>, stdout: &mut String, stderr: &mut String) {
    for file in &files {
        match read_file(&file) {
            Ok(result) => {
                stdout.push_str(&(result + "\r\n"));
            }
            Err(error) => {
                stderr.push_str(&(error + "\r\n"));
            }
        }
    }
    if stdout.len() >= 2 {
        stdout.truncate(stdout.len() - 2)
    }
    if stderr.len() >= 2 {
        stderr.truncate(stderr.len() - 2)
    }
}

pub fn process_options(options: Vec<String>, pref: Vec<char>) -> Result<Vec<char>, char> {
    let mut valid_result: Vec<char> = Vec::new();
    for option in options {
        let chars = option.chars().collect::<Vec<char>>();
        for char in chars {
            match pref.contains(&char) {
                true => valid_result.push(char),
                false => return Err(char)
            }
        }
    }
    Ok(valid_result)
}

pub fn get_args() -> Vec<String> {
    env::args().skip(1).collect::<Vec<String>>()
}