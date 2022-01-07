// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[cfg(test)]
mod tests;

mod types;
pub use types::Lexer;

#[inline]
pub fn run(source: &Source) -> Result<Tokens> {
    let mut tokens = Tokens::new();
    let mut cursor = StrCursor::new_with_extras::<LexerExtras>(source.body);

    while let Some(c) = cursor.next() {
        match c {
            ' ' | '\r' | '\t' | '\n' => continue,
            '+' => {
                cursor.save();
                tokens.push(TokenKind::Plus, SourceChunk::from(&cursor), Object::None)
            }
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

    loop {
        match cursor.bump_without_flush() {
            // without flush -> manually flush
            c if WHITESPACE_CHARS.contains(&c) => {
                cursor.flush();
                continue;
            }

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

            ch!(BANG) => {
                if cursor.first() == ch!(EQUAL) {
                    push_double_token!(@flush ts, cursor, BangEqual)
                } else {
                    push_single_token!(@flush ts, cursor, Bang)
                }
            }
            ch!(EQUAL) => {
                if cursor.first() == ch!(EQUAL) {
                    push_double_token!(@flush ts, cursor, EqualEqual)
                } else {
                    push_single_token!(@flush ts, cursor, Equal)
                }
            }
            ch!(LESS) => {
                if cursor.first() == ch!(EQUAL) {
                    push_double_token!(@flush ts, cursor, LessEqual)
                } else {
                    push_single_token!(@flush ts, cursor, Less)
                }
            }
            ch!(GREATER) => {
                if cursor.first() == ch!(EQUAL) {
                    push_double_token!(@flush ts, cursor, GreaterEqual)
                } else {
                    push_single_token!(@flush ts, cursor, Greater)
                }
            }
            ch!(SLASH) => {
                cursor.flush();
                match cursor.first() {
                    // line comment
                    ch!(SLASH) => {
                        while !matches!(cursor.bump(), nl!() | EOF_CHAR) {} // skip comments until it meets a new line or EOF
                    }
                    // block comment -> /* ... */
                    ch!(STAR) => {
                        cursor.bump();
                        let mut nest_count = 1;
                        loop {
                            match (cursor.bump(), cursor.first()) {
                                // -> /*
                                (ch!(SLASH), ch!(STAR)) => {
                                    cursor.bump();
                                    nest_count += 1;
                                }

                                // -> */
                                (ch!(STAR), ch!(SLASH)) => {
                                    cursor.bump();
                                    nest_count -= 1;
                                    if nest_count <= 0 {
                                        break;
                                    }
                                }

                                (EOF_CHAR, _) => break,
                                _ => {}
                            }
                        }
                    }
                    _ => push_single_token!(ts, cursor, Slash),
                }
            }

            // string literals
            ch!(DOUBLE_QUOTE) => {
                cursor.save_offset();
                cursor.flush();
                loop {
                    match cursor.bump() {
                        EOF_CHAR => reterr!(cursor.to_error_with_load("Unterminated string.")),
                        ch!(DOUBLE_QUOTE) => break,
                        _ => continue,
                    }
                }

                let s = cursor.load_str();
                let literal = &s[1..s.len() - 1];

                ts.push(TokenKind::String, &cursor, Some(literal));
            }

            // number literals
            chs!(0 - 9) => {
                cursor.save_offset();
                cursor.flush();

                while cursor.first().is_digit(10) {
                    cursor.bump();
                }

                if cursor.first() == ch!(DOT) {
                    cursor.bump();
                    while cursor.first().is_digit(10) {
                        cursor.bump();
                    }
                }

                ts.push(TokenKind::Number, &cursor, Some(cursor.load_str()));
            }

            // identifiers or keywords
            chs!(a - z, A - Z) | ch!(UNDERSCORE) => {
                cursor.save_offset();
                cursor.flush();

                while matches!(cursor.first(), chs!(a - z, A - Z, 0 - 9) | ch!(UNDERSCORE)) {
                    cursor.bump();
                }

                let s = cursor.load_str();

                if let Some(keyword) = __parse_keyword(s) {
                    ts.push(keyword, &cursor, None);
                } else {
                    ts.push(TokenKind::Identifier, &cursor, Some(s));
                }
            }

            EOF_CHAR => {
                push_single_token!(@eof ts, cursor);
                break;
            }
            _ => {
                cursor.flush();
                to_error!(cursor, message = "Unexpected character.",);
            }
        }
    }
    Ok(ts)
}
