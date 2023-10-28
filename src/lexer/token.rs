#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    IntKeyword,
    ReturnKeyword,
    Identifier(Box<String>),
    IntLiteral(u64),
}
