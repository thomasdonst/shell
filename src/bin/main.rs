#![allow(warnings)]

use std::env;
use std::io::{Read, stdin, stdout, Write};
use shell::ast::Expr;
use shell::interpreter::Interpreter;
use shell::lexer::Lexer;
use shell::parser::Parser;
use shell::utils::{get_program_dir, parse};

fn main() {
    let program_dir = get_program_dir();
    let mut interpreter = Interpreter::new(&program_dir);
    loop {
        display_prompt();
        let input = read_input();
        let ast = parse(&input, &program_dir);
        match &ast {
            Ok(expr) => {
                let (stderr, stdout) = interpreter.eval(expr);
                stderr.iter().for_each(|x| print!("{}", x));
                stdout.iter().for_each(|x| print!("{}", x));
            },
            Err(err) => eprintln!("{}", err)
        };
    }
}

fn display_prompt() {
    let cwd = env::current_dir().unwrap().display().to_string();
    print!("{}> ", cwd);
    stdout().flush().expect("Could not flush stdout")
}

fn read_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not read input");
    input.trim().to_string()
}