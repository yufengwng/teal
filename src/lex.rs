use std::iter::Iterator;
use std::str::CharIndices;

use crate::ast::Token;
use crate::ast::TokenKind;

pub fn tokenize<'a>(src: &'a str) -> TokenStream<'a> {
    TokenStream {
        cursor: Cursor::new(src),
    }
}

pub struct TokenStream<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            None
        } else {
            Some(self.cursor.next())
        }
    }
}

struct Cursor<'a> {
    src: &'a str,
    line: usize,
    iter: CharIndices<'a>,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a str) -> Self {
        Cursor {
            src: src,
            line: 1,
            iter: src.char_indices(),
        }
    }

    fn is_eof(&self) -> bool {
        self.src.is_empty()
    }

    fn next(&mut self) -> Token<'a> {
        let line = self.line;
        let kind = self.scan();
        let token = Token { kind, line };
        self.slide(token.len());
        token
    }

    fn slide(&mut self, len: usize) {
        self.src = &self.src[len..];
        self.iter = self.src.char_indices();
    }

    fn scan(&mut self) -> TokenKind<'a> {
        let ch = self.advance();
        match ch {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '\n' => {
                self.line += 1;
                TokenKind::NL
            }
            '#' => self.finish_comment(),
            c if is_digit(c) => self.finish_number(),
            c if is_whitespace(c) => TokenKind::WS,
            _ => TokenKind::Unknown(ch),
        }
    }

    fn advance(&mut self) -> char {
        self.iter.next().map(|(_, ch)| ch).unwrap()
    }

    fn finish_comment(&mut self) -> TokenKind<'a> {
        let end = self.consume_to_eol();
        TokenKind::Hash(&self.src[..end + 1])
    }

    fn consume_to_eol(&mut self) -> usize {
        while let Some((idx, ch)) = self.iter.next() {
            if ch == '\n' {
                self.line += 1;
                return idx;
            }
        }
        self.src.len() - 1
    }

    fn finish_number(&mut self) -> TokenKind<'a> {
        let mut end = 0;
        while let Some((idx, ch)) = self.iter.next() {
            if is_digit(ch) {
                end = idx;
            } else {
                break;
            }
        }
        TokenKind::Int(&self.src[..end + 1])
    }
}

pub fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}
