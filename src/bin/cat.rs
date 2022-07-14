#![allow(warnings)]

use shell::interpreter::{get_args, process_options, read_files, read_stdin};

fn main() {
    let mut stdout = String::new();
    let mut stderr = String::new();

    let arguments = get_args();
    let (files, options) = split_args(&arguments);


    if !files.is_empty() {
        read_files(files, &mut stdout, &mut stderr)
    } else {}

    match process_options(options, vec!['b', 'n']) {
        Ok(flags) => {
            if flags.contains(&'b') {
                stdout = remove_empty_lines(&stdout)
            }
            if flags.contains(&'n') {
                stdout = display_line_numbers(&stdout)
            }
        }
        Err(char) => {
            eprint!("{} is not a valid option", char);
            return;
        }
    }

    print!("{}", stdout);
    eprint!("{}", stderr);
}

fn split_args(arguments: &[String]) -> (Vec<String>, Vec<String>) {
    arguments
        .iter()
        .fold((vec![], vec![]), |mut acc, x| {
            if x.starts_with("-") {
                acc.1.push(x.to_string().replacen("-", "", 1));
            } else {
                acc.0.push(x.to_string());
            }
            (acc.0, acc.1)
        })
}

fn remove_empty_lines(str: &str) -> String {
    str
        .split("\r\n")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .join("\r\n")
}

fn display_line_numbers(str: &str) -> String {
    let mut counter = 0;
    str
        .split("\r\n")
        .map(|x| {
            counter += 1;
            format!("    {} {}", counter, x)
        })
        .collect::<Vec<String>>()
        .join("\r\n")
}
