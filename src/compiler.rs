use std::convert::TryFrom;
use std::convert::TryInto;

use chunk::{Chunk, Op};
use debug::{DEBUG_PRINT_CODE};
use scanner::{Scanner, Token, TokenType};
use value::Value;

type ParseRuleFn = Option<fn(&mut Compiler) -> ()>;

#[derive(Clone, Copy)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl TryFrom<usize> for Precedence {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == Precedence::None as usize => Precedence::None,
            x if x == Precedence::Assignment as usize => Precedence::Assignment,
            x if x == Precedence::Or as usize => Precedence::Or,
            x if x == Precedence::And as usize => Precedence::And,
            x if x == Precedence::Equality as usize => Precedence::Equality,
            x if x == Precedence::Comparison as usize => Precedence::Comparison,
            x if x == Precedence::Term as usize => Precedence::Term,
            x if x == Precedence::Factor as usize => Precedence::Factor,
            x if x == Precedence::Unary as usize => Precedence::Unary,
            x if x == Precedence::Call as usize => Precedence::Call,
            x if x == Precedence::Primary as usize => Precedence::Primary,
            _ => return Err(()),
        })
    }
}

pub struct Parser<'a> {
    pub scanner: Scanner<'a>,
    pub current: Token<'a>,
    pub previous: Option<Token<'a>>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: Scanner<'a>) -> Parser {
        let mut scanner: Scanner = scanner;
        let current: Token = scanner.scan_token();

        Parser {
            scanner,
            current,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) -> () {
        self.previous = Some(self.current);

        loop {
            self.current = self.scanner.scan_token();

            if TokenType::Error != self.current.token_type {
                break;
            }

            self.error_at_current(self.current.lexeme);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> () {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error(&mut self, message: &str) -> () {
        self.error_at(&self.previous.unwrap(), message);
    }

    fn error_at(&mut self, token: &Token<'a>, message: &str) -> () {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => (),
            _ => eprint!(" at '{}'", token.lexeme),
        }

        eprintln!(": {}", message);

        self.had_error = true;
    }

    fn error_at_current(&mut self, message: &str) -> () {
        let token = self.current;
        self.error_at(&token, message);
    }
}

pub struct ParseRule {
    prefix: ParseRuleFn,
    infix: ParseRuleFn,
    precedence: Precedence,
}

impl Default for ParseRule {
    fn default() -> Self {
        Self {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }
}

fn make_rules() -> Vec<ParseRule> {
    let mut rules: Vec<(TokenType, ParseRule)> = vec! [
        (
            TokenType::LeftParen,
            ParseRule {
                prefix: Some(|c| c.grouping()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (TokenType::RightParen, ParseRule::default()),
        (TokenType::LeftBrace, ParseRule::default()),
        (TokenType::RightBrace, ParseRule::default()),
        (TokenType::Dot, ParseRule::default()),
        (TokenType::Comma, ParseRule::default()),
        (TokenType::Semicolon, ParseRule::default()),
        (
            TokenType::Plus,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Term,
            }
        ),
        (
            TokenType::Minus,
            ParseRule {
                prefix: Some(|c| c.unary()),
                infix: Some(|x| x.binary()),
                precedence: Precedence::Term,
            }
        ),
        (
            TokenType::Slash,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Factor,
            }
        ),
        (
            TokenType::Star,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Factor,
            }
        ),
        (
            TokenType::Bang,
            ParseRule {
                prefix: Some(|c| c.unary()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (
            TokenType::BangEqual,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Equality,
            }
        ),
        (TokenType::Equal, ParseRule::default()),
        (TokenType::EqualEqual,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Equality,
            }
        ),
        (TokenType::Greater,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            }
        ),
        (TokenType::GreaterEqual,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            }
        ),
        (TokenType::Less,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            }
        ),
        (TokenType::LessEqual,
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            }
        ),
        (TokenType::Var, ParseRule::default()),
        (TokenType::Fun, ParseRule::default()),
        (TokenType::Class, ParseRule::default()),
        (TokenType::This, ParseRule::default()),
        (TokenType::Super, ParseRule::default()),
        (TokenType::If, ParseRule::default()),
        (TokenType::Else, ParseRule::default()),
        (TokenType::For, ParseRule::default()),
        (TokenType::While, ParseRule::default()),
        (TokenType::Return, ParseRule::default()),
        (TokenType::Print, ParseRule::default()),
        (TokenType::And, ParseRule::default()),
        (TokenType::Or, ParseRule::default()),
        (
            TokenType::True,
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (
            TokenType::False,
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (
            TokenType::Nil,
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (
            TokenType::Number,
            ParseRule {
                prefix: Some(|c| c.number()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (
            TokenType::String,
            ParseRule {
                prefix: Some(|c| c.string()),
                infix: None,
                precedence: Precedence::None,
            }
        ),
        (TokenType::Identifier, ParseRule::default()),
        (TokenType::Error, ParseRule::default()),
        (TokenType::Eof, ParseRule::default()),

    ];

    rules.sort_by(|a, b| (a.0 as usize).cmp(&(b.0 as usize)));
    return rules.into_iter().map(|(_, rule)| rule).collect::<Vec<ParseRule>>();
}

pub struct Compiler<'a> {
    parser: Parser<'a>,
    current_chunk: &'a mut Chunk,
    rules: Vec<ParseRule>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Compiler<'a> {
        return Compiler {
            parser: Parser::new(Scanner::new(source)),
            current_chunk: chunk,
            rules: make_rules(),
        };
    }

    pub fn compile(&mut self) -> bool {
        self.expression();
        self.parser.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();
        return !self.parser.had_error;
    }

    fn binary(&mut self) -> () {
        let operator_type: TokenType = self.parser.previous.unwrap().token_type;
        let rule: &ParseRule = self.get_rule(operator_type);

        self.parse_precedence((rule.precedence as usize + 1).try_into().unwrap());

        match operator_type {
            TokenType::BangEqual => {
                self.emit_op(Op::Equal);
                self.emit_op(Op::Not);
            },
            TokenType::EqualEqual => self.emit_op(Op::Equal),
            TokenType::Greater => self.emit_op(Op::Greater),
            TokenType::GreaterEqual => {
                self.emit_op(Op::Less);
                self.emit_op(Op::Not);
            },
            TokenType::Less => self.emit_op(Op::Less),
            TokenType::LessEqual => {
                self.emit_op(Op::Greater);
                self.emit_op(Op::Not);
            },
            TokenType::Plus => self.emit_op(Op::Add),
            TokenType::Minus => self.emit_op(Op::Subtract),
            TokenType::Star => self.emit_op(Op::Multiply),
            TokenType::Slash => self.emit_op(Op::Divide),
            _ => (),
        }
    }

    fn emit_byte(&mut self, byte: u8) -> () {
        self.current_chunk.write(byte, self.parser.previous.as_ref().unwrap().line);
    }

    fn emit_bytes(&mut self, byte0: u8, byte1: u8) -> () {
        self.emit_byte(byte0);
        self.emit_byte(byte1);
    }

    fn emit_constant(&mut self, value: Value) -> () {
        let constant: u8 = self.make_constant(value);
        return self.emit_bytes(Op::Constant.into(), constant);
    }

    fn emit_op(&mut self, op: Op) -> () {
        self.emit_byte(op.into());
    }

    fn emit_return(&mut self) -> () {
        self.emit_op(Op::Return);
    }

    fn end_compiler(&mut self) -> () {
        self.emit_return();

        if DEBUG_PRINT_CODE && !self.parser.had_error {
            self.current_chunk.dissassemble_chunk("code");
        }
    }

    fn expression(&mut self) -> () {
        self.parse_precedence(Precedence::Assignment);
    }

    fn get_rule(&self, token_type: TokenType) -> &ParseRule {
        return &self.rules[token_type as usize];
    }

    fn grouping(&mut self) -> () {
        self.expression();

        self.parser.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn literal(&mut self) -> () {
        match self.parser.previous.unwrap().token_type {
            TokenType::False => self.emit_op(Op::False),
            TokenType::Nil => self.emit_op(Op::Nil),
            TokenType::True => self.emit_op(Op::True),
            _ => (),
        }
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.current_chunk.add_constant(value);

        if constant > std::u8::MAX as usize {
            self.parser.error("Too many constants in one chunk.");
            return 0;
        }

        return constant as u8;
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> () {
        self.parser.advance();

        let prefix_rule: ParseRuleFn = self.get_rule(self.parser.previous.unwrap().token_type).prefix;

        match prefix_rule {
            None => {
                self.parser.error("Expect expression.");
                return;
            }
            Some(prefix_rule) => prefix_rule(self),
        }

        loop {
            let rule: &ParseRule = self.get_rule(self.parser.current.token_type);

            if precedence as usize > rule.precedence as usize {
                return;
            }

            let infix_rule = rule.infix.unwrap();
            self.parser.advance();
            infix_rule(self);
        }

    }

    fn number(&mut self) -> () {
        let value: f64 = self.parser.previous.unwrap().lexeme.parse().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn string(&mut self) -> () {
        let lexeme: &str = self.parser.previous.unwrap().lexeme;
        let value: String = (&lexeme[1..(lexeme.len() - 1)]).to_string();
        self.emit_constant(Value::String(value));
    }

    fn unary(&mut self) -> () {
        let operator_type: TokenType = self.parser.previous.unwrap().token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_op(Op::Not),
            TokenType::Minus => self.emit_op(Op::Negate),
            _ => (),
        }
    }
}
