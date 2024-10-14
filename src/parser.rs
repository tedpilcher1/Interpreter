use crate::{
    expressions::{BinaryExpression, Expr, GroupingExpression, LiteralExpression, UnaryExpression},
    token_type::{self, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while (self.match_token_types(&vec![TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = self.previous().clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.token_type,
                right: Box::new(right),
            })
        }
        expr
    }

    fn match_token_types(&mut self, types: &Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == *token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_token_types(&vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            expr = Expr::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.token_type.clone(),
                right: Box::new(self.term()),
            });
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_token_types(&vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            expr = Expr::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.token_type.clone(),
                right: Box::new(self.factor()),
            })
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token_types(&vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            expr = Expr::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: operator.token_type.clone(),
                right: Box::new(self.unary()),
            })
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token_types(&vec![TokenType::Bang, TokenType::Minus]) {
            let right = self.unary();
            let operator = self.previous();
            return Expr::Unary(UnaryExpression {
                operator: operator.token_type.clone(),
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token_types(&vec![TokenType::False]) {
            return Expr::Literal(LiteralExpression::Boolean(false));
        }

        if self.match_token_types(&vec![TokenType::True]) {
            return Expr::Literal(LiteralExpression::Boolean(true));
        }

        if self.match_token_types(&vec![TokenType::Nil]) {
            return Expr::Literal(LiteralExpression::Nil);
        }

        if self.match_token_types(&vec![TokenType::Number, TokenType::String]) {
            // could probably avoid the clone here but oh well
            return Expr::Literal(self.previous().literal.clone().unwrap().into());
        }

        if self.match_token_types(&vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(GroupingExpression {
                expression: Box::new(expr),
            });
        }

        panic!("Primary didn't match an expected token type");
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(&token_type) {
            return self.advance();
        }

        panic!("{}", message);
    }

    fn synchonise(&mut self) {
        self.advance();

        while (!self.is_at_end()) {
            if (self.previous().token_type == TokenType::Semicolon) {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}
