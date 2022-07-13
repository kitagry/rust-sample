use std::env;
use std::fmt::Debug;
use std::fs::read_dir;
use std::io;
use std::process::exit;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] io::Error),
}

fn main() {
    let args = env::args().skip(1);
    for arg in args {
        if let Err(err) = run(&arg) {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn run(path: &str) -> Result<(), Error> {
    println!("{}", path);
    print_tree(path, "")?;
    Ok(())
}

fn print_tree(path: &str, prefix: &str) -> Result<(), Error> {
    let (dirs, errors): (Vec<_>, Vec<_>) = read_dir(path)?.partition(Result::is_ok);
    for err in errors {
        eprintln!("{:?}", err);
    }

    let dir_len = dirs.len();
    for (i, dir) in dirs.into_iter().map(Result::unwrap).enumerate() {
        let file_prefix = if i + 1 == dir_len {
            "└──"
        } else {
            "├──"
        };
        println!(
            "{}{} {}",
            prefix,
            file_prefix,
            dir.file_name().to_str().unwrap_or_else(|| "")
        );

        if dir.metadata()?.is_dir() {
            let dir_prefix = if i + 1 == dir_len { "    " } else { "│   " };
            let new_prefix = format!("{}{}", prefix, dir_prefix);
            print_tree(dir.path().to_str().unwrap_or_else(|| ""), &new_prefix)?;
        }
    }
    Ok(())
}
