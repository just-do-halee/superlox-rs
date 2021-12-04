// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[inline]
pub fn run(source: Source) -> Result<Source> {
    if let Some(e) = source
        .chop(10..20)
        .map(|s| s.into_error("this is some error.".to_string()))
    {
        // testing error
        return Err(e);
    }
    Ok(source)
}
