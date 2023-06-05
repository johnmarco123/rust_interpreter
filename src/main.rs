use std::{env, process};
use rust_interpreter::lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    lox(args);
}
