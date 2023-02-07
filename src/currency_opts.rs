#[derive(Debug, Clone)]
pub struct CurrencyOpts<'a> {
    symbol: &'a str,
    separator: &'a str,
    decimal: &'a str,
    precision: f64,
    pattern: &'a str,
    negative_pattern: &'a str,
    from_cents: bool,
    increment: Option<f64>,
    use_vedic: bool,
    error_on_invalid: bool,
}

impl Default for CurrencyOpts<'_> {
    fn default() -> Self {
        Self {
            symbol: "$",
            separator: ",",
            decimal: ".",
            precision: 2.,
            pattern: "!#",
            negative_pattern: "-!#",
            from_cents: false,
            increment: None,
            use_vedic: false,
            error_on_invalid: false,
        }
    }
}

impl CurrencyOpts<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    /// `pow` takes a `f64` and returns a `f64`
    ///
    /// Arguments:
    ///
    /// * `p`: The power to raise 10 to.
    ///
    /// Returns:
    ///
    /// The value of 10 to the power of p.
    fn pow(p: f64) -> f64 {
        10_f64.powf(p)
    }
}

// SETTERS
impl<'a> CurrencyOpts<'a> {
    pub fn set_symbol(
        mut self,
        symbol: &'a str,
    ) -> Self {
        self.symbol = symbol;
        self
    }

    pub fn set_separator(
        mut self,
        separator: &'a str,
    ) -> Self {
        self.separator = separator;
        self
    }

    pub fn set_decimal(
        mut self,
        decimal: &'a str,
    ) -> Self {
        self.decimal = decimal;
        self
    }

    pub fn set_precision(
        mut self,
        precision: i64,
    ) -> Self {
        self.precision = precision as f64;
        self
    }

    pub fn set_pattern(
        mut self,
        pattern: &'a str,
    ) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn set_negative_pattern(
        mut self,
        negative_pattern: &'a str,
    ) -> Self {
        self.negative_pattern = negative_pattern;
        self
    }

    pub fn set_from_cents(
        mut self,
        from_cents: bool,
    ) -> Self {
        self.from_cents = from_cents;
        self
    }

    pub fn set_increment(
        mut self,
        increment: f64,
    ) -> Self {
        self.increment = Some(increment);
        self
    }

    pub fn set_unset_increment(mut self) -> Self {
        self.increment = None;
        self
    }

    pub fn set_use_vedic(
        mut self,
        use_vedic: bool,
    ) -> Self {
        self.use_vedic = use_vedic;
        self
    }

    pub fn set_error_on_invalid(
        mut self,
        error_on_invalid: bool,
    ) -> Self {
        self.error_on_invalid = error_on_invalid;
        self
    }
}

// GETTERS
impl CurrencyOpts<'_> {
    pub fn symbol(&self) -> &str {
        self.symbol
    }

    pub fn separator(&self) -> &str {
        self.separator
    }

    pub fn decimal(&self) -> &str {
        self.decimal
    }

    pub fn precision(&self) -> f64 {
        self.precision
    }

    pub fn pattern(&self) -> &str {
        self.pattern
    }

    pub fn negative_pattern(&self) -> &str {
        self.negative_pattern
    }

    pub fn from_cents(&self) -> bool {
        self.from_cents
    }

    pub fn increment(&self) -> f64 {
        if let Some(inc) = self.increment {
            inc
        } else {
            1. / Self::pow(self.precision)
        }
    }

    pub fn use_vedic(&self) -> bool {
        self.use_vedic
    }

    pub fn error_on_invalid(&self) -> bool {
        self.error_on_invalid
    }
}
