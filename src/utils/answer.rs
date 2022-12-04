use std::fmt::{Display, Formatter, Result};
use Answer::*;

pub enum Answer {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
}

pub type AnsPair = (Answer, Answer);

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            Str(x) => x.fmt(f),
        }
    }
}

impl From<i32> for Answer {
    fn from(sol: i32) -> Self {
        I32(sol)
    }
}

impl From<i64> for Answer {
    fn from(sol: i64) -> Self {
        I64(sol)
    }
}

impl From<i128> for Answer {
    fn from(sol: i128) -> Self {
        I128(sol)
    }
}

impl From<u32> for Answer {
    fn from(sol: u32) -> Self {
        U32(sol)
    }
}

impl From<u64> for Answer {
    fn from(sol: u64) -> Self {
        U64(sol)
    }
}

impl From<u128> for Answer {
    fn from(sol: u128) -> Self {
        U128(sol)
    }
}

impl From<String> for Answer {
    fn from(sol: String) -> Self {
        Str(sol)
    }
}

impl From<&str> for Answer {
    fn from(sol: &str) -> Self {
        Str(sol.to_owned())
    }
}
