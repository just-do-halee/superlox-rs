// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

/// * this must be source file.
#[derive(PartialEq, Eq, Clone)]
pub enum SourceHeader {
    Header { file_name: OsString, path: PathBuf },
    IO,
}

impl fmt::Debug for SourceHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SourceHeader {
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
    /// if the given path is not a file then returning error.
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

derive_debug_partials! {
    /// * this must be source file.
    #[derive(Clone)]
    pub struct Source {
        // when the head is none,
        // it's only because of targeting io-stream.
        pub head: SourceHeader,
        pub body: String,
    }

}

impl Source {
    /// * if the path is given is not suitable then returns error.
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
    /// `out of bounds == None`
    #[inline]
    pub fn chop(&self, span: Span) -> Option<SourceChunk> {
        SourceChunk::new(self, span)
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

#[derive(Clone, Copy)]
pub struct SourceChunk<'s> {
    source: &'s Source, // whole mass
    span: Span,
}

impl<'s> Display for SourceChunk<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body())
    }
}
impl<'s> fmt::Debug for SourceChunk<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.body())
    }
}

impl<'s> SourceChunk<'s> {
    /// `out of bounds == None`
    #[inline]
    pub fn new(source: &'s Source, span: Span) -> Option<Self> {
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
