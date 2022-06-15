use shell::interpreter::Interpreter;
use shell::lexer::Lexer;
use shell::parser::Parser;

fn main() {
    let mut lexer = Lexer::new("cat /tests/files/TMP.txt | cat");
    // println!("{:?}", lexer.get_all_tokens());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    match &ast {
        Ok(x) => {
            let mut interpreter = Interpreter::new();
            println!("{}", interpreter.eval(x));
        }
        Err(s) => println!("{:?}", s)
    };
}