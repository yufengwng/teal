//! Module for structures to represent language as an abstract syntax tree.

pub enum TokenKind<'a> {
    // Punctuation
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Colon,
    Comma,

    // Operators
    Plus,
    Minus,
    Star,
    Lt,
    Gt,
    Eq_,
    LtEq,
    GtEq,
    EqEq,
    NotEq,

    // Keywords
    Not,
    And,
    Or,
    If,
    Elif,
    Else,
    Loop,
    Break,
    Cont,
    Ret,
    Let,
    Fn_,

    // Literals
    True,
    False,
    Int(&'a str),
    Real(&'a str),
    Str { raw: &'a str, terminated: bool },
    Ident(&'a str),

    // Documentation
    Hash(&'a str),
    Doc(&'a str),
    InnerDoc(&'a str),

    // Markers
    NL,
    Unknown(char),
}

impl<'a> TokenKind<'a> {
    pub fn from_ident(name: &'a str) -> TokenKind {
        use TokenKind::*;
        match name {
            "not" => Not,
            "and" => And,
            "or" => Or,
            "if" => If,
            "elif" => Elif,
            "else" => Else,
            "loop" => Loop,
            "break" => Break,
            "continue" => Cont,
            "return" => Ret,
            "let" => Let,
            "fn" => Fn_,
            "true" => True,
            "false" => False,
            _ => Ident(name),
        }
    }

    pub fn len(&self) -> usize {
        use TokenKind::*;
        match self {
            LtEq | GtEq | EqEq | NotEq => 2,
            Or | If | Fn_ => 2,
            Not | And | Let => 3,
            Elif | Else | Loop => 4,
            Break => 5,
            Ret => 6,
            Cont => 8,
            True => 4,
            False => 5,
            Int(s) => s.len(),
            Real(s) => s.len(),
            Str { raw: s, terminated } => s.len() + if *terminated { 2 } else { 1 },
            Ident(s) => s.len(),
            Hash(s) => s.len() + 1,
            Doc(s) => s.len() + 3,
            InnerDoc(s) => s.len() + 3,
            _ => 1,
        }
    }
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub line: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    pub fn len(&self) -> usize {
        self.kind.len()
    }
}

pub enum LogOp {
    And,
    Or,
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Lt,
    LtEq,
    Gt,
    GtEq,
    EqEq,
    NotEq,
}

pub enum UniOp {
    Neg,
    Not,
}

pub enum ExprKind<'a> {
    If(Vec<(Expr<'a>, Vec<Item<'a>>)>, Option<Vec<Item<'a>>>),
    Log(LogOp, Box<Expr<'a>>, Box<Expr<'a>>),
    Bin(BinOp, Box<Expr<'a>>, Box<Expr<'a>>),
    Uni(UniOp, Box<Expr<'a>>),
    Group(Box<Expr<'a>>),
    Block(Vec<Item<'a>>),
    Name,
    Lit,
}

pub struct Expr<'a> {
    pub kind: ExprKind<'a>,
    pub token: Token<'a>,
}

pub enum StmtKind<'a> {
    Loop(Option<Expr<'a>>, Vec<Item<'a>>),
    Assign(Token<'a>, Expr<'a>),
    Ret(Option<Expr<'a>>),
    Break,
    Cont,
}

pub struct Stmt<'a> {
    pub kind: StmtKind<'a>,
    pub token: Token<'a>,
}

pub enum DeclKind<'a> {
    Let(Token<'a>, Expr<'a>),
    Fun(&'a str, Vec<&'a str>, Vec<Item<'a>>),
}

pub struct Decl<'a> {
    pub kind: DeclKind<'a>,
    pub token: Token<'a>,
}

pub enum Item<'a> {
    Declaration(Decl<'a>),
    Statement(Stmt<'a>),
    Expression(Expr<'a>),
}
