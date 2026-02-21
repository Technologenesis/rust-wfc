use std::error::Error as StdError;

use crate::lang::VerbPhrase;
use crate::world::handle::WorldObjectHandle;

use super::wieldable::WieldableTrait;

pub struct UseEffect {
    pub verb_phrase: VerbPhrase,
    pub message: String,
}

pub trait UsableTrait: WieldableTrait {
    fn use_item(&mut self, target: Option<&WorldObjectHandle>) -> Result<UseEffect, Box<dyn StdError>>;
}
