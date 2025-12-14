use crate::quantities::product;
use crate::quantities::per;
use crate::quantities::mass;
use crate::quantities::volume;

pub type Density = product::Product<mass::Mass, per::Per<volume::Volume>>;