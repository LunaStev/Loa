use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Fun,
    If,
    Else,
    While,
    For,
    Return,
    Continue,
    Break,
    Print,
    Println,
    Input,

    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),

    Plus,         // +
    Minus,        // -
    Star,         // *
    Div,          // /
    Equal,        // =
    EqualTwo,     // ==
    NotEqual,     // !=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    LogicalAnd,   // &&
    LogicalOr,    // ||
    Not,          // !

    Lparen,       // (
    Rparen,       // )
    Colon,        // :
    Comma,        // ,

    Eof,
    Error,
    Whitespace,
}