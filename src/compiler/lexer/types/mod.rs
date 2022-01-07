use super::*;

pub struct Lexer<'s> {
    pub source: &'s Source,
    pub cursor: StrCursor<'s, LexerExtras>,
}

impl<'s> Lexer<'s> {
    #[inline]
    pub fn new<T: Into<Source>>(source: T) -> Self {
        Lexer {
            source,
            cursor: StrCursor::new_with_extras(source.into().body),
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    fn next(&mut self) -> Option<Self::Item> {
        let c = self
            .cursor
            .next_to_while(|c| matches!(c, chs!(WHITE - SPACE)))?;

        match c {
            ch!(PLUS) => tokens.push(TokenKind::Plus, &cursor, Object::None),
            ch!(LEFT_PAREN) => push_single_token!(@flush ts, cursor, LeftParen),
            ch!(RIGHT_PAREN) => push_single_token!(@flush ts, cursor, RightParen),
            ch!(LEFT_BRACE) => push_single_token!(@flush ts, cursor, LeftBrace),
            ch!(RIGHT_BRACE) => push_single_token!(@flush ts, cursor, RightBrace),
            ch!(COMMA) => push_single_token!(@flush ts, cursor, Comma),
            ch!(DOT) => push_single_token!(@flush ts, cursor, Dot),
            ch!(MINUS) => push_single_token!(@flush ts, cursor, Minus),
            ch!(PLUS) => push_single_token!(@flush ts, cursor, Plus),
            ch!(SEMICOLON) => push_single_token!(@flush ts, cursor, Semicolon),
            ch!(STAR) => push_single_token!(@flush ts, cursor, Star),
            ch!(AMPERSAND) => push_single_token!(@flush ts, cursor, Ampersand),
            ch!(VERTICAL_BAR) => push_single_token!(@flush ts, cursor, VerticalBar),
            ch!(CIRCUMFLEX) => push_single_token!(@flush ts, cursor, Circumflex),

            _ => {
                to_error!(cursor, message = "Unexpected character.",);
            }
        }
    }
}
