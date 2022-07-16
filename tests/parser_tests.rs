#![allow(warnings)]

use shell::ast::{Expr, Operator};
use shell::ast::Expr::{Binary, Cmd};
use shell::utils::{get_program_dir, parse};

// NOTE: cat, echo, grep, fmt and seq should exist in the programs directory in order to pass this test.
// NOTE: q should NOT exist in the programs directory ...

fn parse_input(input: &str) -> Result<Expr, String> {
    let program_dir = get_program_dir();
    parse(input, &program_dir)
}

#[test]
fn parse_commands_test() {
    let expected_ast = Cmd {
        program: "cat".to_string(),
        arguments: vec!["./tests/files/tmp.txt".to_string()],
    };
    assert_eq!(parse_input("cat ./tests/files/tmp.txt").unwrap(), expected_ast);

    let expected_ast = Cmd {
        program: "echo".to_string(),
        arguments: vec!["123".to_string(), "456\n".to_string(), "abc".to_string()],
    };
    assert_eq!(parse_input("echo 123 456\n abc").unwrap(), expected_ast);
}

#[test]
fn parse_argument_test() {
    let expected_ast = Expr::Argument("q".to_string());
    assert_eq!(parse_input("q").unwrap(), expected_ast);
}

#[test]
fn parse_binary_test() {
    let expected_ast = Binary(
        Box::new(Cmd {
            program: "cat".to_string(),
            arguments: vec!["./tests/files/tmp.txt".to_string()],
        }),
        Operator::Pipe,
        Box::new(Cmd {
            program: "grep".to_string(),
            arguments: vec!["1".to_string()],
        }),
    );
    assert_eq!(parse_input("cat ./tests/files/tmp.txt | grep 1").unwrap(), expected_ast);
}

#[test]
fn parse_complex_binaries_test() {
    let expected_ast = Binary(
        Box::new(Binary(
            Box::new(Binary(
                Box::new(Cmd {
                    program: "cat".to_string(),
                    arguments: vec!["./f".to_string()],
                }),
                Operator::Pipe,
                Box::new(Cmd {
                    program: "fmt".to_string(),
                    arguments: vec![],
                }),
            )),
            Operator::And,
            Box::new(Cmd {
                program: "echo".to_string(),
                arguments: vec!["ok".to_string()],
            }),
        )),
        Operator::Next,
        Box::new(Cmd {
            program: "echo".to_string(),
            arguments: vec!["next".to_string()],
        }),
    );
    assert_eq!(parse_input("cat ./f | fmt && echo ok ; echo next").unwrap(), expected_ast);


    let expected_ast = Binary(
        Box::new(Binary(
            Box::new(Binary(
                Box::new(Cmd {
                    program: "echo".to_string(),
                    arguments: vec!["123".to_string()],
                }),
                Operator::And,
                Box::new(Binary(
                    Box::new(Binary(
                        Box::new(Cmd {
                            program: "echo".to_string(),
                            arguments: vec!["456".to_string()],
                        }),
                        Operator::Pipe,
                        Box::new(Cmd {
                            program: "grep".to_string(),
                            arguments: vec!["4".to_string()],
                        }),
                    )),
                    Operator::Pipe,
                    Box::new(Cmd {
                        program: "fmt".to_string(),
                        arguments: vec![],
                    }),
                )),
            )),
            Operator::And,
            Box::new(Cmd {
                program: "seq".to_string(),
                arguments: vec!["3".to_string()],
            }),
        )),
        Operator::Next,
        Box::new(Cmd {
            program: "echo".to_string(),
            arguments: vec![],
        }),
    );
    assert_eq!(parse_input("echo 123 && echo 456 | grep 4 | fmt && seq 3 ; echo").unwrap(), expected_ast);
}