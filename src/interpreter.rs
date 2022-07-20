#![allow(warnings)]

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, exit, Stdio};

use crate::ast::{Expr, Operator};
use crate::utils::is_dir;

pub struct Interpreter {
    stderr: Option<ChildStderr>,
    stdout: Option<ChildStdout>,

    exit_success: bool,
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

            exit_success: false,
            is_piped: false,

            output_result: vec![],
            error_result: vec![],

            program_dir: program_path.to_string(),
        }
    }

    pub fn eval(&mut self, ast: &Expr) -> (Vec<String>, Vec<String>) {
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
                if self.exit_success {
                    self.eval_expr(rhs);
                    self.process_result();
                }
            }
            Expr::Binary(lhs, Operator::LogicAnd, rhs) => { //TODO: can be done with global variable
                self.eval_expr(lhs);
                if let Some(mut stdout) = self.stdout.take() {
                    let buffer = self.get_buffer(&mut stdout as &mut dyn Read);
                    let left = buffer.unwrap().trim() == "true";
                    if left {
                        self.eval_expr(rhs);
                        if let Some(mut stdout) = self.stdout.take() {
                            let buffer = self.get_buffer(&mut stdout as &mut dyn Read);
                            let right = buffer.unwrap().trim() == "true";
                            if right {
                                self.output_result.push("true".to_string());
                            } else {
                                self.output_result.push("false".to_string());
                            }
                        }
                    }
                }
            }
            Expr::Binary(lhs, Operator::LogicOr, rhs) => {
                let mut buffer_left: String;
                let mut buffer_right: String;
                let mut left = false;
                let mut right = false;
                self.eval_expr(lhs);
                if let Some(mut stdout) = self.stdout.take() {
                    buffer_left = self.get_buffer(&mut stdout as &mut dyn Read).unwrap();
                } else {
                    buffer_left = self.output_result.last().unwrap().clone();
                }
                left = buffer_left.trim() == "true";

                self.eval_expr(rhs);
                if let Some(mut stdout) = self.stdout.take() {
                    buffer_right = self.get_buffer(&mut stdout as &mut dyn Read).unwrap();
                } else {
                    buffer_right = self.output_result.last().unwrap().clone();
                }
                right = buffer_right.trim() == "true";
                if left | right {
                    self.output_result.push("true".to_string());
                } else {
                    self.output_result.push("false".to_string());
                }
            }
            Expr::If(cond, then_expr) => {
                self.eval_expr(cond);
                if let Some(mut stdout) = self.stdout.take() {
                    let mut buffer: String;
                    if let Some(mut stdout) = self.stdout.take() {
                        buffer = self.get_buffer(&mut stdout as &mut dyn Read).unwrap();
                    } else {
                        buffer = self.output_result.last().unwrap().clone();
                    }
                    let condition = buffer.trim() == "true";
                    if condition {
                        self.eval_expr(then_expr);
                    }
                }
            }
            Expr::IfElse(cond, then_expr, else_expr) => {
                self.eval_expr(cond);
                let mut buffer: String;
                if let Some(mut stdout) = self.stdout.take() {
                    buffer = self.get_buffer(&mut stdout as &mut dyn Read).unwrap();
                } else {
                    buffer = self.output_result.last().unwrap().clone();
                }
                let condition = buffer.trim() == "true";
                self.eval_expr(if condition { then_expr } else { else_expr });
            }
            Expr::Cmd {
                name: cmd_type, arguments,
                stdin_redirect, stdout_redirect
            } => {
                self.execute(cmd_type, arguments, stdin_redirect, stdout_redirect);
            }
        }
    }

    fn execute(&mut self, cmd_type: &str, arguments: &Vec<String>,
               stdin_redirect: &Option<String>, stdout_redirect: &Option<String>) {
        match cmd_type {
            "cd" => self.cd(arguments),
            "clear" => self.clear(),
            "exit" => self.exit(),
            _ => self.execute_program(cmd_type, arguments, stdin_redirect, stdout_redirect)
        }
    }

    fn cd(&mut self, arguments: &Vec<String>) {
        if self.is_piped {
            self.stdout = None;
            self.stderr = None;
            return;
        }
        // todo: implement options
        let directory = match arguments.last() {
            Some(dir) => dir,
            _ => {
                self.error_result.push("Cd has no argument".to_string());
                return;
            }
        };

        if !is_dir(directory) {
            self.error_result.push(format!("{} is not a valid directory", directory));
            return;
        }

        let path = Path::new(directory);
        match env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => self.error_result.push("Could set working directory".to_string())
        }

        self.stdout = None;
        self.stderr = None;
    }

    fn clear(&mut self) {
        self.output_result.push("\x1b[2J\x1b[1;1H".to_string())
    }

    fn exit(&self) {
        exit(0)
    }

    fn execute_program(&mut self, program_name: &str, arguments: &Vec<String>,
                       stdin_redirect: &Option<String>, stdout_redirect: &Option<String>) {
        let program_path = self.program_dir.clone() + program_name;
        let mut command = Command::new(&program_path);
        command
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if self.is_piped {
            command = self.pipe_stdout_to_stdin(command)
        }
        if let Some(filename) = stdin_redirect {
            command = self.redirect_file_to_stdin(command, filename)
        }
        if let Some(filename) = stdout_redirect {
            command = self.redirect_stdout_to_file(command, filename)
        }

        match command.spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) => self.exit_success = status.success(),
                    Err(_) => self.error_result.push("Command was not running".to_string())
                }
                self.stderr = child.stderr;
                self.stdout = child.stdout;
            }
            Err(e) => self.error_result.push(format!("{}\r\n{}", e.to_string(), &program_path))
        };
    }

    fn pipe_stdout_to_stdin(&mut self, mut program: Command) -> Command {
        match self.stdout.take() {
            Some(stdout) => {
                let prev_stdout = Stdio::from(stdout);
                program.stdin(prev_stdout);
            }
            None => { program.stdin(Stdio::null()); }
        }
        program
    }

    fn redirect_file_to_stdin(&mut self, mut program: Command, filename: &str) -> Command {
        match File::open(filename) {
            Ok(file) => { program.stdin(file); }
            Err(_) => self.error_result.push(format!("Could not read file: {}\n", filename))
        };
        program
    }

    fn redirect_stdout_to_file(&mut self, mut program: Command, filename: &str) -> Command {
        match File::create(filename) {
            Ok(file) => { program.stdout(file); }
            Err(_) => self.error_result.push(format!("Could not create file: {}\n", filename))
        };
        program
    }

    fn process_result(&mut self) {
        if let Some(mut stderr) = self.stderr.take() {
            if let Ok(buffer) = self.get_buffer(&mut stderr as &mut dyn Read) {
                self.error_result.push(buffer)
            }
        }
        if let Some(mut stdout) = self.stdout.take() {
            if let Ok(buffer) = self.get_buffer(&mut stdout as &mut dyn Read) {
                self.output_result.push(buffer)
            }
        }
    }

    fn get_buffer(&mut self, fd: &mut dyn Read) -> Result<String, ()> {
        let mut buffer = String::new();
        fd.read_to_string(&mut buffer).expect("Could not read buffer from file descriptor");
        if !buffer.is_empty() {
            if !buffer.ends_with("\n") {
                buffer.push_str("\n")
            }
            Ok(buffer)
        } else {
            Err(())
        }
    }
}