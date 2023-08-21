use crate::{Currency, CurrencyErr};
use std::ops::{Add, AddAssign};

impl Add for Currency {
    type Output = Currency;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.add(rhs.value())
    }
}

impl Add<f64> for Currency {
    type Output = Currency;

    fn add(
        self,
        rhs: f64,
    ) -> Self::Output {
        self.add(rhs)
    }
}

impl Add<&str> for Currency {
    type Output = Result<Currency, CurrencyErr>;

    fn add(
        self,
        rhs: &str,
    ) -> Self::Output {
        self.add_string(rhs)
    }
}

/// Assign

impl AddAssign for Currency {
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone().add(rhs.value())
    }
}

impl AddAssign<f64> for Currency {
    fn add_assign(
        &mut self,
        rhs: f64,
    ) {
        *self = self.clone().add(rhs)
    }
}
