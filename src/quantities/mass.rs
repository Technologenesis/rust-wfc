use crate::quantities;

use serde::Serialize;
use serde::Deserialize;

use std::cmp;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Mass {
    pub grams: f64
}

impl TryFrom<serde_json::Value> for Mass {
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Mass {
            grams: value.get("grams").ok_or(String::from("grams not found"))
                .and_then(|v| v.as_f64().ok_or(String::from("failed to parse grams")))?
        })
    }
}

pub fn grams(n: f64) -> quantities::Quantity<Mass> {
    quantities::Quantity(Mass { grams: n })
}

pub fn kilograms(n: f64) -> quantities::Quantity<Mass> {
    quantities::Quantity(Mass { grams: n * 1000.0 })
}

impl quantities::QuantityTrait for Mass {
    fn add(&self, other: &Self) -> quantities::Quantity<Self> {
        quantities::Quantity(Mass { grams: self.grams + other.grams })
    }

    fn neg(&self) -> quantities::Quantity<Self> {
        quantities::Quantity(Mass { grams: -self.grams })
    }

    fn div(&self, other: &Self) -> quantities::Quantity<quantities::Unitless> {
        quantities::Quantity(quantities::Unitless(self.grams / other.grams))
    }

    fn mul(&self, other: &quantities::Quantity<quantities::Unitless>) -> quantities::Quantity<Self> {
        quantities::Quantity(Mass { grams: self.grams * other.0.0 })
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.grams.total_cmp(&other.grams)
    }
}