// Copyright 2021 Hwakyeom Kim(=just-do-halee)

//! 1. SourceCursor
//! 2. TokenCursor

use super::*;

#[derive(Clone)]
pub struct SourceCursor<'s> {
    pub source: &'s Source,
    chars: Chars<'s>,
    offset: Offset,
    saved_offset: Offset,
    current_char: char,
}

impl<'s> Display for SourceCursor<'s> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.offset, self.current_char)
    }
}
impl<'s> fmt::Debug for SourceCursor<'s> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t {} <- [{}] \t{}",
            self.preserved(),
            self.current_char,
            self.offset,
            self.remains()
        )
    }
}

impl<'s> Cursor for SourceCursor<'s> {
    type Item = char;

    type Iter = Chars<'s>;

    #[inline]
    fn EOF(&self) -> Self::Item {
        EOF_CHAR
    }

    #[inline]
    fn clone_items(&self) -> Self::Iter {
        self.chars.clone()
    }

    #[inline]
    fn current(&mut self) -> &mut Self::Item {
        &mut self.current_char
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }

    #[inline]
    fn flush(&mut self) {
        let c = self.current_char;

        if c != EOF_CHAR && self.offset.pos < self.source.len() {
            self.offset.pos += 1;

            // new line
            if c == '\n' {
                self.offset.line += 1;
                self.offset.column = 0; // reset
            } else {
                self.offset.column += 1;
            }
        }
    }
}

impl<'s> SourceCursor<'s> {
    #[inline]
    pub fn new<S: Into<SourceChunk<'s>>>(chunk: S) -> Self {
        let chunk = chunk.into();
        SourceCursor {
            source: chunk.source,
            chars: chunk.source.chars(),
            offset: chunk.span().start,
            saved_offset: chunk.span().start,
            current_char: EOF_CHAR,
        }
    }

    #[inline]
    pub fn save_offset(&mut self) {
        self.saved_offset = self.offset;
    }

    #[inline]
    pub fn load_str(&self) -> &str {
        &self.source.body[self.saved_offset.pos..self.offset.pos]
    }

    #[inline]
    pub fn load_span(&self) -> Span {
        Span {
            start: self.saved_offset,
            end: self.offset,
        }
    }

    #[inline]
    pub fn to_single_chunk(&self) -> SourceChunk<'s> {
        SourceChunk::new(
            self.source,
            Span {
                start: {
                    let mut offset = self.offset;
                    if offset.pos > 0 {
                        offset.pos -= 1;
                    }
                    offset
                },
                end: self.offset,
            },
        )
        .unwrap()
    }

    #[inline]
    pub fn to_single_token(&self, kind: TokenKind, literal: Option<&str>) -> Token<'s> {
        Token::new(kind, self.to_single_chunk(), literal)
    }

    #[inline]
    pub fn preserved(&self) -> &str {
        &self.source.body[0..self.offset.pos - 1]
    }
    #[inline]
    pub fn remains(&self) -> &str {
        &self.source.body[self.offset.pos..self.source.len()]
    }

    #[inline]
    pub fn to_error_with_load<D: Display>(&self, message: D) -> Error {
        SourceChunk::from(self).to_error(ErrOpt::new(
            Some(ErrKind::Cursor),
            Some(message.to_string()),
            None,
        ))
    }
}

impl<'s> ErrorConverter for SourceCursor<'s> {
    #[inline]
    fn to_error(&self, opt: ErrOpt) -> Error {
        self.to_single_chunk().to_error(opt)
    }
}

#[derive(Debug, Clone)]
pub struct TokenCursor<'s> {
    it: TokenIntoIter<'s>,
    current: Token<'s>,
    eof: Token<'s>,
}

impl<'s> Cursor for TokenCursor<'s> {
    type Item = Token<'s>;
    type Iter = TokenIntoIter<'s>;

    #[inline]
    fn EOF(&self) -> Self::Item {
        self.eof.clone()
    }

    #[inline]
    fn clone_items(&self) -> Self::Iter {
        self.it.clone()
    }

    #[inline]
    fn current(&mut self) -> &mut Self::Item {
        &mut self.current
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }

    #[inline]
    fn flush(&mut self) {}
}

impl<'s> TokenCursor<'s> {
    #[inline]
    pub fn new<T: Into<Tokens<'s>>>(tokens: T) -> Result<Self> {
        let tokens = tokens.into();
        if tokens.is_empty() {
            reterr!("empty token list")
        } else {
            let eof = tokens[0].clone().into_eof();
            Ok(Self {
                it: tokens.into_iter(),
                current: eof.clone(),
                eof,
            })
        }
    }
}
