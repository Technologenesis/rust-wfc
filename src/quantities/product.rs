use crate::quantities;

use serde::Serialize;
use serde::Deserialize;

use std::cmp;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Product<A: quantities::QuantityTrait, B: quantities::QuantityTrait> {
    pub a: quantities::Quantity<A>,
    pub b: quantities::Quantity<B>,
}

pub fn product<A: quantities::QuantityTrait, B: quantities::QuantityTrait>(a: quantities::Quantity<A>, b: quantities::Quantity<B>) -> quantities::Quantity<Product<A, B>> {
    quantities::Quantity(Product { a, b })
}

impl<A: quantities::QuantityTrait, B: quantities::QuantityTrait> quantities::QuantityTrait for Product<A, B> {
    fn add(&self, other: &Self) -> quantities::Quantity<Self> {
        let ratio_other_a_to_self_a = (&self.a / &other.a).cancel();

        let other_b_normalized = (&other.b * &ratio_other_a_to_self_a).cancel();

        &self.a * &(&self.b + &other_b_normalized)
    }

    fn neg(&self) -> quantities::Quantity<Self> {
        quantities::Quantity(Product {
            a: self.a.clone(),
            b: -self.b.clone()
        })
    }

    fn div(&self, other: &Self) -> quantities::Quantity<quantities::Unitless> {
        ((&self.a / &other.a).cancel() * (&self.b / &other.b).cancel()).cancel()
    }

    fn mul(&self, other: &quantities::Quantity<quantities::Unitless>) -> quantities::Quantity<Self> {
        product((&self.a * other).cancel(), self.b.clone())
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let ratio_self_a_to_other_a = (&self.a / &other.a).cancel();

        let other_b_normalized = (&other.b * &ratio_self_a_to_other_a).cancel();

        self.b.0.cmp(&other_b_normalized.0)
    }
}