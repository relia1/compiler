// Ast
use crate::Token;

#[derive(Debug)]
pub struct Ast {
    pub token: Token,
    pub token_type: TokenType,
    pub children: Vec<Ast>,
    pub visited: bool,
}

#[derive(Debug)]
pub enum TokenType {
    AstRoot,
    Function(String),
    Body,
    Expression(u32)
}
