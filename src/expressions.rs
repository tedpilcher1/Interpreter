use crate::token_type::TokenType;

pub enum Expr {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

// impl ToString for Expr {
//     fn to_string(&self) -> String {
//         match self {
//             Expr::Literal(literal) => match literal {
//                 LiteralExpression::Number(num) => num.to_string(),
//                 LiteralExpression::String(string) => string.to_string(),
//                 LiteralExpression::Boolean(boolean) => boolean.to_string(),
//                 LiteralExpression::Nil => "nil".to_string(),
//             },
//             Expr::Unary(unary) => unary.operator.to_string(),
//             Expr::Binary(binary) => binary.operator.to_string(),
//             Expr::Grouping(grouping) => grouping.to_string(),
//         }
//     }
// }

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
