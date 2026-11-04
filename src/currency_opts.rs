use crate::{Currency, CurrencyErr};

/// Build once with chained setters, then create values with [`CurrencyOpts::from`] / [`CurrencyOpts::parse`].
///
/// ```
/// use currency_rs::CurrencyOpts;
///
/// let eur = CurrencyOpts::new().symbol("€").separator(".").decimal(",");
/// assert_eq!(eur.parse("1.234,56").unwrap().format(), "€1.234,56");
/// assert_eq!(eur.symbol, "€");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct CurrencyOpts {
    pub symbol: String,
    pub separator: String,
    pub decimal: String,
    pub precision: u32,
    /// `!` is replaced by the symbol, `#` by the amount.
    pub pattern: String,
    pub negative_pattern: String,
    /// Treat inputs as minor units (cents).
    pub from_cents: bool,
    /// Display-only rounding step, e.g. `0.05`. Stored values are not rounded to it.
    pub increment: Option<f64>,
    /// Indian numbering groups (`10,00,000`).
    pub use_vedic: bool,
    /// Return `Err` on unparsable strings instead of `0`.
    pub error_on_invalid: bool,
}

impl Default for CurrencyOpts {
    fn default() -> Self {
        Self {
            symbol: "$".into(),
            separator: ",".into(),
            decimal: ".".into(),
            precision: 2,
            pattern: "!#".into(),
            negative_pattern: "-!#".into(),
            from_cents: false,
            increment: None,
            use_vedic: false,
            error_on_invalid: false,
        }
    }
}

impl CurrencyOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(
        &self,
        value: f64,
    ) -> Currency {
        Currency::with_int(self.parse_num(value, true), self.clone())
    }

    pub fn parse(
        &self,
        value: &str,
    ) -> Result<Currency, CurrencyErr> {
        Ok(Currency::with_int(self.parse_str(value)?, self.clone()))
    }

    pub fn symbol(
        mut self,
        symbol: impl Into<String>,
    ) -> Self {
        self.symbol = symbol.into();
        self
    }

    pub fn separator(
        mut self,
        separator: impl Into<String>,
    ) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn decimal(
        mut self,
        decimal: impl Into<String>,
    ) -> Self {
        self.decimal = decimal.into();
        self
    }

    pub fn precision(
        mut self,
        precision: u32,
    ) -> Self {
        self.precision = precision;
        self
    }

    pub fn pattern(
        mut self,
        pattern: impl Into<String>,
    ) -> Self {
        self.pattern = pattern.into();
        self
    }

    pub fn negative_pattern(
        mut self,
        negative_pattern: impl Into<String>,
    ) -> Self {
        self.negative_pattern = negative_pattern.into();
        self
    }

    pub fn from_cents(
        mut self,
        from_cents: bool,
    ) -> Self {
        self.from_cents = from_cents;
        self
    }

    pub fn increment(
        mut self,
        increment: f64,
    ) -> Self {
        self.increment = Some(increment);
        self
    }

    pub fn use_vedic(
        mut self,
        use_vedic: bool,
    ) -> Self {
        self.use_vedic = use_vedic;
        self
    }

    pub fn error_on_invalid(
        mut self,
        error_on_invalid: bool,
    ) -> Self {
        self.error_on_invalid = error_on_invalid;
        self
    }
}

// Port of currency.js `parse()`.
impl CurrencyOpts {
    pub(crate) fn scale(&self) -> f64 {
        10_f64.powi(self.precision as i32)
    }

    /// With `from_cents`, inputs are already minor units, so the divisor is 1.
    pub(crate) fn input_scale(&self) -> f64 {
        if self.from_cents {
            1.
        } else {
            self.scale()
        }
    }

    pub(crate) fn increment_or_default(&self) -> f64 {
        self.increment.unwrap_or(1. / self.scale())
    }

    pub(crate) fn parse_num(
        &self,
        mut v: f64,
        use_rounding: bool,
    ) -> f64 {
        if !self.from_cents {
            v *= self.scale();
            v = crate::js_to_fixed(v, 4).parse().unwrap(); // currency.js: v.toFixed(4)
        }

        if use_rounding {
            js_round(v)
        } else {
            v
        }
    }

    pub(crate) fn parse_str(
        &self,
        value: &str,
    ) -> Result<f64, CurrencyErr> {
        let negated = crate::allow_negative_values(value);
        let numeric = crate::replace_any_non_numeric_values(&self.decimal, &negated);
        let normalized = crate::convert_any_decimal_values(&self.decimal, &numeric);

        match normalized.parse::<f64>() {
            Ok(v) => Ok(self.parse_num(v, true)),
            Err(err) if self.error_on_invalid => Err(CurrencyErr::ParseErr(err.to_string())),
            Err(_) => Ok(0.),
        }
    }
}

/// JS `Math.round`: halves round toward +∞ (`-1.5 → -1`), unlike `f64::round`.
/// Also normalizes `-0` to `0`.
pub(crate) fn js_round(v: f64) -> f64 {
    let floor = v.floor();
    if v - floor >= 0.5 {
        floor + 1.
    } else {
        floor + 0.
    }
}
