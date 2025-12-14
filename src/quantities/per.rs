use crate::quantities;

use serde::Serialize;
use serde::Deserialize;

use std::cmp;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Per<Denom: quantities::QuantityTrait>{
    pub num: quantities::Quantity<quantities::Unitless>,
    pub denom: quantities::Quantity<Denom>,
}

pub fn per<
    Denom: quantities::QuantityTrait
>(denom: quantities::Quantity<Denom>) -> quantities::Quantity<Per<Denom>> {
    quantities::Quantity(Per {
        num: quantities::Quantity(quantities::Unitless(1.0)),
        denom
    })
}

impl<Denom: quantities::QuantityTrait> Per<Denom> {
    pub fn renormalize(&self, new_denom: &quantities::Quantity<Denom>) -> quantities::Quantity<Self> {
        let ratio_new_denom_to_self_denom = new_denom / &self.denom;

        let new_num = (&self.num * &(ratio_new_denom_to_self_denom.cancel())).cancel();

        quantities::Quantity(Per {
            num: new_num,
            denom: new_denom.clone()
        })
    }
}

impl<
    Denom: quantities::QuantityTrait
> quantities::QuantityTrait for Per<Denom> {
    fn add(&self, other: &Self) -> quantities::Quantity<Self> {
        let other_normalized = other.renormalize(&self.denom);

        quantities::Quantity(Per {
            num: &self.num + &other_normalized.0.num,
            denom: self.denom.clone()
        })
    }

    fn neg(&self) -> quantities::Quantity<Self> {
        quantities::Quantity(Per {
            num: -self.num.clone(),
            denom: self.denom.clone()
        })
    }

    fn div(&self, other: &Self) -> quantities::Quantity<quantities::Unitless> {
        (&self.num / &other.num).cancel()
    }

    fn mul(&self, other: &quantities::Quantity<quantities::Unitless>) -> quantities::Quantity<Self> {
        quantities::Quantity(Per {
            num: (&self.num * other).cancel(),
            denom: self.denom.clone()
        })
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.num.0.cmp(&other.num.0)
    }
}