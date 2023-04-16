#[derive(Debug, Clone)]
pub struct CurrencyOpts {
    symbol: String,
    separator: String,
    decimal: String,
    precision: f64,
    pattern: String,
    negative_pattern: String,
    from_cents: bool,
    increment: Option<f64>,
    use_vedic: bool,
    error_on_invalid: bool,
}

impl Default for CurrencyOpts {
    fn default() -> Self {
        Self {
            symbol: "$".to_string(),
            separator: ",".to_string(),
            decimal: ".".to_string(),
            precision: 2.,
            pattern: "!#".to_string(),
            negative_pattern: "-!#".to_string(),
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
impl CurrencyOpts {
    pub fn set_symbol(
        mut self,
        symbol: impl Into<String>,
    ) -> Self {
        self.symbol = symbol.into();
        self
    }

    pub fn set_separator(
        mut self,
        separator: impl Into<String>,
    ) -> Self {
        self.separator = separator.into();
        self
    }

    pub fn set_decimal(
        mut self,
        decimal: impl Into<String>,
    ) -> Self {
        self.decimal = decimal.into();
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
        pattern: impl Into<String>,
    ) -> Self {
        self.pattern = pattern.into();
        self
    }

    pub fn set_negative_pattern(
        mut self,
        negative_pattern: impl Into<String>,
    ) -> Self {
        self.negative_pattern = negative_pattern.into();
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
impl CurrencyOpts {
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn separator(&self) -> String {
        self.separator.clone()
    }

    pub fn decimal(&self) -> String {
        self.decimal.clone()
    }

    pub fn precision(&self) -> f64 {
        self.precision
    }

    pub fn pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn negative_pattern(&self) -> String {
        self.negative_pattern.clone()
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
