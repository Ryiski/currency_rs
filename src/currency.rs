use crate::{
    allow_negative_values, convert_any_decimal_values, format_dollar,
    replace_any_non_numeric_values, CurrencyErr, CurrencyOpts,
};

#[derive(Debug, Clone)]
pub struct Currency {
    pub(crate) value: f64,
    pub(crate) int_value: f64,
    pub(crate) opts: CurrencyOpts,
}

impl Currency {
    /// It returns the value of the currency.
    ///
    /// Returns:
    ///
    /// f64 value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// It returns the int_value of the currency.
    ///
    /// Returns:
    ///
    /// f64 int_value.
    pub fn int_value(&self) -> f64 {
        self.int_value
    }

    /// It creates a new currency object from a f64.
    ///
    /// Arguments:
    ///
    /// * `value`: The value of the currency.
    /// * `opts`: CurrencyOpts
    ///
    /// Returns:
    ///
    /// A new instance of the Currency struct.
    pub fn new_float(
        value: f64,
        opts: Option<CurrencyOpts>,
    ) -> Self {
        let currency_options = match opts {
            Some(options) => options,
            None => CurrencyOpts::new(),
        };

        let v = Self::parse(value, &currency_options, true);

        Self::new(v, currency_options)
    }

    /// It creates a new currency object from a string.
    ///
    /// Arguments:
    ///
    /// * `value`: The value of the currency.
    /// * `opts`: CurrencyOpts
    ///
    /// Returns:
    ///
    /// A new instance of the Currency struct.
    pub fn new_string(
        value: &str,
        opts: Option<CurrencyOpts>,
    ) -> Result<Self, CurrencyErr> {
        let currency_options = match opts {
            Some(options) => options,
            None => CurrencyOpts::default(),
        };

        let v = Self::parse_string(value, &currency_options, true)?;

        Ok(Self::new(v, currency_options))
    }

    /// It creates a new currency object from a currency object.
    ///
    /// Arguments:
    ///
    /// * `cur`: The currency object to be parsed.
    /// * `opts`: Option<CurrencyOpts>
    ///
    /// Returns:
    ///
    /// A new instance of the Currency struct.
    pub fn new_cur(
        cur: Self,
        opts: Option<CurrencyOpts>,
    ) -> Self {
        let currency_options = match opts {
            Some(options) => options,
            None => CurrencyOpts::default(),
        };

        let v = Self::parse(cur.value, &currency_options, true);

        Self::new(v, currency_options)
    }

    /// It creates a new instance of the Currency struct.
    ///
    /// Arguments:
    ///
    /// * `value`: The value of the currency.
    /// * `opts`: CurrencyOpts
    ///
    /// Returns:
    ///
    /// A new instance of the Currency struct.
    fn new(
        v: f64,
        opts: CurrencyOpts,
    ) -> Self {
        let precision = Self::pow(opts.precision());

        let int_value = v;

        let value = int_value / precision;

        Self {
            value,
            int_value,
            opts,
        }
    }

    /// > This function takes a value, a set of options, and a boolean value, and returns a float
    ///
    /// Arguments:
    ///
    /// * `value`: The value to be parsed.
    /// * `opts`: CurrencyOpts
    /// * `use_rounding`: If true, the value will be rounded to the nearest integer.
    ///
    /// Returns:
    ///
    /// a f64 value.
    fn parse(
        value: f64,
        opts: &CurrencyOpts,
        use_rounding: bool,
    ) -> f64 {
        let mut v: f64 = value;

        let from_cents = opts.from_cents();

        let precision = Self::pow(opts.precision());

        if !from_cents {
            v *= precision; // scale number to integer value
            v = Self::round_dp(v, 4)
        }

        if use_rounding {
            Self::round(v)
        } else {
            v
        }
    }

    /// It takes a string, removes all non-numeric characters, converts it to a float, and then rounds it
    /// to the nearest integer
    ///
    /// Arguments:
    ///
    /// * `value`: The string value to be parsed.
    /// * `opts`: CurrencyOpts
    /// * `use_rounding`: If true, the value will be rounded to the nearest integer.
    fn parse_string(
        value: &str,
        opts: &CurrencyOpts,
        use_rounding: bool,
    ) -> Result<f64, CurrencyErr> {
        let decimal = opts.decimal();

        let from_cents = opts.from_cents();

        let precision = Self::pow(opts.precision());

        let value_allow_negative = allow_negative_values(value);

        let numeric_values = replace_any_non_numeric_values(decimal.clone(), value_allow_negative);

        let value = convert_any_decimal_values(decimal, numeric_values);

        match value.parse::<f64>() {
            Ok(mut parsed_val) => {
                if !from_cents {
                    parsed_val *= precision; // scale number to integer value
                    parsed_val = Self::round_dp(parsed_val, 4);
                }

                if use_rounding {
                    Ok(Self::round(parsed_val))
                } else {
                    Ok(parsed_val)
                }
            }
            Err(err) => {
                if opts.error_on_invalid() {
                    return Err(CurrencyErr::ParseErr(err.to_string()));
                }

                Ok(0.)
            }
        }
    }

    pub fn format(&self) -> String {
        let precision = self.opts.precision();

        let increment = self.opts.increment();

        let rounded_value = Self::rounding(self.value, increment);

        let pos_pattern = self.opts.pattern();

        let negative_pattern = self.opts.negative_pattern();

        let symbol = self.opts.symbol();

        let separator = self.opts.separator();

        let decimal = self.opts.decimal();

        let mut currency: String =
            Self::round_dp_to_string(rounded_value, self.opts.precision() as usize);

        currency = currency.replacen('-', "", 1);

        let split_currency: Vec<&str> = currency.split('.').collect();

        let dollars = format_dollar(
            self.opts.use_vedic(),
            split_currency.first().unwrap().to_string(),
            separator,
        );

        let mut cents = if split_currency.len() > 1 {
            split_currency.last().unwrap().to_string()
        } else {
            "0".to_string()
        };

        cents = if precision > 0. {
            decimal + &cents
        } else {
            "".to_string()
        };

        let pattern = if self.value >= 0. {
            pos_pattern
        } else {
            negative_pattern
        };

        pattern
            .replace('!', &symbol)
            .replace('#', &(dollars + &cents))
    }

    /// It returns the cents of the value.
    ///
    /// Returns:
    ///
    /// The cents of the value.
    pub fn cents(&self) -> u64 {
        (self.int_value % Self::pow(self.opts.precision())) as u64
    }

    /// It returns the value of the dollar.
    ///
    /// Returns:
    ///
    /// The value of the money object.
    pub fn dollars(&self) -> i64 {
        (if self.value > 0. {
            self.value.floor()
        } else {
            self.value.ceil()
        }) as i64
    }

    /// > It takes a number, converts it to an integer, adds it to the current integer value, and then
    /// divides it by the precision
    ///
    /// Arguments:
    ///
    /// * `number`: The number to add to the current value.
    ///
    /// Returns:
    ///
    /// A new instance of the Money struct.
    #[allow(clippy::should_implement_trait)]
    pub fn add(
        self,
        number: f64,
    ) -> Self {
        let mut int_value = self.int_value;

        let p = if self.opts.from_cents() {
            1.
        } else {
            Self::pow(self.opts.precision())
        };

        int_value += Self::parse(number, &self.opts, true);

        Self::new_float(int_value / p, Some(self.opts))
    }

    /// > It takes a number, converts it to an integer, adds it to the current integer value, and then
    /// divides it by the precision
    ///
    /// Arguments:
    ///
    /// * `number`: The number to add to the current value.
    ///
    /// Returns:
    ///
    /// A new instance of the Money struct.
    pub fn add_string(
        self,
        number: &str,
    ) -> Result<Self, CurrencyErr> {
        let mut int_value = self.int_value;

        let p = if self.opts.from_cents() {
            1.
        } else {
            Self::pow(self.opts.precision())
        };

        int_value += Self::parse_string(number, &self.opts, true)?;

        Ok(Self::new_float(int_value / p, Some(self.opts)))
    }

    /// > Subtracts a number from the current instance
    ///
    /// Arguments:
    ///
    /// * `number`: The number to subtract from the current instance.
    ///
    /// Returns:
    ///
    /// A new instance of the Money struct.
    pub fn subtract(
        self,
        number: f64,
    ) -> Self {
        let mut int_value = self.int_value;

        let p = if self.opts.from_cents() {
            1.
        } else {
            Self::pow(self.opts.precision())
        };

        int_value -= Self::parse(number, &self.opts, true);

        Self::new_float(int_value / p, Some(self.opts))
    }

    /// > Multiply the value of the current instance by the given number
    ///
    /// Arguments:
    ///
    /// * `number`: The number to multiply the Money object by.
    ///
    /// Returns:
    ///
    /// A new instance of the Money struct.
    pub fn multiply(
        self,
        number: f64,
    ) -> Self {
        let mut int_value = self.int_value;
        let precision = if self.opts.from_cents() {
            1.
        } else {
            Self::pow(self.opts.precision())
        };

        int_value *= number;

        Self::new_float(int_value / precision, Some(self.opts))
    }

    /// It divides the number by the number passed in.
    ///
    /// Arguments:
    ///
    /// * `number`: The number to divide by.
    ///
    /// Returns:
    ///
    /// A new instance of the Money struct.
    pub fn divide(
        self,
        number: f64,
    ) -> Self {
        let mut int_value = self.int_value;
        if number > 0. {
            int_value /= Self::parse(number, &self.opts, false);
            Self::new_float(int_value, Some(self.opts))
        } else {
            self
        }
    }

    /// > This function takes a Money object and a count, and returns a vector of Money objects that are
    /// split evenly
    ///
    /// Arguments:
    ///
    /// * `count`: The number of items to distribute the money into.
    ///
    /// Returns:
    ///
    /// A vector of Money objects.
    pub fn distribute(
        self,
        mut count: i64,
    ) -> Vec<Self> {
        let int_value = self.int_value;

        let mut distribution: Vec<Self> = vec![];

        let split = if int_value >= 0. {
            (int_value / count as f64).floor()
        } else {
            (int_value / count as f64).ceil()
        };

        let mut pennies = int_value.abs() - split.abs() * (count as f64).abs();

        let precision = if self.opts.from_cents() {
            1.
        } else {
            Self::pow(self.opts.precision())
        };

        while count != 0 {
            let mut item = Self::new_float(split / precision, Some(self.opts.clone()));

            if pennies > 0. {
                if int_value >= 0. {
                    item = item.add(1. / precision);
                } else {
                    item = item.subtract(1. / precision);
                }
            }

            distribution.push(item);

            pennies -= 1.;
            count -= 1;
        }

        distribution
    }

    /// It rounds a floating point number to the nearest integer
    ///
    /// Arguments:
    ///
    /// * `r`: The number to round.
    ///
    /// Returns:
    ///
    /// The rounded value of the input.
    fn round(r: f64) -> f64 {
        r.round()
    }

    /// It rounds a number to the nearest increment.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to round.
    /// * `increment`: The value to round to.
    ///
    /// Returns:
    ///
    /// The value of the rounded number.
    pub(crate) fn rounding(
        value: f64,
        increment: f64,
    ) -> f64 {
        Self::round(value / increment) * increment
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

    /// It rounds a float to a given number of decimal places.
    ///
    /// Arguments:
    ///
    /// * `v`: The value to round
    /// * `dp`: The number of decimal places to round to.
    ///
    /// Returns:
    ///
    /// A function that takes two arguments, a float and an unsigned integer, and returns a float.
    fn round_dp(
        v: f64,
        dp: usize,
    ) -> f64 {
        format!("{v:.dp$}").parse::<f64>().unwrap()
    }

    /// It takes a floating point number and a number of decimal places, and returns a string
    /// representation of the floating point number rounded to the specified number of decimal places
    ///
    /// Arguments:
    ///
    /// * `v`: the value to round
    /// * `dp`: The number of decimal places to round to.
    ///
    /// Returns:
    ///
    /// A string
    pub(crate) fn round_dp_to_string(
        v: f64,
        dp: usize,
    ) -> String {
        format!("{v:.dp$}")
    }
}
