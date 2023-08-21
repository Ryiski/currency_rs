use crate::Currency;
use std::ops::{Sub, SubAssign};

impl Sub for Currency {
    type Output = Currency;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        self.subtract(rhs.value())
    }
}

impl Sub<f64> for Currency {
    type Output = Currency;

    fn sub(
        self,
        rhs: f64,
    ) -> Self::Output {
        self.subtract(rhs)
    }
}

/// Assign

impl SubAssign for Currency {
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = self.clone().subtract(rhs.value());
    }
}

impl SubAssign<f64> for Currency {
    fn sub_assign(
        &mut self,
        rhs: f64,
    ) {
        *self = self.clone().subtract(rhs);
    }
}
