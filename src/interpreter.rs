#![allow(warnings)]

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, stdin, stdout, Write};
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, exit, Stdio};

use crate::ast::{Expr, Operator, Redirect};
use crate::utils::is_dir;


pub struct Interpreter {
    stderr: Option<ChildStderr>,
    stdout: Option<ChildStdout>,
    exit_success: Vec<bool>,

    is_piped: bool,

    output_result: Vec<String>,
    error_result: Vec<String>,

    program_dir: String,
}

impl Interpreter {
    pub fn new(program_path: &str) -> Interpreter {
        Interpreter {
            stderr: None,
            stdout: None,
            exit_success: vec![],

            is_piped: false,

            output_result: vec![],
            error_result: vec![],

            program_dir: program_path.to_string(),
        }
    }

    pub fn eval(&mut self, ast: &Expr) -> (Vec<String>, Vec<String>) {
        self.exit_success = vec![];
        self.output_result = vec![];
        self.error_result = vec![];

        self.eval_expr(ast);
        self.process_result();
        (self.error_result.clone(), self.output_result.clone())
    }

    fn eval_expr(&mut self, node: &Expr) {
        match node {
            Expr::Binary(lhs, Operator::Pipe, rhs) => {
                self.eval_expr(lhs);
                self.is_piped = true;
                self.eval_expr(rhs);
                self.is_piped = false;
            }
            Expr::Binary(lhs, Operator::Next, rhs) => {
                self.eval_expr(lhs);
                self.process_result();
                self.eval_expr(rhs);
                self.process_result();
            }
            Expr::Binary(lhs, Operator::NextIfSuccess, rhs) => {
                self.eval_expr(lhs);
                self.process_result();
                if *self.exit_success.last().unwrap_or(&false) {
                    self.eval_expr(rhs);
                    self.process_result();
                }
            }
            Expr::Binary(lhs, Operator::LogicOr, rhs) => {
                self.process_logic(lhs, rhs, |l, r| l || r)
            }
            Expr::Binary(lhs, Operator::LogicAnd, rhs) => {
                self.process_logic(lhs, rhs, |l, r| l && r)
            }
            Expr::If(cond, then_expr) => {
                self.eval_expr(cond);
                self.process_result();
                let condition = self.exit_success.pop().unwrap_or(false);
                if condition { self.eval_expr(then_expr) }
            }
            Expr::IfElse(cond, then_expr, else_expr) => {
                self.eval_expr(cond);
                self.process_result();
                let condition = self.exit_success.pop().unwrap_or(false);
                self.eval_expr(if condition { then_expr } else { else_expr });
            }
            Expr::Cmd { name: cmd_type, arguments, redirect } => {
                self.execute(cmd_type, arguments, &redirect);
            }
        }
    }

    fn execute(&mut self, cmd_type: &str, arguments: &Vec<String>, redirect: &Redirect) {
        match cmd_type {
            "cd" => self.cd(&arguments),
            "exit" => self.exit(),
            "set" => self.set(&arguments),
            "clear" => self.clear(),
            _ => self.execute_command(cmd_type, arguments, redirect)
        }
    }

    fn cd(&mut self, arguments: &Vec<String>) {
        let directory = match arguments.last() {
            Some(dir) => dir,
            _ => {
                self.push_error_result("Cd has no argument".to_string());
                return;
            }
        };

        if !is_dir(directory) {
            self.push_error_result(format!("{} is not a valid directory", directory));
            return;
        }

        let path = Path::new(directory);
        match env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => self.push_error_result("Could set working directory".to_string())
        }

        self.stdout = None;
        self.stderr = None;
    }

    fn exit(&self) {
        exit(0)
    }

    fn set(&mut self, arguments: &Vec<String>) {
        if arguments.len() < 2 {
            self.push_error_result(format!("Expected at least 2 arguments but found {}", arguments.len()));
            return;
        }
        let valid_key = arguments[0].chars().into_iter().all(|c| c.is_alphabetic() || c == '_');
        if !valid_key {
            self.push_error_result(format!("An environment variable can only contain \
            alphabetic characters or _ but found {}", arguments[0]));
            return;
        }
        let key = arguments[0].to_string();
        let value = arguments[1..].join(" ");
        env::set_var(key, value)
    }

    fn clear(&self) {
        Command::new("powershell").arg("cls").output().unwrap();
    }

    fn execute_command(&mut self, program_name: &str, arguments: &Vec<String>, redirect: &Redirect) {
        let program_path = self.program_dir.clone() + program_name;
        let mut command = Command::new(&program_path);
        command
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if self.is_piped {
            command = self.pipe_prev_stdout_to_stdin(command)
        }
        if let Some(filename) = &redirect.stdin {
            command = self.redirect_file_to_stdin(command, filename)
        }
        if let Some(filename) = &redirect.stdout {
            command = self.redirect_stdout_to_file(command, filename)
        }
        if let Some(filename) = &redirect.stderr {
            command = self.redirect_stderr_to_file(command, filename)
        }

        match command.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => {
                        self.exit_success.push(status.success())
                    }
                    Err(_) => self.push_error_result("Command was not running".to_string())
                }
                self.stderr = child.stderr;
                self.stdout = child.stdout;
            }
            Err(e) => self.push_error_result(format!("{}\r\n{}", e.to_string(), &program_path))
        };
    }

    fn pipe_prev_stdout_to_stdin(&mut self, mut command: Command) -> Command {
        match self.stdout.take() {
            Some(stdout) => {
                let prev_stdout = Stdio::from(stdout);
                command.stdin(prev_stdout);
            }
            None => { command.stdin(Stdio::null()); }
        }
        command
    }

    fn redirect_file_to_stdin(&mut self, mut command: Command, filename: &str) -> Command {
        if filename == "/dev/null" {
            command.stdin(Stdio::null());
            return command;
        }
        if !Path::new(filename).is_file() {
            self.push_error_result(format!("File does not exist: {}", filename));
            command.stdin(Stdio::null());
            return command;
        }
        match File::open(filename) {
            Ok(file) => { command.stdin(file); }
            Err(_) => self.push_error_result(format!("Could not read file: {}", filename))
        };
        command
    }

    fn redirect_stdout_to_file(&mut self, mut command: Command, filename: &str) -> Command {
        if filename == "/dev/null" {
            command.stdout(Stdio::null());
            return command;
        }
        match File::create(filename) {
            Ok(file) => { command.stdout(file); }
            Err(_) => self.push_error_result(format!("Could not create file: {}", filename))
        };
        command
    }

    fn redirect_stderr_to_file(&mut self, mut command: Command, filename: &str) -> Command {
        if filename == "/dev/null" {
            command.stderr(Stdio::null());
            return command;
        }
        match File::create(filename) {
            Ok(file) => { command.stderr(file); }
            Err(_) => self.push_error_result(format!("Could not create file: {}", filename))
        };
        command
    }

    fn process_result(&mut self) {
        if let Some(mut stderr) = self.stderr.take() {
            if let Ok(buffer) = self.get_buffer(&mut stderr as &mut dyn Read) {
                self.push_error_result(buffer);
            }
        }
        if let Some(mut stdout) = self.stdout.take() {
            if let Ok(buffer) = self.get_buffer(&mut stdout as &mut dyn Read) {
                self.push_output_result(buffer);
            }
        }
    }

    fn get_buffer(&mut self, fd: &mut dyn Read) -> Result<String, ()> {
        let mut buffer = String::new();
        fd.read_to_string(&mut buffer).expect("Could not read buffer from file descriptor");
        if !buffer.is_empty() { Ok(buffer) } else { Err(()) }
    }

    fn push_output_result(&mut self, buffer: String) {
        self.output_result.push(buffer.trim().to_string())
    }

    fn push_error_result(&mut self, buffer: String) {
        self.error_result.push(buffer.trim().to_string())
    }

    fn process_logic(&mut self, lhs: &Expr, rhs: &Expr, logic: fn(bool, bool) -> bool) {
        self.eval_expr(lhs);
        self.process_result();
        let left = self.exit_success.pop().unwrap_or(false);

        self.eval_expr(rhs);
        self.process_result();
        let right = self.exit_success.pop().unwrap_or(false);

        self.exit_success.push(logic(left, right));
    }
}