use crate::Currency;
use std::ops::{Div, DivAssign};

impl Div<f64> for Currency {
    type Output = Self;

    fn div(
        self,
        rhs: f64,
    ) -> Self::Output {
        self.divide(rhs)
    }
}

/// Assign

impl DivAssign<f64> for Currency {
    fn div_assign(
        &mut self,
        rhs: f64,
    ) {
        *self = self.clone().divide(rhs);
    }
}
