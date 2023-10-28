use std::slice::Iter;

use crate::lexer::token::Token;

use self::ast_node::*;
pub mod ast_node;

// These need to all be results
pub fn parse(tokens: Vec<Token>) -> Result<Program, ParserError> {
    let mut tok_iter = tokens.iter();
    parse_program(&mut tok_iter)
}

// <program> ::= <function>
pub fn parse_program(tokens: &mut Iter<Token>) -> Result<Program, ParserError> {
    let mut tokens2 = tokens.clone();
    if let Ok(func) = parse_function(&mut tokens2) {
        *tokens = tokens2;
        Ok(Program { fns: func })
    } else {
        Err(ParserError)
    }
}

// <function> ::= IntKeyword <identifier> OpenParen CloseParen OpenBrace <statement> CloseBrace
pub fn parse_function(tokens: &mut Iter<Token>) -> Result<Function, ParserError> {
    let mut tokens2 = tokens.clone();
    if tokens2.next() != Some(&Token::IntKeyword) {
        return Err(ParserError);
    }
    if let Ok(name) = parse_identifier(&mut tokens2) {
        if tokens2.next() != Some(&Token::OpenParen) {
            return Err(ParserError);
        }
        if tokens2.next() != Some(&Token::CloseParen) {
            return Err(ParserError);
        }
        if tokens2.next() != Some(&Token::OpenBrace) {
            return Err(ParserError);
        }
        if let Ok(statement) = parse_statement(&mut tokens2) {
            if tokens2.next() != Some(&Token::CloseBrace) {
                Err(ParserError)
            } else {
                *tokens = tokens2;
                Ok(Function {
                    name,
                    statements: statement,
                })
            }
        } else {
            Err(ParserError)
        }
    } else {
        Err(ParserError)
    }
}

// <identifier> ::= Identifier
pub fn parse_identifier(tokens: &mut Iter<Token>) -> Result<Identifier, ParserError> {
    let mut tokens2 = tokens.clone();
    if let Some(Token::Identifier(val)) = tokens2.next() {
        *tokens = tokens2;
        Ok(Identifier {
            value: *val.clone(),
        })
    } else {
        Err(ParserError)
    }
}

// <function> ::= ReturnKeyword <expression> Semicolon
pub fn parse_statement(tokens: &mut Iter<Token>) -> Result<Statement, ParserError> {
    let mut tokens2 = tokens.clone();
    if tokens2.next() != Some(&Token::ReturnKeyword) {
        return Err(ParserError);
    }
    if let Ok(exp) = parse_expression(&mut tokens2) {
        if tokens2.next() != Some(&Token::Semicolon) {
            Err(ParserError)
        } else {
            *tokens = tokens2;
            Ok(Statement { result: exp })
        }
    } else {
        Err(ParserError)
    }
}

// <expression> ::= IntLiteral
pub fn parse_expression(tokens: &mut Iter<Token>) -> Result<Expression, ParserError> {
    let mut tokens2 = tokens.clone();
    if let Some(Token::IntLiteral(num)) = tokens2.next() {
        *tokens = tokens2;
        Ok(Expression { value: *num })
    } else {
        Err(ParserError)
    }
}
