use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use crate::ast::{CmdType, Expr};

pub struct Interpreter {
    stdout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
    is_piped: bool,

    working_dir: String,
    program_path: String,
}

impl Interpreter {
    pub fn new(program_path: String) -> Interpreter {
        Interpreter {
            stdout: None,
            stderr: None,
            is_piped: false,

            working_dir: env::current_dir().unwrap().display().to_string(),
            program_path,
        }
    }

    pub fn eval(&mut self, ast: &Expr) -> String {
        self.eval_binary(ast);
        self.get_result()
    }

    fn eval_binary(&mut self, node: &Expr) {
        match node {
            Expr::Pipe { left, right } => {
                self.is_piped = true;
                self.eval_binary(left);
                self.eval_binary(right);
                self.is_piped = false;
            }
            Expr::Cmd { program: ty, arguments } => {
                self.execute(ty, arguments)
            }
        }
    }

    fn execute(&mut self, program: &CmdType, arguments: &Vec<String>) {
        match program {
            CmdType::Cd => self.execute_cd(arguments),
            _ => self.spawn_process(&program.to_string(), arguments)
        }
    }

    fn execute_cd(&mut self, arguments: &Vec<String>) {
        if self.is_piped {
            self.stdout = None;
            self.stderr = None;
            return
        }
        // todo: implement options
        let directory = match arguments.last() {
            Some(dir) => dir,
            _ => {
                eprintln!("Cd has no argument");
                return
            }
        };

        if !is_dir(directory) {
            eprintln!("{directory} is not a valid directory");
            return
        }

        let path = Path::new(directory);
        match env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => eprintln!("Could set working directory")
        }

        self.stdout = None;
        self.stderr = None;
    }

    fn spawn_process(&mut self, program: &str, arguments: &Vec<String>) {
        let path = self.working_dir.clone() + &self.program_path + program;

        let stdin = match self.stdout.take() {
            Some(c) => Stdio::from(c),
            None => Stdio::inherit()
        };

        let process = Command::new(path)
            .args(arguments)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match process {
            Ok(c) => {
                self.stdout = c.stdout;
                self.stderr = c.stderr;
            }
            Err(e) => eprintln!("Process could not be started: {}", e.to_string())
        };
    }

    fn get_result(&mut self) -> String {
        let mut stderr_result = String::new();
        match self.stderr.take() {
            Some(mut x) => {
                let _ = x.read_to_string(&mut stderr_result);
            }
            None => {}
        }
        let mut stdout_result = String::new();
        match self.stdout.take() {
            Some(mut x) => {
                let _ = x.read_to_string(&mut stdout_result);
            }
            None => {}
        }

        stderr_result + stdout_result.as_str()
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
                Err(format!("Can't read file: {path}"))
            }
        }
        Err(_) => Err(format!("Can't open file: {path}"))
    }
}

pub fn read_files(files: Vec<String>, stdout: &mut String, stderr: &mut String) {
    for file in &files {
        match read_file(&file) {
            Ok(result) => {
                stdout.push_str(&result);
            }
            Err(error) => {
                stderr.push_str(&error);
            }
        }
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

