use std::str::Chars;

use crate::{
    helpers::{abs, app, bit, gate, ket, var},
    term::Term,
};

enum Token {
    LPar(usize),
    RPar(usize),
    LKet(usize),
    RKet(usize),
    Bit(bool),
    Lam(usize),
    Gate(String),
    Var(String),
}

fn tokenize(input: &mut Chars) -> Vec<Token> {
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
            '|' => next_token = Some(Token::LKet(pos)),
            '>' => next_token = Some(Token::RKet(pos)),
            '0' => next_token = Some(Token::Bit(false)),
            '1' => next_token = Some(Token::Bit(true)),
            'H' => next_token = Some(Token::Gate("H".into())),
            'C' => next_token = Some(Token::Gate("C".into())),
            'T' => next_token = Some(Token::Gate("T".into())),
            _ => {
                cur.push(c);
                continue;
            }
        }

        if cur.len() > 0 {
            res.push(Token::Var(cur));
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
    UnopenedKet(usize),
    UnclosedKet(usize),
    LoneQubit(usize),
    MissingVar(usize),
    MissingBody(usize),
    EmptyList,
}

fn parse_tokens(tokens: &[Token]) -> Result<Term, ParseError> {
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
                                let inner = parse_tokens(&tokens[i + 1..=j - 1])?;
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
            Token::LKet(pos) => {
                let mut j = i + 1;
                let mut pushed = false;
                let mut k = Vec::new();
                while j < tokens.len() {
                    match tokens[j] {
                        Token::Bit(val) => {
                            k.push(val);
                        }
                        Token::RKet(_) => {
                            res.push(ket(k));
                            pushed = true;
                            break;
                        }
                        _ => return Err(ParseError::UnclosedKet(*pos)),
                    }
                    j += 1;
                }

                if !pushed {
                    return Err(ParseError::UnclosedKet(*pos));
                }
                i = j;
            }
            Token::RKet(pos) => return Err(ParseError::UnopenedKet(*pos)),
            Token::Bit(b) => res.push(bit(*b)),
            Token::Lam(pos) => {
                if tokens.len() <= i + 2 {
                    return Err(ParseError::MissingBody(*pos));
                }
                if let Some(Token::Var(x)) = tokens.get(i + 1) {
                    let rest = parse_tokens(&tokens[i + 2..])?;
                    res.push(abs(x, rest));
                    i = tokens.len();
                } else {
                    return Err(ParseError::MissingVar(*pos));
                }
            }
            Token::Var(x) => {
                res.push(var(x));
            }
            Token::Gate(g) => {
                res.push(gate(g));
            }
        }
        i += 1;
    }
    match res.into_iter().reduce(|acc, item| app(acc, item)) {
        Some(res) => Ok(res),
        None => Err(ParseError::EmptyList),
    }
}

pub fn parse(input: &mut Chars) -> Result<Term, ParseError> {
    let tokens = tokenize(input);
    parse_tokens(&tokens)
}
