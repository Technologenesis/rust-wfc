use serde::Serialize;
use serde::Deserialize;

use crate::quantities;

use std::cmp;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Duration (pub f64);

impl TryFrom<serde_json::Value> for Duration {
    type Error = &'static str;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Duration(value.as_f64().ok_or("failed to parse duration")?))
    }
}

impl quantities::QuantityTrait for Duration {
    fn add(&self, other: &Self) -> quantities::Quantity<Self> {
        quantities::Quantity(Duration(self.0 + other.0))
    }

    fn neg(&self) -> quantities::Quantity<Self> {
        quantities::Quantity(Duration(-self.0))
    }

    fn div(&self, other: &Self) -> quantities::Quantity<quantities::Unitless> {
        quantities::Quantity(quantities::Unitless(self.0 / other.0))
    }

    fn mul(&self, other: &quantities::Quantity<quantities::Unitless>) -> quantities::Quantity<Self> {
        quantities::Quantity(Duration(self.0 * other.0.0))
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub fn seconds(n: f64) -> quantities::Quantity<Duration> {
    quantities::Quantity(Duration(n))
}