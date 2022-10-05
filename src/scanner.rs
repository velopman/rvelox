use std::{fmt};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Salsh,
    Star,

    // One or two-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Keywords
    Var,
    Fun,
    Class,
    This,
    Super,
    If,
    Else,
    For,
    While,
    Return,
    Print,
    And,
    Or,
    True,
    False,
    Nil,

    // Literals
    Number,
    String,
    Identifier,

    Error(String),
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

pub struct Scanner<'a> {
    pub source: &'a String,
    pub line: usize,
    pub start: usize,
    pub current: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Scanner<'a> {
        return Scanner {
            source,
            line: 1,
            start: 0,
            current: 0,
        };
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let character: char = self.advance();

        if self.is_alpha(character) {
            return self.identifier();
        }

        if self.is_digit(character) {
            return self.number();
        }

        match character {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            '.' => return self.make_token(TokenType::Dot),
            ',' => return self.make_token(TokenType::Comma),
            ';' => return self.make_token(TokenType::Semicolon),
            '-' => return self.make_token(TokenType::Plus),
            '+' => return self.make_token(TokenType::Minus),
            '/' => return self.make_token(TokenType::Salsh),
            '*' => return self.make_token(TokenType::Star),
            '!' => (
                if self.match_character('=') {
                    self.make_token(TokenType::BangEqual);
                } else {
                    self.make_token(TokenType::Bang);
                }
            ),
            '=' => (
                if self.match_character('=') {
                    self.make_token(TokenType::EqualEqual);
                } else {
                    self.make_token(TokenType::Equal);
                }
            ),
            '<' => (
                if self.match_character('=') {
                    self.make_token(TokenType::LessEqual);
                } else {
                    self.make_token(TokenType::Less);
                }
            ),
            '>' => (
                if self.match_character('=') {
                    self.make_token(TokenType::GreaterEqual);
                } else {
                    self.make_token(TokenType::Greater);
                }
            ),
            '"' => return self.string(),
            _ => (),
        }


        return self.error_token("Unexpected character.");
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        return self.source.chars().nth(self.current - 1).unwrap();
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType) -> TokenType {
        if self.current - self.start == start + length &&
           self.source.chars().skip(start).take(length).collect::<String>() == rest.to_string() {
            return token_type;
        }

        return TokenType::Identifier;
    }

    fn error_token(&mut self, message: &str) -> Token {
        return self.make_token(TokenType::Error(message.to_string()));
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).unwrap() {
                        'h' => self.check_keyword(2, 3, "is", TokenType::This),
                        'r' => self.check_keyword(2, 1, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),

            _ => TokenType::Identifier,
        }
    }

    fn is_alpha(&self, character: char) -> bool {
        return (character >= 'a' && character <= 'z') ||
               (character >= 'A' && character <= 'Z') ||
                character == '_';
    }

    fn is_at_end(&self) -> bool {
        return self.current == self.source.len();
    }

    fn is_digit(&self, character: char) -> bool {
        return character >= '0' && character <= '9';
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        return Token {
            token_type: token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            // literal: LiteralValue()
        };
    }

    fn match_character(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn number(&mut self) -> Token {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    return;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    return;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                    return;
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated String");
        }

        self.advance();

        return self.make_token(TokenType::String);
    }
}
