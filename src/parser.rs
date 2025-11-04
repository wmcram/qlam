use std::str::Chars;

use crate::{
    helpers::{abs, app, var},
    term::Term,
};

pub enum Token {
    LPar(usize),
    RPar(usize),
    Lam(usize),
    Var(usize, String),
}

pub fn tokenize(input: &mut Chars) -> Vec<Token> {
    let mut res = Vec::new();
    let mut cur = String::new();
    let mut pos = 0;

    while let Some(c) = input.next() {
        pos += 1;
        let mut next_token = None;
        match c {
            '\\' | 'λ' => next_token = Some(Token::Lam(pos)),
            c if c.is_whitespace() || c == '.' => (),
            '(' => next_token = Some(Token::LPar(pos)),
            ')' => next_token = Some(Token::RPar(pos)),
            _ => {
                cur.push(c);
                continue;
            }
        }

        if cur.len() > 0 {
            res.push(Token::Var(pos, cur));
            cur = String::new();
        }

        match next_token {
            Some(token) => res.push(token),
            None => (),
        }
    }
    res
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedPar(usize),
    UnopenedPar(usize),
    MissingVar(usize),
    MissingBody(usize),
    EmptyList,
}

pub fn parse(tokens: &[Token]) -> Result<Term, ParseError> {
    let mut i = 0;
    let mut res = Vec::new();
    while i < tokens.len() {
        match &tokens[i] {
            Token::LPar(pos) => {
                let mut depth = 0;
                let mut j = i + 1;
                let mut pushed = false;
                while j < tokens.len() {
                    match tokens[j] {
                        Token::LPar(_) => {
                            depth += 1;
                        }
                        Token::RPar(_) => {
                            if depth == 0 {
                                let inner = parse(&tokens[i + 1..=j - 1])?;
                                res.push(inner);
                                pushed = true;
                                break;
                            }
                            depth -= 1;
                        }
                        _ => (),
                    }
                    j += 1;
                }

                if !pushed {
                    return Err(ParseError::UnclosedPar(*pos));
                }
                i = j;
            }
            Token::RPar(pos) => return Err(ParseError::UnopenedPar(*pos)),
            Token::Lam(pos) => {
                if tokens.len() <= i + 2 {
                    return Err(ParseError::MissingBody(*pos));
                }
                if let Some(Token::Var(_, x)) = tokens.get(i + 1) {
                    let rest = parse(&tokens[i + 2..])?;
                    res.push(abs(x, rest));
                    i = tokens.len();
                } else {
                    return Err(ParseError::MissingVar(*pos));
                }
            }
            Token::Var(_, x) => {
                res.push(var(x));
            }
        }
        i += 1;
    }
    match res.into_iter().reduce(|acc, item| app(acc, item)) {
        Some(res) => Ok(res),
        None => Err(ParseError::EmptyList),
    }
}
