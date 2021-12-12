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
macro_rules! push_single_token {
    (@eof $token:expr, $cursor:expr) => {{
        let mut eof = $cursor.to_single_token(TokenKind::Eof, TokenLiteral::None);
        eof.lexeme.clear();
        $token.push_token(eof);
    }};
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
        let token = $cursor.to_single_token(TokenKind::$kind, TokenLiteral::$literal);
        $token.push_token(token);
    }};
}

#[macro_export]
macro_rules! push_double_token {
    (@flush $cursor:expr) => {{
        $cursor.save_offset();
        $cursor.flush();
    }};
    (
        @flush $token:expr, $cursor:expr, $kind:tt
    ) => {{
        push_double_token!(@flush $cursor);
        push_double_token!($token, $cursor, $kind, None)
    }};
    (
        @flush $token:expr, $cursor:expr, $kind:tt, $literal:tt
    ) => {{
        push_double_token!(@flush $cursor);
        push_double_token!($token, $cursor, $kind TokenLiteral::$literal);
    }};
    (
        $token:expr, $cursor:expr, $kind:tt
    ) => {
        push_double_token!($token, $cursor, $kind, None)
    };
    (
        $token:expr, $cursor:expr, $kind:tt, $literal:tt
    ) => {{
        $cursor.bump();
        $token.push(TokenKind::$kind, &$cursor, TokenLiteral::$literal);
    }};
}

#[macro_export]
macro_rules! impl_ {
    (
        Chopable$(<$lt:lifetime>)? for $name:ty
    ) => {
        impl$(<$lt>)? Chopable$(<$lt>)? for $name {
            type Out = SourceChunk$(<$lt>)?;
            /// `out of bounds == None`
            #[inline]
            fn chop<A: Into<Span>>(&$($lt)?self, span: A) -> Option<Self::Out> {
                SourceChunk::new(self.as_ref(), span)
            }
        }
    };
}
