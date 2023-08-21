pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

use crate::Currency;
use std::fmt::Display;

impl Display for Currency {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let precision = self.opts.precision();

        let increment = self.opts.increment();

        let rounded_value = Self::rounding(self.value, increment);

        let currency: String = Self::round_dp_to_string(rounded_value, precision as usize);

        write!(f, "{currency}")
    }
}
