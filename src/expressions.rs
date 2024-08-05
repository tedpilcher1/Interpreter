pub enum Expr {
    Literal(LiteralExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

struct BinaryExpression {
    left: Box<Expr>,
    operator: TokenType,
    right: Box<Expr>,
}

struct UnaryExpression {
    operator: TokenType,
    right: Box<Expr>,
}

pub enum LiteralExpression {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

pub struct GroupingExpression {
    expression: Box<Expr>,
}
