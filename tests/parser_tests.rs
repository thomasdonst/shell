#![allow(warnings)]

use std::fs::File;
use shell::ast::{Expr, Operator};
use shell::ast::Expr::{Binary, Cmd};
use shell::utils::{get_program_dir, parse};
use std::sync::Once;

// NOTE: cat, echo, grep, fmt and seq should exist in the programs directory

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        File::create("input.txt").expect("Can not create input.txt");
    })
}

fn parse_input(input: &str) -> Result<Expr, String> {
    let program_dir = get_program_dir();
    parse(input, &program_dir)
}

#[test]
fn parse_commands_test() {
    let expected_ast = Cmd {
        name: "cat".to_string(),
        arguments: vec!["./tests/files/tmp.txt".to_string()],
        stdin_redirect: None,
        stdout_redirect: None,
    };
    assert_eq!(parse_input("cat ./tests/files/tmp.txt").unwrap(), expected_ast);

    let expected_ast = Cmd {
        name: "echo".to_string(),
        arguments: vec!["123".to_string(), "456".to_string(), "abc".to_string()],
        stdin_redirect: None,
        stdout_redirect: None,
    };
    assert_eq!(parse_input("echo 123 456 abc").unwrap(), expected_ast);
}

#[test]
fn parse_redirects_test() {
    let expected_ast = Cmd {
        name: "echo".to_string(),
        arguments: vec!["123".to_string(), "456".to_string(), "abc".to_string()],
        stdin_redirect: None,
        stdout_redirect: None,
    };
    assert_eq!(parse_input("echo 123 456 abc").unwrap(), expected_ast);
}

#[test]
fn parse_binary_test() {
    let expected_ast = Binary(
        Box::new(Cmd {
            name: "cat".to_string(),
            arguments: vec!["./tests/files/tmp.txt".to_string()],
            stdin_redirect: None,
            stdout_redirect: None,
        }),
        Operator::Pipe,
        Box::new(Cmd {
            name: "grep".to_string(),
            arguments: vec!["1".to_string()],
            stdin_redirect: None,
            stdout_redirect: None,
        }),
    );
    assert_eq!(parse_input("cat ./tests/files/tmp.txt | grep 1").unwrap(), expected_ast);
}

#[test]
fn parse_complex_binary_test() {
    initialize();
    let expected_ast = Binary(
        Box::new(Binary(
            Box::new(Binary(
                Box::new(Cmd {
                    name: "echo".to_string(),
                    arguments: vec!["123".to_string()],
                    stdin_redirect: None,
                    stdout_redirect: None,
                }),
                Operator::NextIfSuccess,
                Box::new(Binary(
                    Box::new(Binary(
                        Box::new(Cmd {
                            name: "echo".to_string(),
                            arguments: vec!["456".to_string()],
                            stdin_redirect: None,
                            stdout_redirect: None,
                        }),
                        Operator::Pipe,
                        Box::new(Cmd {
                            name: "grep".to_string(),
                            arguments: vec!["4".to_string()],
                            stdin_redirect: None,
                            stdout_redirect: None,
                        }),
                    )),
                    Operator::Pipe,
                    Box::new(Cmd {
                        name: "fmt".to_string(),
                        arguments: vec![],
                        stdin_redirect: None,
                        stdout_redirect: None,
                    }),
                )),
            )),
            Operator::NextIfSuccess,
            Box::new(Cmd {
                name: "seq".to_string(),
                arguments: vec!["3".to_string()],
                stdin_redirect: None,
                stdout_redirect: None,
            }),
        )),
        Operator::Next,
        Box::new(Cmd {
            name: "echo".to_string(),
            arguments: vec![],
            stdin_redirect: None,
            stdout_redirect: None,
        }),
    );
    assert_eq!(parse_input("echo 123 & echo 456 | grep 4 | fmt & seq 3 ; echo").unwrap(), expected_ast);
}

#[test]
fn parse_complex_binary_with_redirects_test() {
    initialize();
    let expected_ast = Binary(
        Box::new(Binary(
            Box::new(Binary(
                Box::new(Cmd {
                    name: "cat".to_string(),
                    arguments: vec!["./f".to_string()],
                    stdin_redirect: Some("input.txt".to_string()),
                    stdout_redirect: None,
                }),
                Operator::Pipe,
                Box::new(Cmd {
                    name: "fmt".to_string(),
                    arguments: vec![],
                    stdin_redirect: None,
                    stdout_redirect: None,
                }),
            )),
            Operator::NextIfSuccess,
            Box::new(Cmd {
                name: "echo".to_string(),
                arguments: vec!["ok".to_string()],
                stdin_redirect: None,
                stdout_redirect: None,
            }),
        )),
        Operator::Next,
        Box::new(Cmd {
            name: "echo".to_string(),
            arguments: vec!["next".to_string()],
            stdin_redirect: None,
            stdout_redirect: Some("a.txt".to_string()),
        }),
    );
    assert_eq!(parse_input("cat ./f < input.txt | fmt & echo ok ; echo next > a.txt").unwrap(), expected_ast);
}

