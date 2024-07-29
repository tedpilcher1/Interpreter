use std::ptr::null;

use crate::token_type::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    fn scan_tokens(&mut self) -> Result<(), String> {
        let line: i32 = 1;

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
            literal: "".to_string(),
            line,
        });

        return Ok(());
    }

    fn scanToken(&mut self) -> Result<(), String> {
        match self.advance() {
            Ok(c) => match c {
                '(' => Ok(self.add_token_null_literal(TokenType::LeftParen)),
                ')' => Ok(self.add_token_null_literal(TokenType::RightParen)),
                '{' => Ok(self.add_token_null_literal(TokenType::LeftBrace)),
                '}' => Ok(self.add_token_null_literal(TokenType::RightBrace)),
                ',' => Ok(self.add_token_null_literal(TokenType::Comma)),
                '.' => Ok(self.add_token_null_literal(TokenType::Dot)),
                '-' => Ok(self.add_token_null_literal(TokenType::Minus)),
                '+' => Ok(self.add_token_null_literal(TokenType::Plus)),
                ';' => Ok(self.add_token_null_literal(TokenType::Semicolon)),
                '*' => Ok(self.add_token_null_literal(TokenType::Star)),

                '!' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token_null_literal(TokenType::BangEqual))
                        } else {
                            Ok(self.add_token_null_literal(TokenType::Bang))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '=' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token_null_literal(TokenType::EqualEqual))
                        } else {
                            Ok(self.add_token_null_literal(TokenType::Equal))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '<' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token_null_literal(TokenType::LessEqual))
                        } else {
                            Ok(self.add_token_null_literal(TokenType::Less))
                        }
                    }
                    Err(msg) => Err(msg),
                },
                '>' => match self.isCharMatch('=') {
                    Ok(was_match) => {
                        if was_match {
                            Ok(self.add_token_null_literal(TokenType::GreaterEqual))
                        } else {
                            Ok(self.add_token_null_literal(TokenType::Greater))
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
                            self.add_token_null_literal(TokenType::Slash);
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
        let index = self.current - 1;
        charAt(&self.source, index)
    }

    fn add_token_null_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, "".to_string());
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        })
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
                let value = &self.source[self.start as usize + 1..self.current as usize - 1];
                self.add_token(TokenType::String, value.to_string());
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }

    fn error(&self, line: i32, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&self, line: i32, where_from: String, message: String) {
        println!("[line {} ] Error {}: {}", line, where_from, message);
        let _had_error = true;
    }

    fn number(&self) -> Result<(), String> {
        match self.peek() {
            Ok(c) => {}
            Err(msg) => return Err(msg),
        };

        // while isDigit(c) {
        //     if self.peek() == Ok('.') && isDigit(self.peek_next()) {
        //         self.advance();
        //     }

        //     match self.peek() {
        //         Ok(c) => {}
        //         Err(msg) => return Err(msg),
        //     };
        // }
    }
}

fn isAtEnd(scanner: &Scanner) -> bool {
    return (scanner.current as usize) >= scanner.source.len();
}

fn charAt(string: &String, index: i32) -> Result<char, String> {
    if index as usize >= string.len() {
        return Err("Index out of bounds".to_string());
    }

    Ok(string.as_bytes()[index as usize] as char)
}

fn isDigit(c: char) -> bool {
    return c >= '0' && c <= '9';
}
