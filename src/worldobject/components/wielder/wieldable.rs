pub type Wieldable = Box<dyn WieldableTrait>;

pub trait WieldableTrait: Send + Sync {}