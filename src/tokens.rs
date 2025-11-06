use std::any::Any;

#[derive(Debug)]
pub enum TokenType {
    // Tokens de Caracteres Únicos
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // Tokens de um ou dois Caracteres
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Tokens literais
    Identifier,
    String,
    Number,

    // Palavras-chave
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Token para final de declaração
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        let str_enum = format!("{:?}", self.token_type);
        let mut str_literal = String::from("null");

        if let Some(literal) = &self.literal {
            if let Some(str) = literal.downcast_ref::<String>() {
                str_literal = str.to_string();
            } else if let Some(num) = literal.downcast_ref::<isize>() {
                str_literal = num.to_string();
            } else if let Some(double) = literal.downcast_ref::<f64>() {
                str_literal = double.to_string();
            }
        } else {
            str_literal = "[no literal]".to_string();
        }

        return format!("{str_enum} {} {str_literal}", self.lexeme);
    }
}
