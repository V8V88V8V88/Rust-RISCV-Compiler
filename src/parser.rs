use crate::ast::{Expr, Stmt};
use crate::lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut iter = tokens.into_iter().peekable();
    let mut statements = Vec::new();

    while iter.peek().is_some() {
        if let Some(stmt) = parse_statement(&mut iter) {
            statements.push(stmt);
        }
    }

    statements
}

fn parse_statement(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Stmt> {
    match iter.peek() {
        Some(Token::Let) => parse_let_statement(iter),
        Some(Token::If) => parse_if_statement(iter),
        _ => parse_expression_statement(iter),
    }
}

fn parse_let_statement(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Stmt> {
    iter.next();
    if let Some(Token::Identifier(name)) = iter.next() {
        if let Some(Token::Equals) = iter.next() {
            if let Some(expr) = parse_expression(iter) {
                if let Some(Token::Semicolon) = iter.next() {
                    return Some(Stmt::Assign(name, expr));
                }
            }
        }
    }
    None
}

fn parse_if_statement(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Stmt> {
    iter.next();
    if let Some(Token::LeftParen) = iter.next() {
        if let Some(condition) = parse_expression(iter) {
            if let Some(Token::RightParen) = iter.next() {
                if let Some(then_branch) = parse_block(iter) {
                    let else_branch = if let Some(Token::Else) = iter.peek() {
                        iter.next();
                        parse_block(iter)
                    } else {
                        None
                    };
                    return Some(Stmt::Expr(Expr::If(
                        Box::new(condition),
                        Box::new(then_branch),
                        else_branch.map(Box::new),
                    )));
                }
            }
        }
    }
    None
}

fn parse_expression_statement(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>,
) -> Option<Stmt> {
    if let Some(expr) = parse_expression(iter) {
        if let Some(Token::Semicolon) = iter.next() {
            return Some(Stmt::Expr(expr));
        }
    }
    None
}

fn parse_expression(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Expr> {
    parse_binary_expression(iter)
}

fn parse_binary_expression(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>,
) -> Option<Expr> {
    let mut left = parse_primary(iter)?;

    while let Some(op) = iter.peek() {
        let binary_op = match op {
            Token::Plus => Some(Expr::Add as fn(Box<Expr>, Box<Expr>) -> Expr),
            Token::Minus => Some(Expr::Sub as fn(Box<Expr>, Box<Expr>) -> Expr),
            Token::Asterisk => Some(Expr::Mul as fn(Box<Expr>, Box<Expr>) -> Expr),
            Token::Slash => Some(Expr::Div as fn(Box<Expr>, Box<Expr>) -> Expr),
            Token::GreaterThan => Some(Expr::Greater as fn(Box<Expr>, Box<Expr>) -> Expr),
            _ => None,
        };

        if let Some(op) = binary_op {
            iter.next();
            if let Some(right) = parse_primary(iter) {
                left = op(Box::new(left), Box::new(right));
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Some(left)
}

fn parse_primary(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Expr> {
    match iter.next() {
        Some(Token::Integer(i)) => Some(Expr::Const(i)),
        Some(Token::Identifier(s)) => Some(Expr::Var(s)),
        Some(Token::LeftParen) => {
            let expr = parse_expression(iter);
            if let Some(Token::RightParen) = iter.next() {
                expr
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_block(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Option<Stmt> {
    if let Some(Token::LeftBrace) = iter.next() {
        let mut statements = Vec::new();
        while let Some(stmt) = parse_statement(iter) {
            statements.push(stmt);
            if let Some(Token::RightBrace) = iter.peek() {
                iter.next();
                return Some(Stmt::Block(statements));
            }
        }
    }
    None
}
