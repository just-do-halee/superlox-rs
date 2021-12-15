// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Default, Clone, Copy)]
pub struct Number(pub f64);

impl fmt::Debug for Number {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl Display for Number {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Number {
    type Target = f64;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for Number {
    #[inline]
    fn from(f: f64) -> Self {
        Number(f)
    }
}

impl TryInto<Number> for &str {
    type Error = Error;
    #[inline]
    fn try_into(self) -> Result<Number, Self::Error> {
        Ok(Number(
            f64::from_str(self).with_context(fnerr!("{} (parse) ", self))?,
        ))
    }
}

impl Number {
    #[inline]
    fn into_u64(self) -> u64 {
        self.0 as u64
    }
    fn into_ux64(self) -> u64 {
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
        self.into_ux64() == other.into_ux64()
    }
}
impl Eq for Number {}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.into_ux64().partial_cmp(&other.into_ux64())
    }
}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_ux64().cmp(&other.into_ux64())
    }
}
impl Hash for Number {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_ux64().hash(state);
    }
}

impl ops::Neg for Number {
    type Output = Number;
    #[inline]
    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl ops::Add for Number {
    type Output = Number;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}
impl ops::Sub for Number {
    type Output = Number;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Number(self.0 - rhs.0)
    }
}
impl ops::Mul for Number {
    type Output = Number;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}
impl ops::Div for Number {
    type Output = Number;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}
impl ops::BitAnd for Number {
    type Output = Number;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Number((self.into_u64() & rhs.into_u64()) as f64)
    }
}
impl ops::BitOr for Number {
    type Output = Number;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Number((self.into_u64() | rhs.into_u64()) as f64)
    }
}
impl ops::BitXor for Number {
    type Output = Number;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Number((self.into_u64() ^ rhs.into_u64()) as f64)
    }
}
