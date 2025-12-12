use std::str::Chars;

use crate::{
    helpers::{abs, app, gate, ket, meas, nonlinear, nonlinear_abs, var},
    term::Term,
};

#[derive(Debug, Clone)]
enum Token {
    LPar(usize),
    RPar(usize),
    LKet(usize),
    RKet(usize),
    Bit(usize, bool),
    Lam(usize),
    NonlinearLam(usize),
    Nonlinear,
    Gate(String),
    Var(String),
    Meas,
}

fn tokenize(input: &mut Chars) -> Vec<Token> {
    let mut res = Vec::new();
    let mut cur = String::new();
    let mut pos = 0;

    for c in input.by_ref() {
        pos += 1;
        let mut next_token = None;
        match c {
            c if c.is_whitespace() || c == '.' => (),
            '\\' | 'λ' => next_token = Some(Token::Lam(pos)),
            '#' => next_token = Some(Token::NonlinearLam(pos)),
            '!' => next_token = Some(Token::Nonlinear),
            '(' => next_token = Some(Token::LPar(pos)),
            ')' => next_token = Some(Token::RPar(pos)),
            '|' => next_token = Some(Token::LKet(pos)),
            '>' => next_token = Some(Token::RKet(pos)),
            '0' | '1' if !cur.is_empty() => {
                cur.push(c);
                continue;
            }
            '0' => next_token = Some(Token::Bit(pos, false)),
            '1' => next_token = Some(Token::Bit(pos, true)),
            'H' => next_token = Some(Token::Gate("H".into())),
            'C' => next_token = Some(Token::Gate("C".into())),
            'T' => next_token = Some(Token::Gate("T".into())),
            'M' => next_token = Some(Token::Meas),
            _ => {
                cur.push(c);
                continue;
            }
        }

        if !cur.is_empty() {
            res.push(Token::Var(cur));
            cur = String::new();
        }

        if let Some(token) = next_token { res.push(token) }
    }

    if !cur.is_empty() {
        res.push(Token::Var(cur));
    }

    res
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedPar(usize),
    UnopenedPar(usize),
    UnopenedKet(usize),
    UnclosedKet(usize),
    UnusedNonlinear,
    LoneQubit(usize),
    MissingVar(usize),
    MissingBody(usize),
    EmptyList,
}

fn parse_tokens(tokens: &[Token]) -> Result<Term, ParseError> {
    let mut i = 0;
    let mut res = Vec::new();
    let mut next_nonlinear = false;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Nonlinear => {
                next_nonlinear = true;
            }
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
                                let mut inner = parse_tokens(&tokens[i + 1..=j - 1])?;
                                if next_nonlinear {
                                    inner = nonlinear(inner);
                                    next_nonlinear = false;
                                }
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
                if i < tokens.len() - 2 {
                    match (&tokens[i + 1], &tokens[i + 2]) {
                        (Token::Bit(_, b), Token::RKet(_)) => {
                            res.push(ket(*b));
                            i += 3;
                            continue;
                        }
                        _ => return Err(ParseError::UnclosedKet(*pos)),
                    }
                }
                return Err(ParseError::UnclosedKet(*pos));
            }
            Token::RKet(pos) => return Err(ParseError::UnopenedKet(*pos)),
            Token::Bit(pos, _) => return Err(ParseError::LoneQubit(*pos)),
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
            Token::NonlinearLam(pos) => {
                if tokens.len() <= i + 2 {
                    return Err(ParseError::MissingBody(*pos));
                }
                if let Some(Token::Var(x)) = tokens.get(i + 1) {
                    let rest = parse_tokens(&tokens[i + 2..])?;
                    res.push(nonlinear_abs(x, rest));
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
            Token::Meas => {
                res.push(meas());
            }
        }
        i += 1;
    }

    if next_nonlinear {
        return Err(ParseError::UnusedNonlinear);
    }

    if res.len() == 1 {
        Ok(res.into_iter().next().unwrap())
    } else {
        match res.into_iter().reduce(app) {
            Some(res) => Ok(res),
            None => Err(ParseError::EmptyList),
        }
    }
}

pub fn parse(input: &mut Chars) -> Result<Term, ParseError> {
    let tokens = tokenize(input);
    parse_tokens(&tokens)
}
