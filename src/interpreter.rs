use std::env;
use std::io::Read;
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use crate::ast::{Cmd, Expr};

pub struct Interpreter {
    pipe: bool,
    prev_result: Option<ChildStdout>,

    working_directory: String
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            pipe: false,
            prev_result: None,
            working_directory: env::current_dir().unwrap().display().to_string()
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
                self.pipe = true;
                self.eval_binary(right)
            }
            Expr::Cmd { ty, options, arguments } => {
                self.execute(ty, options, arguments)
            }
        }
    }

    fn execute(&mut self, ty: &Cmd, options: &Vec<String>, arguments: &Vec<String>) {
        match ty {
            Cmd::Cd => self.execute_cd(options, arguments),
            _ => {
                let args = [options.clone(), arguments.clone()].concat();
                self.spawn_process(&ty.to_string(), &args);
            }
        }
    }

    fn execute_cd(&self, _options: &Vec<String>, arguments: &Vec<String>) {
        // todo: implement options
        // let arguments = self.replace_args_with_stdin(arguments);
        let directory = match arguments.last() {
            Some(dir) => dir,
            _ => {
                println!("Cd has no argument");
                return;
            }
        };

        if !self.is_dir(directory) {
            println!("{} is not a valid directory", directory);
            return;
        }

        let path = Path::new(directory);
        match env::set_current_dir(path) {
            Ok(_) => (),
            Err(_) => println!("Could set working directory")
        }
    }

    fn spawn_process(&mut self, cmd_type: &str, arguments: &Vec<String>) {
        let program = self.working_directory.clone() + "/bin/" + cmd_type;

        let stdin = match self.pipe {
            true => {
                self.pipe = false;
                match self.prev_result.take() {
                    Some(c) => Stdio::from(c),
                    None => {
                        println!("No input");
                        Stdio::inherit()
                    }
                }
            }
            false => Stdio::inherit()
        };

        let process = Command::new(program)
            .args(arguments)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .spawn();

        match process {
            Ok(c) => self.prev_result = c.stdout,
            Err(e) => println!("error: {}", e.to_string())
        };
    }

    fn get_result(&mut self) -> String {
        let result = match self.prev_result.take() {
            Some(mut stdout) => {
                let mut buffer = String::new();
                let _ = stdout.read_to_string(&mut buffer);
                buffer
            }
            None => "None".to_string()
        };
        result
    }

    fn is_file(&self, file: &str) -> bool {
        Path::new(file).is_file()
    }

    fn is_dir(&self, dir: &str) -> bool {
        Path::new(dir).is_dir()
    }
}