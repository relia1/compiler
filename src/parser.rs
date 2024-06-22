use core::panic;
use std::slice::Iter;

use crate::{
    ast::{Ast, TokenType},
    Token, Tokens,
};

// <program> ::= <function>
pub fn parse_program(tokens: &Tokens) -> Ast {
    let binding = tokens.clone();
    let token_iter = binding.all_tokens.iter().collect::<Vec<_>>();
    let mut root = Ast {
        token_type: TokenType::AstRoot,
        children: vec![],
    };
    root.children.push(parse_function(&mut token_iter.iter()));
    root
}

// <function> ::= "int" <id> "(" ")" "{" <statement> "}"
pub fn parse_function(token_iter: &mut Iter<&Token>) -> Ast {
    let mut token = token_iter.next();
    if token.is_none() {
        panic!("Token should not be none");
    };

    match token {
        Some(Token::KeywordInt) => {}
        _ => panic!("Expected keyword int"),
    };

    token = token_iter.next();
    let function_name = match token {
        Some(Token::Identifier(val)) => val,
        _ => panic!("Expected identifier"),
    };

    let mut function_node = Ast {
        token_type: TokenType::Function(function_name.clone()),
        children: vec![],
    };

    token = token_iter.next();
    match token {
        Some(Token::OpenParen) => {}
        _ => panic!("Expected open paren '(' after function name"),
    };

    token = token_iter.next();
    match token {
        Some(Token::ClosedParen) => {}
        _ => panic!("Expected closing paren ')' after function name"),
    };

    token = token_iter.next();
    match token {
        Some(Token::OpenBrace) => {}
        _ => panic!("Expected open brace '{{' after function name"),
    };

    function_node.children.push(parse_statement(token_iter));
    function_node
}

// <statement> ::= "return" <exp> ";"
pub fn parse_statement(token_iter: &mut Iter<&Token>) -> Ast {
    let mut token = token_iter.next();
    match token {
        Some(Token::KeywordReturn) => {}
        _ => panic!("Expected keyword return"),
    };

    let mut statement_node = Ast {
        token_type: TokenType::Body,
        children: vec![],
    };

    statement_node.children.push(parse_exp(token_iter));

    token = token_iter.next();
    match token {
        Some(Token::Semicolon) => {}
        val => panic!("Expected ';' at end of statement, found {:?}", val),
    };

    token = token_iter.next();
    match token {
        Some(Token::ClosedBrace) => {}
        _ => panic!("Expected closing brace }}"),
    };

    statement_node
}

// <exp> ::= <int>
pub fn parse_exp(token_iter: &mut Iter<&Token>) -> Ast {
    //let mut peeked_token = token_iter.peekable();
    //let mut binding = token_iter.peekable();
    let number = match token_iter.next() {
        //binding.peek() {
        Some(Token::IntLiteral(num)) => num,
        val => panic!("Expected integer literal found {:?}", val),
    };

    Ast {
        token_type: TokenType::Expression(*number),
        children: vec![],
    }
}
