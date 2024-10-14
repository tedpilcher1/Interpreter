use crate::{scanner::LiteralValue, token_type::TokenType};

pub enum Expr {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

fn interal_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Literal(literal) => match literal {
            LiteralExpression::Number(num) => num.to_string(),
            LiteralExpression::String(string) => string.to_string(),
            LiteralExpression::Boolean(boolean) => boolean.to_string(),
            LiteralExpression::Nil => "nil".to_string(),
        },
        Expr::Unary(unary) => unary.operator.to_string(),
        Expr::Binary(binary) => binary.operator.to_string(),
        _ => panic!("Should not hit this arm"),
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Literal(_) | Expr::Binary(_) | Expr::Unary(_) => interal_to_string(self),
            Expr::Grouping(grouping_expression) => {
                let expr = grouping_expression.expression.as_ref();
                interal_to_string(expr)
            }
        }
    }
}

pub struct BinaryExpression {
    pub left: Box<Expr>,
    pub operator: TokenType,
    pub right: Box<Expr>,
}

pub struct UnaryExpression {
    pub operator: TokenType,
    pub right: Box<Expr>,
}

pub enum LiteralExpression {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct GroupingExpression {
    pub expression: Box<Expr>,
}

impl From<LiteralValue> for LiteralExpression {
    fn from(value: LiteralValue) -> Self {
        match value {
            LiteralValue::FValue(f_value) => self::LiteralExpression::Number(f_value),
            LiteralValue::SValue(s_value) => self::LiteralExpression::String(s_value),
        }
    }
}
