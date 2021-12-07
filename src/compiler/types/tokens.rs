// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

derive_debug_partials! {
    #[derive(Clone, Copy)]
    pub enum TokenKind {
        // Single-character tokens.
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,

        // One or two character tokens.
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // Literals.
        Identifier,
        String,
        Number,

        // Keywords.
        And,
        Class,
        Else,
        False,
        Fun,
        For,
        If,
        Nil,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Var,
        While,

        Eof,
    }

    #[derive(Clone, Copy)]
    pub enum TokenLiteral {
        None,
    }

    #[derive(Clone, Copy)]
    pub struct Token<'s> {
        pub kind: TokenKind,
        pub lexeme: SourceChunk<'s>,
        pub literal: TokenLiteral,
    }

    pub struct Tokens<'s> {
        pub body: Vec<Token<'s>>,
    }
}

impl<'s> Token<'s> {
    pub fn new<S: Into<SourceChunk<'s>>>(
        kind: TokenKind,
        lexeme: S,
        literal: TokenLiteral,
    ) -> Self {
        Token {
            kind,
            lexeme: lexeme.into(),
            literal,
        }
    }
}

impl<'s> Tokens<'s> {
    #[inline]
    pub fn new() -> Self {
        Self { body: Vec::new() }
    }
    #[inline]
    pub fn push<S: Into<SourceChunk<'s>>>(
        &mut self,
        kind: TokenKind,
        lexeme: S,
        literal: TokenLiteral,
    ) -> &mut Self {
        self.body.push(Token::new(kind, lexeme, literal));
        self
    }
}

impl<'s> From<Vec<Token<'s>>> for Tokens<'s> {
    #[inline]
    fn from(body: Vec<Token<'s>>) -> Self {
        Self { body }
    }
}

impl<'s> Deref for Tokens<'s> {
    type Target = Vec<Token<'s>>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl<'s> DerefMut for Tokens<'s> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}
