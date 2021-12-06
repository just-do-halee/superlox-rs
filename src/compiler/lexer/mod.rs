// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[inline]
pub fn run(source: &Source) -> Result<Tokens> {
    let mut tokens = Vec::new();
    let mut cursor = Cursor::new(source);
    eprintln!("[{}]", cursor.source.head);
    loop {
        let c = cursor.bump();
        match c {
            EOF_CHAR => {
                cursor.save_offset();
                tokens.push(Token::new(
                    TokenKind::Eof,
                    cursor.to_source_chunk(),
                    TokenLiteral::None,
                ));
                break;
            }
            'a' => eprintln!("{}", cursor),
            _ => {}
        }
    }
    Ok(tokens)
}
