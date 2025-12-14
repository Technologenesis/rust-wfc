use std::fmt::{self, write};

use crate::quantities;
use crate::quantities::density;

#[derive(Clone)]
pub enum Material {
    Bronze,
    Iron,
    Steel,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bronze => write!(f, "bronze"),
            Self::Iron => write!(f, "iron"),
            Self::Steel => write!(f, "steel")
        }
    }
}