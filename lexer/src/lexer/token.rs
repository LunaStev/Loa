#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Fun,
    Var,
    Let,
    Mut,
    Deref,
    Const,
    If,
    Else,
    While,
    For,
    Import,
    Return,
    Continue,
    Input,
    Print,
    Println,
    Module,
    Class,
    Match,
    LogicalAnd,            // &&
    AddressOf,            // &
    LogicalOr,             // ||
    BitwiseOr,             // |
    NotEqual,              // !=
    Xor,                    // ^
    Xnor,                   // ~^
    BitwiseNot,            // ~
    Nand,                   // !&
    Nor,                    // !|
    Not,                    // !
    Condition,              // ?
    NullCoalesce,          // ??
    Conditional,            // ?:
    In,                     // in
    Is,                     // is
    Rol,
    Ror,
    Xnand,
    Operator(String),
    TypeInt(u16),
    TypeFloat(u16),
    TypeBool,
    TypeChar,
    TypeByte,
    TypeString,
    TypeArray(Box<TokenType>, u32),
    Identifier(String),
    String(String),
    Number(i64),
    Float(f64),
    Plus,                   // +
    Increment,              // ++
    Minus,                  // -
    Decrement,              // --
    Star,                   // *
    Div,                    // /
    Equal,                  // =
    EqualTwo,              // ==
    Comma,                  // ,
    Dot,                    // .
    SemiColon,              // ;
    Colon,                  // :
    Lchevr,                 // <
    LchevrEq,              // <=
    Rchevr,                 // >
    RchevrEq,              // >=
    Lparen,                 // (
    Rparen,                 // )
    Lbrace,                 // {
    Rbrace,                 // }
    Lbrack,                 // [
    Rbrack,                 // ]
    Eof,                    // End of file
    Error,
    Whitespace,
    Break,
    Arrow,                  // ->
    Array,
}