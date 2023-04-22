//! # A library for handling currencies
//!
//! When working with currencies, decimals only need to be precise up to the smallest cent value while avoiding
//! common floating point errors when performing basic arithmetic. currency_rs resolves this issue by working with
//! `vodo` behind the scenes, so you don't have to be concerned about decimal precision.
//!
//! ## Quick startup
//!
//! ```
//! use currency_rs::{Currency, CurrencyOpts};
//!
//!
//! let item_price = 1.33;
//! let qty = 3.;
//! let flat_discount = 10.;
//! let cur_opts = Some(
//!     CurrencyOpts::new()
//!         .set_pattern("$ #")
//!         .set_precision(2)
//!         .set_increment(0.001),
//! );
//! let mut cur = Currency::new_float(item_price, cur_opts);
//! cur = cur.multiply(qty);
//!
//! if cur.value() > 30. {
//!     cur = cur.subtract(flat_discount);
//! }
//!
//! let final_total = cur.format();
//!
//! println!("Final Total: {}", final_total);
//!
//! ```

mod currency;
mod currency_err;
mod currency_opts;
mod currency_regex;

pub use currency::Currency;
pub use currency_err::CurrencyErr;
pub use currency_opts::CurrencyOpts;
pub(crate) use currency_regex::*;

#[cfg(test)]
mod currency_test;
