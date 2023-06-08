use std::{env,
process, io::{self, Write}, fs};

mod expr;
use expr::*;

mod parser;
use parser::*;

mod token;
use token::*;

mod error;
use error::*;

mod token_type;
use token_type::*;

mod scanner;
use scanner::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    lox(args);
}

pub fn lox (args:Vec<String>) {
    if args.len() > 2 {

    } else if args.len() == 2 {
        // interpret the entire file
        run_file(&args[1]);
    } else {
        // node style one line at a time
        run_prompt();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    for token in scanner.tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        print!("> ");
        // flush outputs the print above
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let line = input.trim().to_string();

                if line == "exit" { break }

                run(line);
            }
            Err(_) => { break }
        }

    }
}

fn run_file(path: &String) {
    let bytes: String = match fs::read(path) {
        Ok(x) => String::from_utf8(x).unwrap(), 
        Err(_) => {
            todo!()
        },
    };
    run(bytes);
}
