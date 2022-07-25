use std::env;
use std::io::{stdin, stdout, Write};
use shell::interpreter::Interpreter;
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
                stderr.iter().for_each(|x| eprintln!("{}", x));
                stdout.iter().for_each(|x| println!("{}", x));
            }
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
    input.trim_start().to_string()
}