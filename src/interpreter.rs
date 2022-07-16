#![allow(warnings)]

use std::{env, io};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, Write};
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, ExitCode, ExitStatus, Stdio};
use std::time::Duration;
use crate::ast::{Operator, Expr};
use crate::token::Token;
use crate::utils::is_dir;

pub struct Interpreter {
    stderr: Option<ChildStderr>,
    stdout: Option<ChildStdout>,
    exit_success: bool,

    is_piped: bool,
    // file: Option<File>,

    program_dir: String,
}

impl Interpreter {
    pub fn new(program_path: &str) -> Interpreter {
        Interpreter {
            stderr: None,
            stdout: None,
            exit_success: false,

            is_piped: false,
            // file: None,

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
                return;
            }
            Expr::Binary(lhs, Operator::Next, rhs) => {
                self.eval_binary(lhs);
                self.print_result();
                self.eval_binary(rhs);
                self.print_result();
            }
            Expr::Binary(lhs, Operator::And, rhs) => {
                self.eval_binary(lhs);
                self.print_result();
                if self.exit_success { self.eval_binary(rhs) }
                self.print_result();
            }
            Expr::Cmd { program: cmd_type, arguments } => {
                self.execute(cmd_type, arguments);
            }
            Expr::Binary(lhs, Operator::InputRedirect, rhs) => {
                // todo: implement InputRedirect
                println!("InputRedirect")
            }
            Expr::Binary(lhs, Operator::OutputRedirect, rhs) => {
                // todo: implement OutputRedirect
                println!("OutputRedirect")
            }
            Expr::Argument(arg) => {
                // todo: handle if expression at this branch (maybe)
                eprintln!("Expected a command")
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
            Ok(mut c) => {
                match c.wait() {
                    Ok(status) => self.exit_success = status.success(),
                    Err(_) => eprintln!("command wasn't running")
                }
                self.stderr = c.stderr;
                self.stdout = c.stdout;
            }
            Err(e) => eprintln!("{}\r\n{}", e.to_string(), &program_path)
        };
    }

    fn print_result(&mut self) {
        if let Some(mut stderr) = self.stderr.take() {
            let stderr_buffer = self.get_buffer(&mut stderr as &mut dyn Read);
            if !stderr_buffer.is_empty() {
                eprint!("{}", stderr_buffer);
            }
        }
        if let Some(mut stdout) = self.stdout.take() {
            let stdout_buffer = self.get_buffer(&mut stdout as &mut dyn Read);
            if !stdout_buffer.is_empty() {
                print!("{}", stdout_buffer);
            }
        }
    }

    fn get_buffer(&mut self, fd: &mut dyn Read) -> String {
        let mut buffer = String::new();
        let _ = fd.read_to_string(&mut buffer);
        if !buffer.is_empty() {
            if !buffer.ends_with("\n") {
                buffer.push_str("\n")
            }
        }
        buffer
    }
}