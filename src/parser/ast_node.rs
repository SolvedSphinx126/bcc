use std::fmt;

#[derive(Debug)]
pub struct Program {
    pub fns: Function,
}

#[derive(Debug)]
pub struct Function {
    // IntKeyword Identifier OpenParen CloseParen OpenBrace Statement CloseBrace
    pub name: Identifier,
    pub statements: Statement,
}

#[derive(Debug)]
pub struct Statement {
    // ReturnKeyword Expression Semicolon
    pub result: Expression,
}

#[derive(Debug)]
pub struct Expression {
    // IntLiteral
    pub value: u64,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The parser encountered an error")
    }
}
