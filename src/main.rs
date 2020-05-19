mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;

use std::{env, path, process, fs};
use std::io::{self, Write};

use interpreter::types;

#[allow(unused_must_use)]
fn run<'a>(code: String) -> Result<types, &'a str> {

    let (tokens, had_error) = lexer::scan(code);

    // println!("Tokens:");
    // for token in &tokens {
    //     println!("{:?}", token.class);
    // }
    // println!();

    if had_error {
        return Err("Aborting due to error while lexing.");
    }

    let (statements, had_error) = parser::parse(tokens);

    // println!("{:#?}", statements);

    if had_error {
        return Err("Aborting due to error while parsing.");
    }

    let expr = interpreter::interpret(statements).unwrap_or(types::nil);

    Ok(expr)
}

fn run_file(file_path: path::PathBuf) {

    let display = file_path.display();

    let code = fs::read_to_string(&file_path).unwrap_or_else(|error| {
        panic!("couldn't read {}: {}", display, error)
    });

    run(code).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(exitcode::DATAERR);
    });

}

#[allow(unused_must_use)]
fn run_prompt() {

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        // errors were already handled at this point,
        // we default the expression to nil so it isn't
        // printed
        let expr = run(line).unwrap_or(types::nil);

        if expr != types::nil {
            println!("{:?}", expr);
        }
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