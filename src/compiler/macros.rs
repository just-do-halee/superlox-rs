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
    (AMPERSAND) => {
        '&'
    };
    (VERTICAL_BAR) => {
        '|'
    };
    (CIRCUMFLEX) => {
        '^'
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
    (@ WHITE-SPACE) => {
        ' ' | '\r' | '\t' | '\n'
    };
}

#[macro_export]
macro_rules! push_single_token {
    (@eof $token:expr, $cursor:expr) => {{
        let mut eof = $cursor.to_single_token(TokenKind::Eof, None);
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
        let token = $cursor.to_single_token(TokenKind::$kind, $literal);
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
        push_double_token!($token, $cursor, $kind $literal);
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
        $token.push(TokenKind::$kind, &$cursor, $literal);
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

#[macro_export]
macro_rules! to_error {
    (@SomeOrNone $e:expr) => {
        Some($e)
    };
    (@SomeOrNone) => {
        None
    };
    ($target:expr, $(kind = $kind:expr,)? $(message = $msg:expr,)? $(red = $red:expr,)?) => {
        $target.to_error(ErrOpt::new(
                            to_error!(@SomeOrNone $($kind)?),
                            to_error!(@SomeOrNone $($msg.to_string())?),
                            to_error!(@SomeOrNone $($red)?),
                        ))
    };
}

#[macro_export]
macro_rules! ret_to_error {
    ($target:expr, $(kind = $kind:expr,)? $(message = $msg:expr,)? $(red = $red:expr,)?) => {
        reterr!(to_error!($target, $(kind = $kind,)? $(message = $msg,)? $(red = $red,)?))
    };
}
