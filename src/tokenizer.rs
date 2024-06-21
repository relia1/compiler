// tokenizer
use std::iter::{self, from_fn};
use std::str::FromStr;

#[macro_export]
macro_rules! pat {
    ['{'] => {
        $crate::tokenizer::Token::OpenBrace
    };
    ['}'] => {
        $crate::tokenizer::Token::ClosedBrace
    };
    ['('] => {
        $crate::tokenizer::Token::OpenParen
    };
    [')'] => {
        $crate::tokenizer::Token::ClosedParen
    };
    [';'] => {
        $crate::tokenizer::Token::Semicolon
    };
    [int] => {
        $crate::tokenizer::Token::KeywordInt
    };
    [return] => {
        $crate::tokenizer::Token::KeywordReturn
    };
    [ident] => {
        $crate::tokenizer::Token::Identifier
    };
    [u32] => {
        $crate::tokenizer::Token::IntLiteral
    };
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // {
    OpenBrace,
    // }
    ClosedBrace,
    // (
    OpenParen,
    // )
    ClosedParen,
    // ;
    Semicolon,
    // int
    KeywordInt,
    // return
    KeywordReturn,
    // [a-zA-Z]\w*
    Identifier(String),
    // [0-9]+
    IntLiteral(u32),
    //
    Program,
    Function,
}


#[derive(Debug, Clone)]
pub struct Tokens {
    pub all_tokens: Vec<Token>,
    //pub start: u64,
    //pub end: u64,
}


#[derive(Debug, PartialEq, Eq)]
pub struct ParseTokenError;


// used this as a reference
// https://brunocalza.me/writing-a-simple-lexer-in-rust/
impl FromStr for Tokens {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Tokens, Self::Err> {
        let lines = s.lines();
        let mut tokens: Vec<Token> = vec![];

        lines.for_each(|line| {
            let mut char_iter = line.chars().peekable();
            while let Some(character) = char_iter.next() {
                let token = match character {
                    ' ' | '\n' => continue,
                    '{' => Token::OpenBrace,
                    '}' => Token::ClosedBrace,
                    '(' => Token::OpenParen,
                    ')' => Token::ClosedParen,
                    ';' => Token::Semicolon,
                    '0'..='9' => {
                        let num: u32 = iter::once(character)
                            .chain(from_fn(|| char_iter.by_ref().next_if(|c| c.is_ascii_digit())))
                            .collect::<String>()
                            .parse()
                            .unwrap();
                        Token::IntLiteral(num)
                    }
                    'a'..='z' | 'A'..='Z' => {
                        let iden: String = iter::once(character)
                            .chain(from_fn(|| char_iter.by_ref().next_if(|c| c.is_alphanumeric())))
                            .collect::<String>();
                        match iden.as_str() {
                            "return" => Token::KeywordReturn,
                            "int" => Token::KeywordInt,
                            _ => Token::Identifier(iden),
                        }
                    },
                    _ => {
                        println!("Unexpected match to _");
                        println!("tokens: {:?}", tokens);
                        let var_name = Err(ParseTokenError);
                        return var_name.expect("REASON");
                    },
                };
                tokens.push(token);
            };
        });
        Ok(Self { all_tokens: tokens })
    }
}


