use crate::quantities::{
    Quantity,
    force::Force,
};

use std::error::Error as StdError;

pub type ForceReceiverComponent = Box<dyn ForceReceiverTrait<Error = Box<dyn StdError>>>;

pub trait ForceReceiverTrait {
    type Error;

    fn apply_force(&self, force: &Quantity<Force>) -> Result<String, Self::Error>;
}

impl<E: StdError + 'static> ForceReceiverTrait for Box<dyn ForceReceiverTrait<Error = E>> {
    type Error = Box<dyn StdError>;

    fn apply_force(&self, force: &Quantity<Force>) -> Result<String, Self::Error> {
        (**self).apply_force(force).map_err(|e| -> Self::Error { Box::new(e) })
    }
}