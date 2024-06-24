// Ast
// use crate::Token;

#[derive(Debug)]
pub struct Ast {
    // pub token: Token,
    pub token_type: TokenType,
    pub children: Vec<Ast>,
}

pub trait ToAsm {
    fn to_asm(&self) -> String;
}

#[derive(Debug)]
pub enum TokenType {
    Program,
    Expr,
    Statement,
    UnaryOp,
}

#[derive(Debug)]
pub enum Program {
    AstRoot,
}

#[derive(Debug)]
pub enum Expr {
    Number(u32),
    FunctionCall(String, Vec<Expr>),
}


#[derive(Debug)]
pub enum Statement {
    Expression(Expr),
    Return(Expr),
    // Name, Params, Body
    Function(String, Vec<String>, Vec<Statement>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Negation,
    LogicalNegation,
    BitwiseComplement,
}

impl ToAsm for Program {
    fn to_asm(&self) -> String {
        format!(".globl {}\n_main", )
    }
}
