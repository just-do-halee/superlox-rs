// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[cfg(test)]
mod tests;

mod types;
use types::*;

#[inline]
pub fn run(tokens: Tokens) -> Result<Object> {
    let (caught, res) = TokenParser::new(tokens)?.parse();
    if caught {
        reterr!("caught errors")
    } else {
        ExprVisitor.visit(res?)
    }
}
