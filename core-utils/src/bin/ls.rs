use std::env;
use std::fs;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() > 1 {
        println!("usage: ls <folder path>");
        process::exit(0);
    }

    let target_path = if args.len() == 1 {
        format!("{}", &args[0])
    } else {
        match env::current_dir() {
            Ok(path) => path.display().to_string(),
            Err(err) => {
                println!("{}", err);
                process::exit(0);
            }
        }
    };

    read_folder(target_path);
}

fn read_folder(path: String) {
    let files = match fs::read_dir(path) {
        Ok(files) => files,
        Err(err) => {
            println!("{}" ,err);
            process::exit(0);
        }
    };

    for file in files {
        match file {
            Ok(file) => {
                let meta = file.metadata().unwrap();
                let mut file_name = file.file_name().to_str().unwrap().to_string();

                if meta.is_dir() {
                    file_name.push('/');
                }

                println!("{}", file_name);
            },
            Err(_) => continue, // skip error prone file instead of showing error
        }
    }
}