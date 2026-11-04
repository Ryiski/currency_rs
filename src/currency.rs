use crate::{format_dollar, CurrencyOpts};

/// An amount stored as an integer number of minor units (e.g. cents).
///
/// ```
/// use currency_rs::Currency;
///
/// let c = Currency::from(2.51) + 0.01;
/// assert_eq!(c, 2.52);
/// assert_eq!(c.int_value(), 252);
/// assert_eq!(c.format(), "$2.52");
/// ```
#[derive(Debug, Clone, Default)]
pub struct Currency {
    pub(crate) value: f64,
    pub(crate) int_value: f64,
    pub(crate) opts: CurrencyOpts,
}

impl Currency {
    pub(crate) fn with_int(
        int_value: f64,
        opts: CurrencyOpts,
    ) -> Self {
        Self {
            value: int_value / opts.scale(),
            int_value,
            opts,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    /// The amount in minor units, e.g. `123` for `1.23`.
    pub fn int_value(&self) -> i64 {
        self.int_value as i64
    }

    pub fn opts(&self) -> &CurrencyOpts {
        &self.opts
    }

    /// Whole units, truncated toward zero: `-1.77 → -1`.
    pub fn dollars(&self) -> i64 {
        self.value.trunc() as i64
    }

    /// Minor units after the whole part, keeping the sign: `-1.23 → -23`.
    pub fn cents(&self) -> i64 {
        (self.int_value % self.opts.scale()) as i64
    }

    pub fn format(&self) -> String {
        self.render(&self.opts)
    }

    /// Format with one-off option overrides.
    ///
    /// ```
    /// use currency_rs::Currency;
    ///
    /// assert_eq!(Currency::from(1.23).format_with(|o| o.symbol("£")), "£1.23");
    /// ```
    pub fn format_with(
        &self,
        f: impl FnOnce(CurrencyOpts) -> CurrencyOpts,
    ) -> String {
        self.render(&f(self.opts.clone()))
    }

    /// Split into `count` parts that sum exactly to the original,
    /// spreading leftover minor units over the first parts.
    ///
    /// ```
    /// use currency_rs::Currency;
    ///
    /// let parts = Currency::from(1.01).distribute(4);
    /// assert_eq!(parts, [0.26, 0.25, 0.25, 0.25]);
    /// ```
    pub fn distribute(
        &self,
        count: usize,
    ) -> Vec<Self> {
        if count == 0 {
            return vec![];
        }

        let n = count as f64;
        let int = self.int_value;
        let (split, penny) = if int >= 0. {
            ((int / n).floor(), 1.)
        } else {
            ((int / n).ceil(), -1.)
        };
        let pennies = (int - split * n).abs() as usize;

        // Two steps like currency.js, which matters for float noise near 2^53.
        (0..count)
            .map(|i| {
                let mut item = self.rewrap(split);
                if i < pennies {
                    item.set_input(item.input_of(item.int_value + penny));
                }
                item
            })
            .collect()
    }

    /// Re-wrap an int result, as currency.js does after every operation.
    pub(crate) fn rewrap(
        &self,
        int: f64,
    ) -> Self {
        self.opts.from(self.input_of(int))
    }

    pub(crate) fn input_of(
        &self,
        int: f64,
    ) -> f64 {
        int / self.opts.input_scale()
    }

    /// In-place `rewrap`, avoids cloning the options.
    pub(crate) fn set_input(
        &mut self,
        input: f64,
    ) {
        self.int_value = self.opts.parse_num(input, true);
        self.value = self.int_value / self.opts.scale();
    }

    pub(crate) fn int_of(
        &self,
        rhs: &Self,
    ) -> f64 {
        if self.opts.from_cents {
            rhs.int_value
        } else {
            self.opts.parse_num(rhs.value, true)
        }
    }

    fn render(
        &self,
        o: &CurrencyOpts,
    ) -> String {
        let number = self.to_string();
        let number = number.strip_prefix('-').unwrap_or(&number);
        let (dollars, cents) = match number.split_once('.') {
            Some((d, c)) => (d, format!("{}{c}", o.decimal)),
            None => (number, String::new()),
        };

        let pattern = if self.value >= 0. {
            &o.pattern
        } else {
            &o.negative_pattern
        };

        pattern.replacen('!', &o.symbol, 1).replacen(
            '#',
            &(format_dollar(o.use_vedic, dollars, &o.separator) + &cents),
            1,
        )
    }
}
