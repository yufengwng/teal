//! Module for structures to represent language as an abstract syntax tree.

use std::fmt;

#[derive(Debug)]
pub enum TokenKind<'a> {
    // Operators
    /// `+`
    Plus,
    /// `-`
    Minus,

    // Literals
    /// Numeric literal, e.g. `123`
    Int(&'a str),
    //Real(&'a str),
    //Str { raw: &'a str, terminated: bool },
    //Ident(&'a str),

    // Comments
    /// Line comment starting with `#` and consuming the ending newline
    Hash(&'a str),
    //Doc(&'a str),
    //InnerDoc(&'a str),

    // Markers
    /// Newline
    NL,
    /// Whitespace
    WS,
    /// Unknown or unexpected character
    Unknown(char),
}

impl<'a> TokenKind<'a> {
    pub fn len(&self) -> usize {
        use TokenKind::*;
        match self {
            Int(s) => s.len(),
            Hash(s) => s.len() + 1,
            _ => 1,
        }
    }
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn len(&self) -> usize {
        self.kind.len()
    }
}

pub enum BinOp {
    Add,
    Sub,
}

pub enum LitKind {
    Int(u64),
}

pub enum ExprKind {
    Bin(BinOp, Box<Expr>, Box<Expr>),
    Lit(LitKind),
}

pub struct Expr {
    pub kind: ExprKind,
    pub line: usize,
}

//pub enum StmtKind<'a> {
//    Loop(Option<Expr<'a>>, Vec<Item<'a>>),
//    Assign(Token<'a>, Expr<'a>),
//    Ret(Option<Expr<'a>>),
//    Break,
//    Cont,
//}
//
//pub struct Stmt<'a> {
//    pub kind: StmtKind<'a>,
//    pub token: Token<'a>,
//}
//
//pub enum DeclKind<'a> {
//    Let(Token<'a>, Expr<'a>),
//    Fun(&'a str, Vec<&'a str>, Vec<Item<'a>>),
//}
//
//pub struct Decl<'a> {
//    pub kind: DeclKind<'a>,
//    pub token: Token<'a>,
//}

pub enum Item {
    //Declaration(Decl<'a>),
    //Statement(Stmt<'a>),
    Expression(Expr),
}
