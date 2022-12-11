use std::str::Chars;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    Slash,
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

    Error,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

pub struct Scanner<'a> {
    pub start: Chars<'a>,
    pub current: Chars<'a>,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner {
        return Scanner {
            start: source.chars(),
            current: source.chars(),
            line: 1,
        };
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current.clone();

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
            '+' => return self.make_token(TokenType::Plus),
            '-' => return self.make_token(TokenType::Minus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => (
                if self.match_character('=') {
                    return self.make_token(TokenType::BangEqual);
                } else {
                    return self.make_token(TokenType::Bang);
                }
            ),
            '=' => (
                if self.match_character('=') {
                    return self.make_token(TokenType::EqualEqual);
                } else {
                    return self.make_token(TokenType::Equal);
                }
            ),
            '<' => (
                if self.match_character('=') {
                    return self.make_token(TokenType::LessEqual);
                } else {
                    return self.make_token(TokenType::Less);
                }
            ),
            '>' => (
                if self.match_character('=') {
                    return self.make_token(TokenType::GreaterEqual);
                } else {
                    return self.make_token(TokenType::Greater);
                }
            ),
            '"' => return self.string(),
            _ => (),
        }

        return self.error_token("Unexpected character.");
    }

    fn advance(&mut self) -> char {
        let previous: char = self.peek();

        self.current.next();

        return previous;
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        if &self.lexeme()[start..] == rest {
            return token_type;
        }

        return TokenType::Identifier;
    }

    fn error_token(&mut self, message: &'static str) -> Token<'a> {
        return Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line,
        };
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        return self.make_token(self.identifier_type());
    }

    fn identifier_type(&self) -> TokenType {
        match self.start.as_str().as_bytes()[0] as char {
            'a' => return self.check_keyword(1, "nd", TokenType::And),
            'c' => return self.check_keyword(1, "lass", TokenType::Class),
            'e' => return self.check_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.start.as_str().len() > 1 {
                    match self.start.as_str().as_bytes()[1] as char {
                        'a' => return self.check_keyword(2, "lse", TokenType::False),
                        'o' => return self.check_keyword(2, "r", TokenType::For),
                        'u' => return self.check_keyword(2, "n", TokenType::Fun),
                        _ => (),
                    }
                }
            },
            'i' => return self.check_keyword(1, "f", TokenType::If),
            'n' => return self.check_keyword(1, "il", TokenType::Nil),
            'o' => return self.check_keyword(1, "r", TokenType::Or),
            'p' => return self.check_keyword(1, "rint", TokenType::Print),
            'r' => return self.check_keyword(1, "eturn", TokenType::Return),
            's' => return self.check_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.start.as_str().len() > 1 {
                    match self.start.as_str().as_bytes()[1] as char {
                        'h' => return self.check_keyword(2, "is", TokenType::This),
                        'r' => return self.check_keyword(2, "ue", TokenType::True),
                        _ => (),
                    }
                }
            },
            'v' => return self.check_keyword(1, "ar", TokenType::Var),
            'w' => return self.check_keyword(1, "hile", TokenType::While),
            _ => (),
        }

        return TokenType::Identifier
    }

    fn is_alpha(&self, character: char) -> bool {
        return (character >= 'a' && character <= 'z') ||
               (character >= 'A' && character <= 'Z') ||
                character == '_';
    }

    fn is_at_end(&self) -> bool {
        return self.current.as_str().len() == 0;
    }

    fn is_digit(&self, character: char) -> bool {
        return character >= '0' && character <= '9';
    }

    fn lexeme(&self) -> &'a str {
        let end: usize = self.start.as_str().len() - self.current.as_str().len();
        &self.start.as_str()[0..end]
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        return Token {
            token_type: token_type,
            lexeme: &self.lexeme(),
            line: self.line,
        };
    }

    fn match_character(&mut self, expected: char) -> bool {
        if !self.is_at_end() && self.peek() == expected {
            self.advance();
            return true;
        }

        return false;
    }

    fn number(&mut self) -> Token<'a> {
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
        if self.is_at_end() {
            return '\0';
        }

        return self.current.as_str().as_bytes()[0] as char;
    }

    fn peek_next(&self) -> char {
        if self.current.as_str().len() < 2 {
            return '\0';
        }

        return self.current.as_str().as_bytes()[1] as char;
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token<'a> {
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
