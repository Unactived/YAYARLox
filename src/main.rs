mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;

use std::io::{self, Write};
use std::{env, fs, path, process};

use interpreter::{types, Interpreter};

#[allow(unused_must_use)]
fn lex_and_parse<'a>(code: String) -> Result<Vec<ast::Stmt>, &'a str> {
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

    Ok(statements)
}

fn run_file(file_path: path::PathBuf) {
    let display = file_path.display();

    let code = fs::read_to_string(&file_path).unwrap_or_else(|error| {
        eprintln!("Couldn't read {}: {}", display, error);
        process::exit(exitcode::NOINPUT);
    });

    let mut interpreter = Interpreter::new();

    let statements = lex_and_parse(code).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(exitcode::DATAERR);
    });

    interpreter.interpret(statements).unwrap_or_else(|()| {
        process::exit(exitcode::DATAERR);
    });
}

#[allow(unused_must_use)]
fn run_prompt() {
    let mut interpreter = Interpreter::new();

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

        let statements = lex_and_parse(line);

        let statements = match statements {
            Ok(stmts) => stmts,
            Err(_) => continue,
        };

        let expr = interpreter.interpret(statements).unwrap_or(types::nil);

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
            eprintln!("Usage: {} [script]", args.next().unwrap());
            process::exit(exitcode::USAGE);
        }
    }
}
