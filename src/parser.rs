use core::panic;
use std::slice::Iter;
use thiserror::Error;

use crate::{
    ast::{Ast, TokenType},
    Token, Tokens,
};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Expected token {expected:?}, found {found:?}")]
    UnexpectedToken { expected: &'static str, found: Token },
    #[error("Unexpected end of input")]
    UnexpectedEOF,
}

// <program> ::= <function>
pub fn parse_program(tokens: &Tokens) -> Result<Ast, ParseError> {
    let mut token_iter = tokens.all_tokens.iter();// .clone();
    let mut root = Ast {
        token_type: TokenType::AstRoot,
        children: vec![],
    };
    root.children.push(parse_function(&mut token_iter)?);
    Ok(root)
}

// <function> ::= "int" <id> "(" ")" "{" <statement> "}"
pub fn parse_function(token_iter: &mut Iter<Token>) -> Result<Ast, ParseError> {
    let mut next_token = || token_iter.next().ok_or(ParseError::UnexpectedEOF);
    match next_token()? {
        Token::KeywordInt => {},
        token => return Err(ParseError::UnexpectedToken { expected: "int", found: token.clone() }),
    }

    let function_name = match next_token()? {
        Token::Identifier(val) => val.clone(),
        token => return Err(ParseError::UnexpectedToken { expected: "identifier", found: token.clone() }),
    };

    let mut function_node = Ast {
        token_type: TokenType::Function(function_name),
        children: vec![],
    };

    match next_token()? {
        Token::OpenParen => {}
        token => return Err(ParseError::UnexpectedToken { expected: "((", found: token.clone() })
    };

    match next_token()? {
        Token::ClosedParen => {}
        token => return Err(ParseError::UnexpectedToken { expected: "))", found: token.clone() })
    };

    match next_token()? {
        Token::OpenBrace => {}
        token => return Err(ParseError::UnexpectedToken { expected: "{{", found: token.clone() })
    };

    function_node.children.push(parse_statement(token_iter));
    Ok(function_node)
}

// <statement> ::= "return" <exp> ";"
pub fn parse_statement(token_iter: &mut Iter<Token>) -> Ast {
    let token = token_iter.next();
    match token {
        Some(Token::KeywordReturn) => {}
        _ => panic!("Expected keyword return"),
    };

    let mut statement_node = Ast {
        token_type: TokenType::Body,
        children: vec![],
    };

    statement_node.children.push(parse_exp(token_iter));

    match token_iter.next() {
        Some(Token::Semicolon) => {}
        val => panic!("Expected ';' at end of statement, found {:?}", val),
    };

    match token_iter.next() {
        Some(Token::ClosedBrace) => {}
        _ => panic!("Expected closing brace }}"),
    };

    statement_node
}

// <exp> ::= <int>
pub fn parse_exp(token_iter: &mut Iter<Token>) -> Ast {
    let number = match token_iter.next() {
        Some(Token::IntLiteral(num)) => *num,
        _ => panic!("Expected integer literal"),
    };

    Ast {
        token_type: TokenType::Expression(number),
        children: vec![],
    }
}
