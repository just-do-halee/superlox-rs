// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! 'tests'

use super::*;

#[test]
fn tokenizing() {
    let source = Source::new(
        r#"
    1 +2;
    print 2 + 1;
    print "one";
    print true;

    if () {}
    /* U(Uj#$*()@#)(@!#&%#%NM) This is Line Comment */ 

    if (1 != 2) {
        print "yes";
    }

    /* Ha This is Block Comment

    H
    a
    wefwkljf mgekrlgmekrlmekrlgmerg
    mwifmweoifmwekfm
    ;,;d bmeiorbmeirom*(&#%(*@&%*(@u @)_%()_KGKLJGM))*/
    "#,
    );

    let tokens = format!("{:#?}", run(&source).unwrap());

    assert_eq!(
        tokens,
        r#"Tokens {
    body: [
        Token {
            kind: Number,
            lexeme: "1",
            literal: Number(
                Number(
                    1.0,
                ),
            ),
        },
        Token {
            kind: Plus,
            lexeme: "+",
            literal: None,
        },
        Token {
            kind: Number,
            lexeme: "2",
            literal: Number(
                Number(
                    2.0,
                ),
            ),
        },
        Token {
            kind: Semicolon,
            lexeme: ";",
            literal: None,
        },
        Token {
            kind: Print,
            lexeme: "print",
            literal: None,
        },
        Token {
            kind: Number,
            lexeme: "2",
            literal: Number(
                Number(
                    2.0,
                ),
            ),
        },
        Token {
            kind: Plus,
            lexeme: "+",
            literal: None,
        },
        Token {
            kind: Number,
            lexeme: "1",
            literal: Number(
                Number(
                    1.0,
                ),
            ),
        },
        Token {
            kind: Semicolon,
            lexeme: ";",
            literal: None,
        },
        Token {
            kind: Print,
            lexeme: "print",
            literal: None,
        },
        Token {
            kind: String,
            lexeme: "\"one\"",
            literal: String(
                "one",
            ),
        },
        Token {
            kind: Semicolon,
            lexeme: ";",
            literal: None,
        },
        Token {
            kind: Print,
            lexeme: "print",
            literal: None,
        },
        Token {
            kind: True,
            lexeme: "true",
            literal: None,
        },
        Token {
            kind: Semicolon,
            lexeme: ";",
            literal: None,
        },
        Token {
            kind: If,
            lexeme: "if",
            literal: None,
        },
        Token {
            kind: LeftParen,
            lexeme: "(",
            literal: None,
        },
        Token {
            kind: RightParen,
            lexeme: ")",
            literal: None,
        },
        Token {
            kind: LeftBrace,
            lexeme: "{",
            literal: None,
        },
        Token {
            kind: RightBrace,
            lexeme: "}",
            literal: None,
        },
        Token {
            kind: If,
            lexeme: "if",
            literal: None,
        },
        Token {
            kind: LeftParen,
            lexeme: "(",
            literal: None,
        },
        Token {
            kind: Number,
            lexeme: "1",
            literal: Number(
                Number(
                    1.0,
                ),
            ),
        },
        Token {
            kind: BangEqual,
            lexeme: "!=",
            literal: None,
        },
        Token {
            kind: Number,
            lexeme: "2",
            literal: Number(
                Number(
                    2.0,
                ),
            ),
        },
        Token {
            kind: RightParen,
            lexeme: ")",
            literal: None,
        },
        Token {
            kind: LeftBrace,
            lexeme: "{",
            literal: None,
        },
        Token {
            kind: Print,
            lexeme: "print",
            literal: None,
        },
        Token {
            kind: String,
            lexeme: "\"yes\"",
            literal: String(
                "yes",
            ),
        },
        Token {
            kind: Semicolon,
            lexeme: ";",
            literal: None,
        },
        Token {
            kind: RightBrace,
            lexeme: "}",
            literal: None,
        },
        Token {
            kind: Eof,
            lexeme: "",
            literal: None,
        },
    ],
}"#
    )
}
