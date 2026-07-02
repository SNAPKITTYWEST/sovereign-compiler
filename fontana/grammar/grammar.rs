// Fontana grammar definition
//
// Non-recursive grammar for symbolic declarations.

/// Token types for the Fontana language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Decl,
    Let,
    If,
    Else,
    Return,
    Verify,
    Seal,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqEq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Arrow,
    FatArrow,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Hash,

    // Literals
    Ident(String),
    Integer(u64),
    Float(f64),
    String(String),
    Bool(bool),

    // Special
    Eof,
    Error(String),
}

/// Lexer for Fontana grammar.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];

        match ch {
            '+' => { self.advance(); Token::Plus }
            '-' => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => { self.advance(); Token::Star }
            '/' => { self.advance(); Token::Slash }
            '=' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::EqEq
                } else {
                    Token::Equal
                }
            }
            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Neq
                } else {
                    Token::Error("unexpected character: !".to_string())
                }
            }
            '<' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            '>' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            '(' => { self.advance(); Token::LParen }
            ')' => { self.advance(); Token::RParen }
            '{' => { self.advance(); Token::LBrace }
            '}' => { self.advance(); Token::RBrace }
            '[' => { self.advance(); Token::LBracket }
            ']' => { self.advance(); Token::RBracket }
            ',' => { self.advance(); Token::Comma }
            ';' => { self.advance(); Token::Semicolon }
            ':' => { self.advance(); Token::Colon }
            '.' => { self.advance(); Token::Dot }
            '#' => { self.advance(); Token::Hash }
            c if c.is_ascii_digit() => self.read_number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.read_ident_or_keyword(),
            '"' => self.read_string(),
            _ => {
                self.advance();
                Token::Error(format!("unexpected character: {}", ch))
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        if s.contains('.') {
            Token::Float(s.parse().unwrap_or(0.0))
        } else {
            Token::Integer(s.parse().unwrap_or(0))
        }
    }

    fn read_ident_or_keyword(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == '_') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();

        match s.as_str() {
            "decl" => Token::Decl,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "verify" => Token::Verify,
            "seal" => Token::Seal,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Ident(s),
        }
    }

    fn read_string(&mut self) -> Token {
        self.advance(); // skip opening quote
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            if self.input[self.pos] == '\\' {
                self.advance();
                if self.pos < self.input.len() {
                    s.push(self.input[self.pos]);
                }
            } else {
                s.push(self.input[self.pos]);
            }
            self.pos += 1;
        }
        if self.pos < self.input.len() {
            self.advance(); // skip closing quote
        }
        Token::String(s)
    }
}
