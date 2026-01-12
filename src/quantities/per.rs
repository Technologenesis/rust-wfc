use crate::quantities;

use serde::Serialize;
use serde::Deserialize;

use std::cmp;
use std::fmt;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Per<Denom: quantities::QuantityTrait>
where Denom::Error: fmt::Display
{
    pub num: quantities::Quantity<quantities::Unitless>,
    pub denom: quantities::Quantity<Denom>,
}

pub fn per<
    Denom: quantities::QuantityTrait
>(denom: quantities::Quantity<Denom>) -> quantities::Quantity<Per<Denom>>
where Denom::Error: fmt::Display {
    quantities::Quantity(Per {
        num: quantities::Quantity(quantities::Unitless(1.0)),
        denom
    })
}

impl<Denom: quantities::QuantityTrait> TryFrom<serde_json::Value> for Per<Denom>
where Denom::Error: fmt::Display
{
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Per {
            num: value.get("num").ok_or(String::from("num not found"))
                .and_then(|v| quantities::Quantity::<quantities::Unitless>::try_from(v.clone()).map_err(|err| format!("failed to parse num: {}", err)))?,
            denom: value.get("denom").ok_or(String::from("denom not found"))
                .and_then(|v| quantities::Quantity::<Denom>::try_from(v.clone()).map_err(|err| format!("failed to parse denom: {}", err)))?,
        })
    }
}

impl<Denom: quantities::QuantityTrait> Per<Denom>
where Denom::Error: fmt::Display
{
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
> quantities::QuantityTrait for Per<Denom>
where Denom::Error: fmt::Display
{
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