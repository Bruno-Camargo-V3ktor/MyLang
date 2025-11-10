use std::{any::Any, char, collections::HashMap};
use crate::tokens::{Token, TokenType};

pub struct Scanner<'a> {
    start: usize,
    current: usize,
    line: usize,
    source: &'a str,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source,
            tokens: vec![],
            keywords: HashMap::from([
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("null".to_string(), TokenType::Null),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
            ])
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        false
    }

    fn is_digit(&self, c: char) -> bool {
        false
    }

    fn is_alpha_and_numeric(&self, c: char) -> bool {
        false
    }

    fn match_next(&self, expect: char) -> bool {
        false
    } 

    fn scan_token(&self) {

    }

    fn advance(&self) -> char{
        ' '
    }

    fn add_token(&self, token: TokenType, literal: Option<Box<dyn Any>>) {

    } 

    fn peek(&self) -> char {
        '\0'
    }

    fn peek_next(&self) -> char {
        ' '
    }

    fn string(&self) {

    }

    fn number(&self) {

    }

    fn identifier(&self) {
        
    }

}