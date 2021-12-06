// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Clone)]
pub struct Cursor<'s> {
    pub source: &'s Source,
    chars: Chars<'s>,
    offset: Offset,
    saved_offset: Offset,
    current_char: char,
}

impl<'s> Display for Cursor<'s> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.offset, self.current_char)
    }
}
impl<'s> fmt::Debug for Cursor<'s> {
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

impl<'s> Cursor<'s> {
    #[inline]
    pub fn new<S: Into<SourceChunk<'s>>>(chunk: S) -> Self {
        let chunk = chunk.into();
        Cursor {
            source: chunk.source,
            chars: chunk.source.chars(),
            offset: chunk.span.start,
            saved_offset: chunk.span.start,
            current_char: EOF_CHAR,
        }
    }

    #[inline]
    pub fn to_source_chunk(&self) -> SourceChunk<'s> {
        self.into()
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
        Span::from(self.saved_offset..self.offset)
    }

    /// base
    /// moves to the next character
    #[inline]
    pub fn bump(&mut self) -> char {
        let c = self.chars.next().unwrap_or(EOF_CHAR);

        self.current_char = c;

        if c != EOF_CHAR {
            self.offset.pos += 1;

            // new line
            if c == '\n' {
                self.offset.line += 1;
                self.offset.column = 0; // reset
            } else {
                self.offset.column += 1;
            }
        }

        c
    }

    /// returns a `Chars` iterator over the remaining characters
    #[inline]
    fn chars(&self) -> Chars<'s> {
        self.chars.clone()
    }

    /// returns nth character relative to the current cursor position
    /// if requested position doesn't exist, `EOF_CHAR` is returned
    /// however, getting `EOF_CHAR` doesn't always mean actual end of file
    /// it should be checked with `is_eof` method
    #[inline]
    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF_CHAR)
    }

    /// peeks the next symbol from the input stream without consuming it
    #[inline]
    pub fn first(&self) -> char {
        self.nth_char(0)
    }

    /// peeks the second symbol from the input stream without consuming it
    #[inline]
    pub fn second(&self) -> char {
        self.nth_char(1)
    }

    #[inline]
    pub fn preserved(&self) -> &str {
        &self.source.body[0..self.offset.pos - 1]
    }
    #[inline]
    pub fn remains(&self) -> &str {
        &self.source.body[self.offset.pos..self.source.len()]
    }
}
