//! # A library for handling currencies
//!
//! A Rust port of [currency.js](https://github.com/scurker/currency.js). Amounts are stored as
//! integers of minor units (e.g. cents), so `2.51 + 0.01` is `2.52`, not `2.5199999999999996`.
//!
//! ```
//! use currency_rs::{Currency, CurrencyOpts};
//!
//! # fn main() -> Result<(), currency_rs::CurrencyErr> {
//! let mut total = Currency::from(1.33);
//! total *= 3.;
//! if total > 3. {
//!     total -= 1.;
//! }
//! assert_eq!(total.format(), "$2.99");
//!
//! // Custom options: build once, create values from them.
//! let eur = CurrencyOpts::new().symbol("€").separator(".").decimal(",");
//! let price = eur.parse("1.234,56")?;
//! assert_eq!((&price + &price).format(), "€2.469,12");
//!
//! // Sum, compare, split.
//! let items = [Currency::from(0.1), Currency::from(0.2)];
//! let sum: Currency = items.iter().sum();
//! assert_eq!(sum, 0.3);
//! assert_eq!(Currency::from(1.01).distribute(4), [0.26, 0.25, 0.25, 0.25]);
//! # Ok(())
//! # }
//! ```

mod currency;
mod currency_err;
mod currency_opts;
mod currency_utils;
mod traits;

pub use currency::Currency;
pub use currency_err::CurrencyErr;
pub use currency_opts::CurrencyOpts;
pub(crate) use currency_utils::*;

#[cfg(test)]
mod currency_test;
