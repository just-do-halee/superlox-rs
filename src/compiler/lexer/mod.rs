// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[inline]
pub fn run(source: Source) -> Source {
    println!(
        "{:?}: {:?}",
        source.head.as_path(),
        source.chop(Span::from(100..110))
    );
    source
}
