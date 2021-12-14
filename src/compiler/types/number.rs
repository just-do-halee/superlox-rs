// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Default, Clone, Copy)]
pub struct Number(pub f64);

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Number {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Number(f)
    }
}

impl TryInto<Number> for &str {
    type Error = Error;
    fn try_into(self) -> Result<Number, Self::Error> {
        Ok(Number(
            f64::from_str(self).with_context(fnerr!("{} (parse) ", self))?,
        ))
    }
}

impl Number {
    fn into_u64(self) -> u64 {
        let u: u64 = self.0.to_bits();
        let bit = 1 << (64 - 1);
        if u & bit == 0 {
            u | bit
        } else {
            !u
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.into_u64() == other.into_u64()
    }
}
impl Eq for Number {}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.into_u64().partial_cmp(&other.into_u64())
    }
}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_u64().cmp(&other.into_u64())
    }
}
impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_u64().hash(state);
    }
}
