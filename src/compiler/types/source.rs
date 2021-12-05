// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub const EOF_CHAR: char = '\0';

//---------------

/// * this must be a source file.
#[derive(PartialEq, Eq, Clone)]
pub enum SourceHeader {
    Header { file_name: OsString, path: PathBuf },
    IO,
}

impl Default for SourceHeader {
    #[inline]
    fn default() -> Self {
        SourceHeader::IO
    }
}

impl fmt::Debug for SourceHeader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SourceHeader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceHeader::Header { path, .. } => write!(f, "{:?}", path),
            SourceHeader::IO => write!(f, name!(IO)),
        }
    }
}

impl SourceHeader {
    #[inline]
    pub fn as_file_name(&self) -> &OsStr {
        match self {
            SourceHeader::Header { file_name, .. } => file_name,
            SourceHeader::IO => OsStr::new(name!(IO)),
        }
    }
    #[inline]
    pub fn as_path(&self) -> &Path {
        match self {
            SourceHeader::Header { path, .. } => path,
            SourceHeader::IO => Path::new(name!(IO)),
        }
    }
    /// if given path isn't a file then returns an error.
    #[inline]
    pub fn new(path: PathBuf) -> Self {
        if let Some(v) = path.file_name() {
            SourceHeader::Header {
                file_name: v.to_os_string(),
                path,
            }
        } else {
            SourceHeader::IO
        }
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

impl Source {
    #[inline]
    pub fn new(body: String) -> Self {
        Source {
            head: SourceHeader::IO,
            body,
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.body.len()
    }
    #[inline]
    pub fn chars(&self) -> Chars {
        self.body.chars()
    }
    #[inline]
    pub fn to_source_chunk(&self) -> SourceChunk {
        self.chop(0..self.len()).unwrap()
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

    #[derive(Default, Clone, Copy, new)]
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

impl Display for Offset {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[P{}/L{}/C{}]", self.pos, self.line, self.column)
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

//---------------

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SourceChunk<'s> {
    pub source: &'s Source, // whole mass
    pub span: Span,
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

impl<'s> AsRef<Source> for SourceChunk<'s> {
    #[inline]
    fn as_ref(&self) -> &Source {
        self.source
    }
}

impl<'s> ErrorConverter for SourceChunk<'s> {
    #[inline]
    fn to_error<D: Display>(&self, message: D) -> Error {
        let Span { start, .. } = self.span;
        makeerr!(
            "{n2}\t[{}:{}] {:?}{n3}\t{}{n3}\t->  {}{n2}",
            start.line,
            start.column,
            self.source.head,
            self,
            message,
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
    pub fn body(&self) -> &'s str {
        let Span { start, end } = self.span;
        &self.source.body[start.pos..end.pos]
    }
}
