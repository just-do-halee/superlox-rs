// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub const EOF_CHAR: char = '\0';
pub const WHITESPACE_CHARS: &[char] = &[' ', '\r', '\t', '\n'];

//---------------

/// * this must be a source file.
#[derive(PartialEq, Eq, Clone)]
pub enum SourceHeader {
    Header { path: PathBuf },
    IO,
}

impl Default for SourceHeader {
    #[inline]
    fn default() -> Self {
        SourceHeader::IO
    }
}

impl Display for SourceHeader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceHeader::Header { path } => write!(f, "{:?}", path),
            SourceHeader::IO => write!(f, name!(IO)),
        }
    }
}

impl fmt::Debug for SourceHeader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl SourceHeader {
    #[allow(dead_code)]
    #[inline]
    pub fn as_file_name(&self) -> Option<&OsStr> {
        match self {
            SourceHeader::Header { path } => path.file_name(),
            SourceHeader::IO => None,
        }
    }
    #[allow(dead_code)]
    #[inline]
    pub fn as_path(&self) -> &Path {
        match self {
            SourceHeader::Header { path, .. } => path,
            SourceHeader::IO => Path::new(name!(IO)),
        }
    }
    /// if given path isn't a file then returns an error.
    #[inline]
    pub fn new(path: PathBuf) -> Result<Self> {
        if path.file_name().is_none() {
            reterr!("{:?} is not a file path", path)
        }
        Ok(SourceHeader::Header { path })
    }
}

//---------------

derive_debug_partials! {
    /// * this must be a source file.
    #[derive(Default, Clone)]
    pub struct Source {
        pub head: SourceHeader,
        pub body: String,
    }
}

impl_!(Chopable<'s> for Source);

impl AsRef<Source> for Source {
    #[inline]
    fn as_ref(&self) -> &Source {
        self
    }
}

impl From<&str> for Source {
    #[inline]
    fn from(s: &str) -> Self {
        Source {
            head: SourceHeader::IO,
            body: s.to_string(),
        }
    }
}

impl Source {
    #[inline]
    pub fn new(body: &str) -> Self {
        Source::from(body)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.body.len()
    }
    #[inline]
    pub fn chars(&self) -> Chars {
        self.body.chars()
    }
}
impl TryFrom<PathBuf> for Source {
    type Error = Error;

    #[inline]
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Source {
            body: fs::read_to_string(&value)
                .with_context(fnerr!("couldn't read: {}", value.display()))?,
            head: SourceHeader::new(value),
        })
    }
}

derive_debug_partials! {

    #[derive(PartialOrd, Ord, Clone, Copy, new)]
    pub struct Offset {
        pub pos: usize,
        pub line: usize,
        pub column: usize,
    }

    #[derive(Default, Clone, Copy, new)]
    pub struct Span {
        pub start: Offset,
        pub end: Offset,
    }

}

impl Default for Offset {
    #[inline]
    fn default() -> Self {
        Offset {
            pos: 0,
            line: 1,
            column: 0,
        }
    }
}

impl Span {
    #[inline]
    pub fn set_biased_end(&mut self) {
        self.start = self.end;
    }
    #[inline]
    pub fn set_biased_start(&mut self) {
        self.end = self.start;
    }
    #[inline]
    pub fn inner_check(&self, dist: Span) -> Result<()> {
        if self.start <= dist.start && dist.end <= self.end {
            Ok(())
        } else {
            reterr!("out of bounds")
        }
    }
}

impl Display for Span {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "* start: {}, * end: {}", self.start, self.end)
    }
}

impl From<Span> for Range<usize> {
    #[inline]
    fn from(span: Span) -> Self {
        let Span { start, end } = span;
        start.pos..end.pos
    }
}

impl From<Range<usize>> for Span {
    #[inline]
    fn from(range: Range<usize>) -> Self {
        Span {
            start: Offset {
                pos: range.start,
                ..Default::default()
            },
            end: Offset {
                pos: range.end,
                ..Default::default()
            },
        }
    }
}

impl From<Range<Offset>> for Span {
    #[inline]
    fn from(range: Range<Offset>) -> Self {
        let Range { start, end } = range;
        Span { start, end }
    }
}

impl From<Range<LexerExtras>> for Span {
    fn from(v: Range<LexerExtras>) -> Self {
        Span {
            start: v.start.offset,
            end: v.end.offset,
        }
    }
}

impl Display for Offset {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[P{}/L{}/C{}]", self.pos, self.line, self.column)
    }
}

//---------------

#[derive(PartialEq, Eq, Clone)]
pub struct SourceChunk<'s> {
    pub source: &'s Source, // whole mass
    span: Span,
}

impl_!(Chopable<'s> for SourceChunk<'s>);

impl<'s> Display for SourceChunk<'s> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body())
    }
}
impl<'s> fmt::Debug for SourceChunk<'s> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.body())
    }
}

impl<'s> ErrorConverter for SourceChunk<'s> {
    #[inline]
    fn to_error(&self, opt: ErrOpt) -> Error {
        let ErrOpt { kind, message, red } = opt;
        let Span { start, end } = self.span;

        let (chunk, column) = if let Some(dist) = red {
            self.span.inner_check(dist).unwrap();
            let left = start.pos..dist.start.pos;
            let mid = dist;
            let right = dist.end.pos..end.pos;
            let column = dist.start.column;
            (
                format!(
                    "{} {} {}",
                    self._body(left),
                    style(self._body(mid)).bold().red(),
                    self._body(right)
                ),
                column,
            )
        } else {
            (self.to_string(), end.column)
        };

        makeerr_with_kind!(
            kind.unwrap_or_default(),
            "{n2}\t[{ln}:{col}] {path}{n3}\t{chunk}{n3}\t* {message}{n2}",
            ln = if start.line != end.line {
                format!("{}-{}", start.line, end.line)
            } else {
                format!("{}", start.line)
            },
            col = column,
            path = self.source.head,
            chunk = chunk,
            message = message.unwrap_or_default(),
            n2 = nl!(2),
            n3 = nl!(3),
        )
    }
}

impl<'s> SourceChunk<'s> {
    /// `out of bounds == None`
    #[inline]
    pub fn new<A: Into<Span>>(source: &'s Source, span: A) -> Option<Self> {
        let span = span.into();
        if source.body.get::<Range<usize>>(span.into()).is_some() {
            Some(SourceChunk { source, span })
        } else {
            None
        }
    }
    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }
    #[inline]
    pub fn clear(&mut self) {
        self.span.set_biased_end();
    }
    #[inline]
    pub fn body(&self) -> &str {
        let Span { start, end } = self.span;
        &self.source.body[start.pos..end.pos]
    }
    #[inline]
    pub fn _body<R: Into<Range<usize>>>(&self, range: R) -> &str {
        &self.source.body[range.into()]
    }
}

impl<'s> AsRef<Source> for SourceChunk<'s> {
    #[inline]
    fn as_ref(&self) -> &Source {
        self.source
    }
}

impl<'s> From<&'s Source> for SourceChunk<'s> {
    #[inline]
    fn from(source: &'s Source) -> Self {
        SourceChunk {
            source,
            span: Default::default(),
        }
    }
}

impl<'s> From<&SourceCursor<'s>> for SourceChunk<'s> {
    #[inline]
    fn from(cursor: &SourceCursor<'s>) -> Self {
        SourceChunk {
            source: cursor.source,
            span: cursor.load_span(),
        }
    }
}

impl<'s> From<&mut SourceCursor<'s>> for SourceChunk<'s> {
    #[inline]
    fn from(cursor: &mut SourceCursor<'s>) -> Self {
        SourceChunk {
            source: cursor.source,
            span: cursor.load_span(),
        }
    }
}

impl<'s> From<SourceCursor<'s>> for SourceChunk<'s> {
    #[inline]
    fn from(cursor: SourceCursor<'s>) -> Self {
        SourceChunk {
            source: cursor.source,
            span: cursor.load_span(),
        }
    }
}

impl<'s> From<&Lexer<'s>> for SourceChunk<'s> {
    fn from(lexer: &Lexer<'s>) -> Self {
        let Lexer { source, cursor } = lexer;
        SourceChunk {
            source,
            span: Span::from(cursor.to_range_extras()),
        }
    }
}

impl<'s> From<Range<Token<'s>>> for SourceChunk<'s> {
    #[inline]
    fn from(v: Range<Token<'s>>) -> Self {
        let Range { start, end } = v;
        SourceChunk {
            source: start.lexeme.source,
            span: Span::from(start.lexeme.span().start..end.lexeme.span().end),
        }
    }
}
