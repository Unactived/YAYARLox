use std::{env, path, process, error::Error, fs};
use std::io::{self, Write};
use exitcode;

mod errors;
mod lexer;

fn run(code: String) -> Result<(), Box<dyn Error>> {

    let (tokens, had_error) = lexer::scan(code);

    if had_error {
        eprintln!("There was an error");
    }

    for token in &tokens {
        println!("{:?}", token.class);
    }

    Ok(())
}

fn run_file(file_path: path::PathBuf) {

    let display = file_path.display();

    let code = fs::read_to_string(&file_path).unwrap_or_else(|error| {
        panic!("couldn't read {}: {}", display, error)
    });

    run(code).unwrap_or_else(|_error| {
        process::exit(exitcode::DATAERR);
    });

}

fn run_prompt() {

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        run(line).unwrap_or_else(|_error| {
            process::exit(exitcode::DATAERR);
        });
    }

}

fn main() {

    let mut args = env::args();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(path::PathBuf::from(args.nth(1).unwrap())),
        _ => {
            eprintln!("Usage: {} [script]", args.nth(0).unwrap());
            process::exit(exitcode::USAGE);
        },
    }

}