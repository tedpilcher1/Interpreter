use std::{fmt::Error, ptr::null};

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
                Err(msg) => return Err(msg.to_string()),
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
        // let c: char = advance();

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

                // '!' | '=' | '<' | '>' => {

                // }
                // '!' => Ok(match()),
                // '=' => Ok(self.add_token_null_literal(TokenType::Star)),
                // '<' => Ok(self.add_token_null_literal(TokenType::Star)),
                // '>' => Ok(self.add_token_null_literal(TokenType::Star)),
                _ => Err("Unexpected character".to_string()),
            },
            Err(msg) => Err(msg),
        }
    }

    // case '*': addToken(STAR); break;
    // case '!':
    // addToken(match('=') ? BANG_EQUAL : BANG);
    // break;
    // case '=':
    // addToken(match('=') ? EQUAL_EQUAL : EQUAL);
    // break;
    // case '<':
    // addToken(match('=') ? LESS_EQUAL : LESS);
    // break;
    // case '>':
    // addToken(match('=') ? GREATER_EQUAL : GREATER);
    // break;

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
            Err(msg) => return Err(msg.to_string()),
        }

        self.current += 1;
        return Ok(true);
    }
}

fn isAtEnd(scanner: &Scanner) -> bool {
    return (scanner.current as usize) >= scanner.source.len();
}

fn charAt(string: &String, index: i32) -> Result<char, String> {
    match string.chars().nth(index as usize) {
        Some(char) => Ok(char),
        None => Err("Index out of bounds".to_string()),
    }
}
