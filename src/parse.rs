use std::iter::Iterator;

use crate::ast;
use crate::ast::Token;
use crate::ast::TokenKind;
use crate::lex::TokenStream;

pub fn parse<'a>(tokens: TokenStream<'a>) -> ItemStream<'a> {
    ItemStream {
        parser: Parser::new(tokens),
    }
}

pub struct ItemStream<'a> {
    parser: Parser<'a>,
}

impl<'a> Iterator for ItemStream<'a> {
    type Item = Result<ast::Item, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next()
    }
}

struct Parser<'a> {
    tokens: TokenStream<'a>,
    curr: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: TokenStream<'a>) -> Self {
        Parser { tokens, curr: None }
    }

    fn next(&mut self) -> Option<Result<ast::Item, String>> {
        match self.next_token() {
            Ok(token) => match token {
                Some(tok) => {
                    self.curr = Some(tok);
                    Some(self.parse())
                }
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }

    fn next_token(&mut self) -> Result<Option<Token<'a>>, String> {
        if self.curr.is_some() {
            return Ok(self.curr.take());
        }
        while let Some(token) = self.tokens.next() {
            match token.kind {
                TokenKind::WS => continue,
                TokenKind::NL => continue,
                TokenKind::Hash(_) => continue,
                TokenKind::Unknown(ch) => {
                    let err = format!("[line {}] unrecognized token '{}'", token.line, ch);
                    return Err(err);
                }
                _ => return Ok(Some(token)),
            }
        }
        Ok(None)
    }

    fn parse(&mut self) -> Result<ast::Item, String> {
        let expr = self.parse_expr()?;
        Ok(ast::Item::Expression(expr))
    }

    fn parse_expr(&mut self) -> Result<ast::Expr, String> {
        self.parse_add()
    }

    fn parse_add(&mut self) -> Result<ast::Expr, String> {
        let mut expr = self.parse_primary()?;
        while let Some(token) = self.next_token()? {
            match token.kind {
                TokenKind::Plus => {
                    let op = ast::BinOp::Add;
                    if let Some(next) = self.next_token()? {
                        self.curr = Some(next);
                    } else {
                        let err = format!(
                            "[line {}] parse error at '{}': expected an expression for right-hand side",
                            token.line, token.kind
                        );
                        return Err(err);
                    }
                    let rhs = self.parse_primary()?;
                    expr = ast::Expr {
                        kind: ast::ExprKind::Bin(op, Box::new(expr), Box::new(rhs)),
                        line: token.line,
                    }
                }
                TokenKind::Minus => {
                    let op = ast::BinOp::Sub;
                    if let Some(next) = self.next_token()? {
                        self.curr = Some(next);
                    } else {
                        let err = format!(
                            "[line {}] parse error at '{}': expected an expression for right-hand side",
                            token.line, token.kind
                        );
                        return Err(err);
                    }
                    let rhs = self.parse_primary()?;
                    expr = ast::Expr {
                        kind: ast::ExprKind::Bin(op, Box::new(expr), Box::new(rhs)),
                        line: token.line,
                    }
                }
                _ => {
                    self.curr = Some(token);
                    break;
                }
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<ast::Expr, String> {
        let curr = self.next_token()?.unwrap();
        Ok(match curr.kind {
            TokenKind::Int(s) => match s.parse::<u64>() {
                Ok(num) => ast::Expr {
                    kind: ast::ExprKind::Lit(ast::LitKind::Int(num)),
                    line: curr.line,
                },
                Err(_) => {
                    let err = format!(
                        "[line {}] parse error at '{}': cannot parse as an integer",
                        curr.line, s
                    );
                    return Err(err);
                }
            },
            _ => {
                let err = format!(
                    "[line {}] parse error at '{}': expected an expression",
                    curr.line, curr.kind
                );
                return Err(err);
            }
        })
    }
}
