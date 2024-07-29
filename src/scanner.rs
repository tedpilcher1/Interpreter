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
    fn scan_tokens(&mut self) -> Result<(), String> {
        let line: usize = 1;

        // while not at end
        while !isAtEnd(&self) {
            // set start to current
            self.start = self.current;
            match self.scanToken() {
                Ok(_) => (),
                Err(msg) => return Err(msg),
            }
        }

        // add EOF token
        self.tokens.push(Token {
            token_type: crate::token_type::TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        });

        return Ok(());
    }

    fn scanToken(&mut self) -> Result<(), String> {
        match self.advance() {
            Ok(c) => match c {
                '(' => Ok(self.add_token(TokenType::LeftParen)),
                ')' => Ok(self.add_token(TokenType::RightParen)),
                '{' => Ok(self.add_token(TokenType::LeftBrace)),
                '}' => Ok(self.add_token(TokenType::RightBrace)),
                ',' => Ok(self.add_token(TokenType::Comma)),
                '.' => Ok(self.add_token(TokenType::Dot)),
                '-' => Ok(self.add_token(TokenType::Minus)),
                '+' => Ok(self.add_token(TokenType::Plus)),
                ';' => Ok(self.add_token(TokenType::Semicolon)),
                '*' => Ok(self.add_token(TokenType::Star)),

                '!' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token(TokenType::BangEqual))
                        } else {
                            Ok(self.add_token(TokenType::Bang))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '=' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token(TokenType::EqualEqual))
                        } else {
                            Ok(self.add_token(TokenType::Equal))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '<' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token(TokenType::LessEqual))
                        } else {
                            Ok(self.add_token(TokenType::Less))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '>' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token(TokenType::GreaterEqual))
                        } else {
                            Ok(self.add_token(TokenType::Greater))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '/' => match self.isCharMatch('/') {
                    Ok(was_match) => {
                        if was_match {
                            while self.peek() != Ok('\n') && !isAtEnd(self) {
                                match self.advance() {
                                    Ok(_) => (),
                                    Err(msg) => return Err(msg),
                                }
                            }
                        } else {
                            self.add_token(TokenType::Slash);
                        }
                        Ok(())
                    }
                    Err(msg) => Err(msg),
                },
                ' ' | '\r' | '\t' => Ok(()),
                '\n' => {
                    self.line += 1;
                    Ok(())
                }
                '"' => match self.string() {
                    Ok(_) => Ok(()),
                    Err(msg) => Err(msg),
                },
                _ => {
                    if isDigit(c) {
                        number()
                    } else {
                        Err("Unexpected character".to_string())
                    }
                }
            },
            Err(msg) => Err(msg),
        }
    }

    fn advance(&mut self) -> Result<char, String> {
        self.current += 1;
        charAt(&self.source, self.current - 1)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = &self.source[self.start..self.current];
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

    fn isCharMatch(&mut self, expected: char) -> Result<bool, String> {
        if isAtEnd(self) {
            return Ok(false);
        };

        match charAt(&self.source, self.current) {
            Ok(currentChar) => {
                if currentChar != expected {
                    return Ok(false);
                }
            }
            Err(msg) => return Err(msg),
        }

        self.current += 1;
        return Ok(true);
    }

    fn peek(&self) -> Result<char, String> {
        if isAtEnd(self) {
            return Ok('\0');
        }
        charAt(&self.source, self.current)
    }

    fn peek_next(&self) -> Result<char, String> {
        if (self.current + 1) as usize >= self.source.len() {
            return Ok('\0');
        }

        charAt(&self.source, self.current + 1)
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != Ok('"') && !isAtEnd(self) {
            if self.peek() == Ok('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if isAtEnd(self) {
            self.error(self.line, "Unterminated string.".to_string());
            return Ok(());
        }

        match self.advance() {
            Ok(_) => {
                let value = &self.source[self.start + 1..self.current - 1];
                self.add_token_literal(TokenType::String, Some(LiteralValue::SValue(value.to_string())));
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }

    fn error(&self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: usize, where_from: String, message: String) {
        println!("[line {} ] Error {}: {}", line, where_from, message);
        let _had_error = true;
    }

    fn number(&self) -> Result<(), String> {
        match self.peek() {
            Ok(c) => {while isDigit(c) {
                if self.peek() == Ok('.') && isDigit(self.peek_next()) {
                    self.advance();
                }
    
                match self.peek() {
                    Ok(c) => {}
                    Err(msg) => return Err(msg),
                };
            }
    
            match &self.source[self.start..self.current].parse::<f32>() {
                Ok(val) => {
                    self.add_token_literal(TokenType::Number, Some(LiteralValue::FValue(*val)));
                    return Ok(());
                    },
                Err(msg) => return Err(msg.to_string()),
            }}
            Err(msg) => return Err(msg),
        };
    }
}

fn isAtEnd(scanner: &Scanner) -> bool {
    return (scanner.current) >= scanner.source.len();
}

fn charAt(string: &String, index: usize) -> Result<char, String> {
    if index >= string.len() {
        return Err("Index out of bounds".to_string());
    }

    Ok(string.as_bytes()[index] as char)
}

fn isDigit(c: char) -> bool {
    return c >= '0' && c <= '9';
}
