use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and",   TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else",  TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for",   TokenType::For);
        keywords.insert("fun",   TokenType::Fun);
        keywords.insert("if",    TokenType::If);
        keywords.insert("nil",   TokenType::Nil);
        keywords.insert("or",    TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return",TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this",  TokenType::This);
        keywords.insert("true",  TokenType::True);
        keywords.insert("var",   TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords
    };
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
#[derive(Clone)]
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
struct Token {
    kind: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(kind: TokenType, lexeme: String,
           literal: Option<String>, line: usize) -> Token {

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
    tokens: Vec<Token>,
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
        while !self.at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        let empty_string_1 = String::new();

        self.tokens.push(
            Token::new(
                TokenType::EOF, empty_string_1, None, self.line));
    }

    fn at_end(&mut self) -> bool {
        return self.current >= self.source.len();
    }

    fn add_token(&mut self, kind: TokenType , literal: Option<String>) {
        let text: String = self.source.
            chars().
            skip(self.start).
            take(self.current).
            collect();
        self.tokens.push(Token::new(kind, text, None, self.line));
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
            '!' =>{
                let n = match self.match_next('='){
                    true => {TokenType::BangEqual}
                    false => {TokenType::Bang}
                };
                self.add_token_type(n);
            }
            '=' =>{
                let n = match self.match_next('='){
                    true => {TokenType::EqualEqual}
                    false => {TokenType::Equal}
                };
                self.add_token_type(n);
            }
            '<' =>{
                let n = match self.match_next('='){
                    true => {TokenType::LessEqual}
                    false => {TokenType::Less}
                };
                self.add_token_type(n);
            }
            '>' =>{
                let n = match self.match_next('='){
                    true => {TokenType::GreaterEqual}
                    false => {TokenType::Greater}
                };
                self.add_token_type(n);
            }
            '/' => match self.match_next('/') {
                true => while self.peek() != '\n' && !self.at_end() { self.advance(); }
                false => { self.add_token_type(TokenType::Slash); }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_digit(10) { self.number() } 

                else if c.is_alphabetic() { self.identifier() } 
                else {
                    panic!("{} Unexpected character", &self.line)
                }
            },
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() { self.advance(); }

        let text: String = self.source.
            chars().
            skip(self.start).
            take(self.current).
            collect();

        let text: &str = text.as_str();


        let kind = KEYWORDS.get(text);
        let kind = match kind {
            Some(x) => x.clone(),
            None => TokenType::Identifier,
        };

        self.add_token_type(kind);
    }

    fn number(&mut self) -> () {
        while self.peek().is_digit(10) { self.advance(); }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) { self.advance(); }
        }

        let literal = self.source.
            chars().
            skip(self.start).
            take(self.current).
            collect::<String>();

        self.add_token(TokenType::Number, Some(literal))
    }

    fn peek_next(&mut self) -> char  {
        if self.current + 1 >= self.source.len() { return '\0' };
        return self.source.chars().nth(self.current + 1).unwrap();
    } 
    
    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' { self.line += 1 };
            self.advance();
        }

        if self.at_end() {
            panic!("{} Unterminated string.", self.line);
        }
        //
        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value: String = self.source.
            chars().
            skip(self.start + 1).
            take(self.current - 1).
            collect();

        self.add_token(TokenType::String, Some(value));
    }

    fn peek(&mut self) -> char {
        if self.at_end() { return '\0' };
        return self.source.chars().nth(self.current).unwrap();
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.at_end() { return false; }
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
