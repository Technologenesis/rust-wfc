use std::cmp;

use serde::Serialize;
use serde::Deserialize;

use crate::quantities;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Distance(f64);

impl quantities::QuantityTrait for Distance {
    fn add(&self, other: &Self) -> quantities::Quantity<Self> {
        quantities::Quantity(Distance(self.0 + other.0))
    }

    fn neg(&self) -> quantities::Quantity<Self> {
        quantities::Quantity(Distance(-self.0))
    }

    fn div(&self, other: &Self) -> quantities::Quantity<quantities::Unitless> {
        quantities::Quantity(quantities::Unitless(self.0 / other.0))
    }

    fn mul(&self, other: &quantities::Quantity<quantities::Unitless>) -> quantities::Quantity<Self> {
        quantities::Quantity(Distance(self.0 * other.0.0))
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub fn centimeters(n: f64) -> quantities::Quantity<Distance> {
    quantities::Quantity(Distance(n * 0.01))
}

pub fn meters(n: f64) -> quantities::Quantity<Distance> {
    quantities::Quantity(Distance(n))
}