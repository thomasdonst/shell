use std::process::exit;
use shell::utils::{eq, get_args};

fn main() {
    let arguments = get_args();
    if arguments.len() < 2 {
        exit(2)
    }
    if eq(&arguments) { exit(0) } else { exit(1) }
}
