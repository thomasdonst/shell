#![allow(warnings)]

use shell::ast::Expr;
use shell::interpreter::Interpreter;
use shell::utils::{get_program_dir, parse};

fn eval(input: &str) -> (Vec<String>, Vec<String>) {
    let program_dir = get_program_dir();
    let ast = match parse(&(input.to_string() + "\n"), &program_dir) {
        Ok(ast) => ast,
        Err(e) => panic!("{}", e)
    };
    let mut interpreter = Interpreter::new(&program_dir);
    interpreter.eval(&ast)
}

fn assert_output(input: &str, expected_output: Vec<String>) {
    let output = eval(input).1;
    assert_eq!(output, expected_output)
}

fn assert_error(input: &str, expected_error: Vec<String>) {
    let error = eval(input).0;
    assert_eq!(error, expected_error)
}

#[test]
fn interpret_if_test() {
    assert_output("if true then echo something", vec!["something".to_string()]);
    assert_output("if false then echo something", vec![]);
    assert_output("if true then echo true else echo false", vec!["true".to_string()]);
    assert_output("if false then echo true else echo false", vec!["false".to_string()]);
}

#[test]
fn interpret_complex_if_test() {
    assert_output("if false then echo . else if false then echo . else echo ok",
                  vec!["ok".to_string()]);
    assert_output("if true then if true then echo ok else if false then echo . else echo .",
                  vec!["ok".to_string()]);
    assert_output("if true then if true then echo ok else echo . ;; else echo .",
                  vec!["ok".to_string()]);
    assert_output("if false then if true then echo . else echo . ;; else echo ok",
                  vec!["ok".to_string()]);
}

#[test]
fn interpret_logic_test() {
    assert_output("if true || false then echo true else echo false", vec!["true".to_string()]);
    assert_output("if true || true then echo true else echo false", vec!["true".to_string()]);
    assert_output("if false || true then echo true else echo false", vec!["true".to_string()]);
    assert_output("if false || false then echo true else echo false", vec!["false".to_string()]);

    assert_output("if true && false then echo true else echo false", vec!["false".to_string()]);
    assert_output("if false && true then echo true else echo false", vec!["false".to_string()]);
    assert_output("if false && false then echo true else echo false", vec!["false".to_string()]);
    assert_output("if true && true then echo true else echo false", vec!["true".to_string()]);

    assert_output("if true && false || false && true then echo true else echo false", vec!["false".to_string()]);
    assert_output("if true && true || false && true then echo true else echo false", vec!["true".to_string()]);
    assert_output("if true && false || true && true then echo true else echo false", vec!["true".to_string()]);
    // todo: test all combinations
}
