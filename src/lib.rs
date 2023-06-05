use core::fmt;
use std::fs;
use std::io;
use std::io::{stdout, Write};

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

fn run(source: String) {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        // For now, just print the tokens.
        for token in scanner.tokens {
            println!("{:?}", token);
        }
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
        Err(_) => panic!("Must provide a valid file"),
    };

    run(bytes);
}

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    // End of File.
    EOF
}

#[derive(Debug)]
struct Token<TokenType>{
    kind: TokenType,
    lexeme: String,
    literal: Option<TokenType>,
    line: usize,
}

impl Token<TokenType> {
    fn new(kind: TokenType, lexeme: String,
           literal: Option<TokenType>, line: usize) -> Token<TokenType> {

        return Token {
            kind,
            lexeme,
            literal,
            line
        };
    }

    fn to_string(&self) -> String {
        return format!("{:?}{:?}{:?}",self.kind,self.lexeme, self.literal)
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token<TokenType>>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self){
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        let empty_string_1 = String::new();

        self.tokens.push(
            Token::new(
                TokenType::EOF, empty_string_1,None, self.line));

    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len();
    }

    fn add_token(&mut self, kind: TokenType , literal: Option<TokenType>) {
        let text: String = self.source.
            chars().
            skip(self.start).
            take(self.current).
            collect();
        self.tokens.push(Token::new(kind, text, literal, self.line));
    }

    fn add_token_type(&mut self, kind: TokenType) {
        self.add_token(kind, None);
    }

    fn scan_token(&mut self) {
        let c:char = self.advance();
        match c {
            '(' => self.add_token_type(TokenType::LeftParen), 
            ')' => self.add_token_type(TokenType::RightParen),
            '{' => self.add_token_type(TokenType::LeftBrace),
            '}' => self.add_token_type(TokenType::RightBrace),
            ',' => self.add_token_type(TokenType::Comma),
            '.' => self.add_token_type(TokenType::Dot),
            '-' => self.add_token_type(TokenType::Minus),
            '+' => self.add_token_type(TokenType::Plus),
            ';' => self.add_token_type(TokenType::Semicolon),
            '*' => self.add_token_type(TokenType::Star), 
            '!' => self.add_token_type(if self.match_next('=')
                                       {TokenType::BangEqual}
                                       else 
                                       {TokenType::Bang}),

            '=' => self.add_token_type(if self.match_next('=')
                                       {TokenType::EqualEqual}
                                       else
                                       {TokenType::Equal}),

            '<' => self.add_token_type(if self.match_next('=')
                                       {TokenType::LessEqual}
                                       else
                                       {TokenType::Less}),

            '>' => self.add_token_type(if self.match_next('=')
                                       {TokenType::GreaterEqual}
                                       else
                                       {TokenType::Greater}),
            '/' => self.add_token_type(if self.match_next('=')
                                       {TokenType::GreaterEqual}
                                       else
                                       {TokenType::Greater}),
            _ => panic!("{} Unexpected character", &self.line),
        }
    }

    fn match_next(&self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(4).unwrap() != expected { return false; }
        self.current += 1;
        return true;
    }
    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current);
        self.current += 1;
        match character {
            Some(x) => return x,
            None => panic!("CHAR NOT FOUND!!!!!!"),
        }
    }

}
fn error_handler(line: u32, message: String) {
    report(line, "TODO IN ERROR_HANDLER", message);
}

fn report(line: u32, location: &str, message: String) {
    panic!("line {line} Error {location} : {message}");
}
