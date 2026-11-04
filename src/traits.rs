use crate::{currency_opts::js_round, js_to_fixed, Currency, CurrencyErr, CurrencyOpts};
use std::{
    cmp::Ordering,
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

/// Plain amount rounded to `increment`, no symbol or grouping: `1234.50`.
impl fmt::Display for Currency {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let inc = self.opts.increment_or_default();
        let rounded = js_round(self.value / inc) * inc;
        f.write_str(&js_to_fixed(rounded, self.opts.precision as usize))
    }
}

impl From<f64> for Currency {
    fn from(value: f64) -> Self {
        let opts = CurrencyOpts::default();
        Self::with_int(opts.parse_num(value, true), opts)
    }
}

impl FromStr for Currency {
    type Err = CurrencyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opts = CurrencyOpts::default();
        Ok(Self::with_int(opts.parse_str(s)?, opts))
    }
}

impl TryFrom<&str> for Currency {
    type Error = CurrencyErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

// Comparisons use `value`, so amounts with different precisions compare sensibly.
impl PartialEq for Currency {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.value == other.value
    }
}

impl PartialEq<f64> for Currency {
    fn eq(
        &self,
        other: &f64,
    ) -> bool {
        self.value == *other
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<f64> for Currency {
    fn partial_cmp(
        &self,
        other: &f64,
    ) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

/// `$body` returns the new input value. The result keeps the left-hand side's options;
/// owned and assign forms reuse `self` so only `&a op b` clones.
macro_rules! impl_op {
    ($Op:ident $op:ident, $OpAssign:ident $op_assign:ident, $Rhs:ty, |$c:ident, $r:ident| $body:expr) => {
        impl $Op<$Rhs> for &Currency {
            type Output = Currency;

            fn $op(
                self,
                $r: $Rhs,
            ) -> Currency {
                let $c = self;
                let input = $body;
                let mut out = self.clone();
                out.set_input(input);
                out
            }
        }

        impl $Op<$Rhs> for Currency {
            type Output = Currency;

            fn $op(
                mut self,
                $r: $Rhs,
            ) -> Currency {
                self.$op_assign($r);
                self
            }
        }

        impl $OpAssign<$Rhs> for Currency {
            fn $op_assign(
                &mut self,
                $r: $Rhs,
            ) {
                let $c = &*self;
                let input = $body;
                self.set_input(input);
            }
        }
    };
}

impl_op!(Add add, AddAssign add_assign, f64, |c, n| c.input_of(c.int_value + c.opts.parse_num(n, true)));
impl_op!(Add add, AddAssign add_assign, &Currency, |c, r| c.input_of(c.int_value + c.int_of(r)));
impl_op!(Add add, AddAssign add_assign, Currency, |c, r| c.input_of(c.int_value + c.int_of(&r)));
impl_op!(Sub sub, SubAssign sub_assign, f64, |c, n| c.input_of(c.int_value - c.opts.parse_num(n, true)));
impl_op!(Sub sub, SubAssign sub_assign, &Currency, |c, r| c.input_of(c.int_value - c.int_of(r)));
impl_op!(Sub sub, SubAssign sub_assign, Currency, |c, r| c.input_of(c.int_value - c.int_of(&r)));
impl_op!(Mul mul, MulAssign mul_assign, f64, |c, n| c.input_of(c.int_value * n));
// Unlike the others, currency.js passes this straight to `currency()` without unscaling.
impl_op!(Div div, DivAssign div_assign, f64, |c, n| c.int_value / c.opts.parse_num(n, false));

/// Sums with the first item's options; an empty iterator gives `0` with default options.
impl Sum for Currency {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        match iter.next() {
            Some(first) => iter.fold(first, |acc, c| acc + c),
            None => Self::default(),
        }
    }
}

impl<'a> Sum<&'a Currency> for Currency {
    fn sum<I: Iterator<Item = &'a Currency>>(mut iter: I) -> Self {
        match iter.next() {
            Some(first) => iter.fold(first.clone(), |acc, c| acc + c),
            None => Self::default(),
        }
    }
}

/// Serializes as the plain number, like currency.js `toJSON`.
#[cfg(feature = "serde")]
impl serde::Serialize for Currency {
    fn serialize<S: serde::Serializer>(
        &self,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        s.serialize_f64(self.value)
    }
}
