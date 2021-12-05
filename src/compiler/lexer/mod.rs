// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[inline]
pub fn run<'s, S: AsRef<Source>>(source: S) -> Result<Tokens<'s>> {
    let tokens = Vec::new();
    let mut cursor = Cursor::new(source.as_ref().to_source_chunk());
    eprintln!("[{}]", cursor.source.head); // for the tests, the bottleneck terminal
    loop {
        match cursor.bump() {
            EOF_CHAR => break,
            c if c == 'a' => eprintln!("{}", cursor),
            _ => {}
        }
    }
    Ok(tokens)
}
