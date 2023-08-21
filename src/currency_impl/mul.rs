use crate::Currency;
use std::ops::{Mul, MulAssign};

impl Mul<f64> for Currency {
    type Output = Self;

    fn mul(
        self,
        rhs: f64,
    ) -> Self::Output {
        self.multiply(rhs)
    }
}

/// Assign

impl MulAssign<f64> for Currency {
    fn mul_assign(
        &mut self,
        rhs: f64,
    ) {
        *self = self.clone().multiply(rhs)
    }
}
