use shell::interpreter::get_args;

fn main() {
    let output = get_args().join(" ");
    print!("{}", output);
}