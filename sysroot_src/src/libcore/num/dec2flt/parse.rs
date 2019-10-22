//! Validating and decomposing a decimal string of the form:

#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct Decimal<'a> {
    pub integral: &'a [u8],
    pub fractional: &'a [u8],
    pub exp: i64,
}

impl<'a> Decimal<'a> {
    pub fn new(integral: &'a [u8], fractional: &'a [u8], exp: i64) -> Decimal<'a> { loop { } }
}

#[derive(Debug)]
pub enum ParseResult<'a> {
    Valid(Decimal<'a>),
    ShortcutToInf,
    ShortcutToZero,
    Invalid,
}

pub fn parse_decimal(s: &str) -> ParseResult<'_> { loop { } }

fn eat_digits(s: &[u8]) -> (&[u8], &[u8]) { loop { } }

fn parse_exp<'a>(integral: &'a [u8], fractional: &'a [u8], rest: &'a [u8]) -> ParseResult<'a> { loop { } }
