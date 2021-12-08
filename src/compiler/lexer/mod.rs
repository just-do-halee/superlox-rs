// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[inline]
pub fn run(source: &Source) -> Result<Tokens> {
    let mut ts = Tokens::new();
    let mut cursor = Cursor::new(source);
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
                        EOF_CHAR => retcursorerr!(@load cursor, "Unterminated string."),
                        ch!(DOUBLE_QUOTE) => break,
                        _ => continue,
                    }
                }

                let s = cursor.load_str();
                let literal = &s[1..s.len() - 1];

                ts.push(
                    TokenKind::String,
                    &cursor,
                    TokenLiteral::String(literal.to_string()),
                );
            }

            // number literals
            chs!(0 - 9) => {
                cursor.save_offset();
                cursor.flush();

                while cursor.first().is_digit(10) {
                    cursor.bump();
                }

                if cursor.current_char() == ch!(DOT) {
                    while cursor.first().is_digit(10) {
                        cursor.bump();
                    }
                }

                ts.push(
                    TokenKind::Number,
                    &cursor,
                    TokenLiteral::Number(Number::from_str(cursor.load_str())?),
                );
            }

            // identifiers or keywords
            chs!(a - z, A - Z) | ch!(UNDERSCORE) => {
                cursor.save_offset();
                cursor.flush();

                while matches!(cursor.first(), chs!(a - z, A - Z, 0 - 9) | ch!(UNDERSCORE)) {
                    cursor.bump();
                }

                let s = cursor.load_str();

                if let Some(keyword) = parse_keyword(s) {
                    ts.push(keyword, &cursor, TokenLiteral::None);
                } else {
                    ts.push(
                        TokenKind::Identifier,
                        &cursor,
                        TokenLiteral::Identifier(s.to_string()),
                    );
                }
            }

            EOF_CHAR => {
                push_single_token!(@eof ts, cursor);
                break;
            }
            _ => {
                cursor.flush();
                retcursorerr!(cursor, "Unexpected character.");
            }
        }
    }
    Ok(ts)
}
