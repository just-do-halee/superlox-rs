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

    #[derive(Clone)]
    pub enum TokenLiteral {
        Identifier(String),
        String(String),
        Number(Number),
        None,
    }

    #[derive(Clone)]
    pub struct Token<'s> {
        pub kind: TokenKind,
        pub lexeme: SourceChunk<'s>,
        pub literal: TokenLiteral,
    }

    #[derive(Clone)]
    pub struct Tokens<'s> {
        pub body: Vec<Token<'s>>,
    }
}

pub type TokenIntoIter<'s> = IntoIter<Token<'s>>;

impl Display for TokenLiteral {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLiteral::None => write!(f, "nil"),
            TokenLiteral::Identifier(i) => write!(f, "{}", i),
            TokenLiteral::String(s) => write!(f, "{:?}", s),
            TokenLiteral::Number(n) => write!(f, "{}", n),
        }
    }
}

pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {

        "and"    => TokenKind::And,
        "class"  => TokenKind::Class,
        "else"   => TokenKind::Else,
        "false"  => TokenKind::False,
        "for"    => TokenKind::For,
        "fun"    => TokenKind::Fun,
        "if"     => TokenKind::If,
        "nil"    => TokenKind::Nil,
        "or"     => TokenKind::Or,
        "print"  => TokenKind::Print,
        "return" => TokenKind::Return,
        "super"  => TokenKind::Super,
        "this"   => TokenKind::This,
        "true"   => TokenKind::True,
        "var"    => TokenKind::Var,
        "while"  => TokenKind::While,

};

#[inline]
pub fn __parse_keyword(keyword: &str) -> Option<TokenKind> {
    KEYWORDS.get(keyword).cloned()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Number(pub f64);

impl Eq for Number {}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for Number {
    #[inline]
    fn from(f: f64) -> Self {
        Number(f)
    }
}

impl FromStr for Number {
    type Err = Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number(
            f64::from_str(s).with_context(fnerr!("{} (parse) ", s))?,
        ))
    }
}

impl<'s> Token<'s> {
    #[inline]
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
    #[inline]
    pub fn into_eof(mut self) -> Self {
        self.kind = TokenKind::Eof;
        self.lexeme.clear();
        self.literal = TokenLiteral::None;
        self
    }
}

impl<'s> FreeErrorConverter for Token<'s> {
    #[inline]
    fn to_error_with_kind<D: Display>(&self, kind: ErrKind, message: D) -> Error {
        self.lexeme.to_error_with_kind(kind, message)
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
    #[inline]
    pub fn push_token(&mut self, token: Token<'s>) -> &mut Self {
        self.body.push(token);
        self
    }
}

impl<'s> AsRef<Tokens<'s>> for Tokens<'s> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s> From<Vec<Token<'s>>> for Tokens<'s> {
    #[inline]
    fn from(body: Vec<Token<'s>>) -> Self {
        Self { body }
    }
}

impl<'s> IntoIterator for Tokens<'s> {
    type Item = Token<'s>;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.body.into_iter()
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
