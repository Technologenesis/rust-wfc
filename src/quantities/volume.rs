use crate::quantities::product;
use crate::quantities::distance;
use crate::quantities::area;

pub type Volume = product::Product<distance::Distance, area::Area>;