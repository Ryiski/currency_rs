use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyErr {
    ParseErr(String),
}

impl fmt::Display for CurrencyErr {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ParseErr(msg) => write!(f, "invalid currency value: {msg}"),
        }
    }
}

impl std::error::Error for CurrencyErr {}
