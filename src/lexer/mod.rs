use regex::Regex;
use std::{fs::File, io::Read};

pub mod token;
use self::token::Token;


pub fn lex(mut src: File) -> Vec<Token> {
    let token_regex = r"\{|}|\(|\)|;|int|return|[a-zA-Z]\w*|[0-9]+";
    let token_regex = Regex::new(token_regex).unwrap();
    let mut buf = String::new();
    src.read_to_string(&mut buf).unwrap();
    token_regex
        .find_iter(&buf)
        .map(|re_match| re_match.as_str().to_owned())
        .map(|str| get_token(&str))
        .collect()
}

fn get_token(str: &str) -> Token {
    let res = [
        Regex::new(r"^\{$").unwrap(),
        Regex::new(r"^}$").unwrap(),
        Regex::new(r"^\($").unwrap(),
        Regex::new(r"^\)$").unwrap(),
        Regex::new(r"^;$").unwrap(),
        Regex::new(r"^int$").unwrap(),
        Regex::new(r"^return$").unwrap(),
        Regex::new(r"^[a-zA-Z]\w*$").unwrap(),
        Regex::new(r"^[0-9]+$").unwrap(),
    ];
    {
        use token::Token::*;
        if res[0].is_match(str) {
            return OpenBrace;
        }
        else if res[1].is_match(str) {
            return CloseBrace;
        }
        else if res[2].is_match(str) {
            return OpenParen;
        }
        else if res[3].is_match(str) {
            return CloseParen;
        }
        else if res[4].is_match(str) {
            return Semicolon;
        }
        else if res[5].is_match(str) {
            return IntKeyword;
        }
        else if res[6].is_match(str) {
            return ReturnKeyword;
        }
        else if res[7].is_match(str) {
            return Identifier(Box::new(str.to_owned()));
        }
        else if res[8].is_match(str) {
            return IntLiteral(str.parse::<u64>().expect("cant parse int literal"));
        }
    }
    
    panic!("Token wasn't found");
}
