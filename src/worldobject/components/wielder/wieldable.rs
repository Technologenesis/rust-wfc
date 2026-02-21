use super::usable::UsableTrait;

pub type Wieldable = Box<dyn WieldableTrait>;

pub trait WieldableTrait: Send + Sync {
    fn wieldable_name(&self) -> String;
    fn as_usable(&mut self) -> Option<&mut dyn UsableTrait> { None }
}