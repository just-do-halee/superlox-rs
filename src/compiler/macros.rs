// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[macro_export]
macro_rules! ch {
    (EOF) => {
        '\0'
    };
    (LEFT_PAREN) => {
        '('
    };
    (RIGHT_PAREN) => {
        ')'
    };
    (LEFT_BRACE) => {
        '{'
    };
    (RIGHT_BRACE) => {
        '}'
    };
    (COMMA) => {
        ','
    };
    (DOT) => {
        '.'
    };
    (MINUS) => {
        '-'
    };
    (PLUS) => {
        '+'
    };
    (SEMICOLON) => {
        ';'
    };
    (STAR) => {
        '*'
    };
    (BANG) => {
        '!'
    };
    (EQUAL) => {
        '='
    };
    (LESS) => {
        '<'
    };
    (GREATER) => {
        '>'
    };
    (SLASH) => {
        '/'
    };
    (DOUBLE_QUOTE) => {
        '"'
    };
    (UNDERSCORE) => {
        '_'
    };
}

#[macro_export]
macro_rules! chs {
    ($($tt1:tt-$tt2:tt),*) => {
        $(chs!(@ $tt1-$tt2))|*
    };
    (@ a-z) => {
        'a'..='z'
    };
    (@ A-Z) => {
        'A'..='Z'
    };
    (@ 0-9) => {
        '0'..='9'
    };
}

#[macro_export]
macro_rules! retcursorerr {
    (@load $cursor:expr, $literal:literal) => {
        _reterr!(SourceChunk::from(&$cursor).to_error($literal))
    };
    (@load $cursor:expr, $fmt:literal, $($expr:expr),*) => {
        _reterr!(SourceChunk::from(&$cursor).to_error(format!($fmt, $($expr),*)))
    };
    ($cursor:expr, $literal:literal) => {
        _reterr!($cursor.to_single_chunk().to_error($literal))
    };
    ($cursor:expr, $fmt:literal, $($expr:expr),*) => {
        _reterr!($cursor.to_single_chunk().to_error(format!($fmt, $($expr),*)))
    };
}

#[macro_export]
macro_rules! push_single_token {
    (
        @flush $token:expr, $cursor:expr, $kind:tt
    ) => {{
        $cursor.flush();
        push_single_token!($token, $cursor, $kind)
    }};
    (
        @flush $token:expr, $cursor:expr, $kind:tt, $literal:tt
    ) => {{
        $cursor.flush();
        push_single_token!($token, $cursor, $kind, $literal)
    }};
    (
        $token:expr, $cursor:expr, $kind:tt
    ) => {
        push_single_token!($token, $cursor, $kind, None)
    };
    (
        $token:expr, $cursor:expr, $kind:tt, $literal:tt
    ) => {{
        $token.push_token($cursor.to_single_token(TokenKind::$kind, TokenLiteral::$literal));
    }};
    (@eof $token:expr, $cursor:expr) => {{
        let mut eof = $cursor.to_single_token(TokenKind::Eof, TokenLiteral::None);
        eof.lexeme.clear();
        $token.push_token(eof);
    }};
}

#[macro_export]
macro_rules! push_double_token {
    (
        $token:expr, $cursor:expr, $kind:tt
    ) => {
        push_double_token!($token, $cursor, $kind, None)
    };
    (
        $token:expr, $cursor:expr, $kind:tt, $literal:tt
    ) => {{
        $cursor.save_offset();
        $cursor.flush();
        $cursor.bump();
        $token.push(TokenKind::$kind, &$cursor, TokenLiteral::$literal);
    }};
}

#[macro_export]
macro_rules! impl_ {
    (
        Chopable<$lt:lifetime> for $name:ty
    ) => {
        impl<$lt> Chopable<$lt> for $name {
            type Out = SourceChunk<$lt>;
            /// `out of bounds == None`
            #[inline]
            fn chop<A: Into<Span>>(&$lt self, span: A) -> Option<Self::Out> {
                SourceChunk::new(self.as_ref(), span)
            }
        }
    };
}
