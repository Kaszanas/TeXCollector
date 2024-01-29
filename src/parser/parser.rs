use logos::Logos;

use crate::lexer::token::Token;

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
        }
    }
}
