use std::env;
use std::process;
use std::fs;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    // exit program if no file name is given
    if args.len() != 1 {
        println!("usage: cat <filename>");
        process::exit(0);
    }

    let file_path = &args[0];

    match fs::read_to_string(file_path) {
        Ok(data) => {
            println!("{}", data);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}