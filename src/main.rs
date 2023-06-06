use std::{env, process};


mod error;
use error::*;

mod scanner;
use scanner::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    lox(args);
}
