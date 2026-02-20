use crate::worldobject::components::wielder::wieldable::WieldableTrait;
use super::Hand;

#[derive(Debug)]
pub struct HandUseError;

impl std::error::Error for HandUseError {}

impl std::fmt::Display for HandUseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "you can't think of anything particularly interesting to do with this hand")
    }
}

impl WieldableTrait for Hand {}