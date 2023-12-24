use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // first argument will be always the program name

    for arg in args {
        print!("{} ", arg);
    }
}