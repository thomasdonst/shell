use shell::interpreter::Interpreter;
use shell::lexer::Lexer;
use shell::parser::Parser;

fn main() {
    let own_programs = "/target/debug/".to_string();
    let git_programs = "/bin/".to_string();

    let mut lexer = Lexer::new("cd .. | cat tests/tmp.txt -e | cd ..");
    // println!("{:?}", lexer.get_all_tokens());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    match &ast {
        Ok(expr) => {
            let mut interpreter = Interpreter::new(own_programs);
            println!("{}", interpreter.eval(expr));
        }
        Err(s) => println!("{:?}", s)
    };
}