use std::ptr::null;

use crate::token_type::{Token, TokenType};

pub enum LiteralValue {
    FValue(f32),
    SValue(String),
}

impl ToString for LiteralValue {
    fn to_string(&self) -> String {
        match self {
            Self::FValue(val) => val.to_string(),
            Self::SValue(val) => val.to_string(),
        }
    }
}

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn scan_tokens(&mut self) {
        let line: usize = 1;

        // while not at end
        while !isAtEnd(&self) {
            // set start to current
            self.start = self.current;
            self.scanToken();
        }

        // add EOF token
        self.tokens.push(Token {
            token_type: crate::token_type::TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        });
    }

    fn scanToken(&mut self) {
        let c = self.advance();

        match c {
            // match self.advance() {
            // Ok(c) => match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.isCharMatch('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.isCharMatch('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.isCharMatch('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.isCharMatch('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.isCharMatch('/') {
                    while self.peek() != '\n' && !isAtEnd(&self) {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            'o' => {
                if self.peek() == 'r' {
                    self.add_token(TokenType::Or)
                }
            }
            _ => {
                if isDigit(c) {
                    self.number()
                } else if isAlpha(c) {
                    self.identifier();
                } else {
                    self.error(self.line, "Unexpected character".to_string())
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        charAt(&self.source, self.current - 1)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        })
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn isCharMatch(&mut self, expected: char) -> bool {
        if isAtEnd(self) {
            return false;
        };

        if charAt(&self.source, self.current) != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if isAtEnd(self) {
            return '\0';
        }
        charAt(&self.source, self.current)
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        }

        charAt(&self.source, self.current + 1)
    }

    fn string(&mut self) {
        while self.peek() != '"' && isAtEnd(&self) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if isAtEnd(&self) {
            self.error(self.line, "Unterminated string.".to_string());
        }

        self.advance();
        let val = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(
            TokenType::String,
            Some(LiteralValue::SValue(val.to_string())),
        )
    }

    fn error(&self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: usize, where_from: String, message: String) {
        println!("[line {} ] Error {}: {}", line, where_from, message);
        let _had_error = true;
    }

    fn number(&mut self) {
        while isDigit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && isDigit(self.peek_next()) {
            // consume the .
            self.advance();
            while isDigit(self.peek()) {
                self.advance();
            }
        }
        let double = &self.source[self.start..self.current].parse::<f32>().unwrap();
        self.add_token_literal(
            TokenType::Number,
            Some(LiteralValue::FValue(*double)),
        )
    }

    fn identifier(&self) {
        return;
    }
}

fn isAtEnd(scanner: &Scanner) -> bool {
    return (scanner.current) >= scanner.source.len();
}

fn charAt(string: &String, index: usize) -> char {
    string.as_bytes()[index] as char
}

fn isDigit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn isAlpha(c: char) -> bool {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}
