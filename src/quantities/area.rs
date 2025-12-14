use crate::quantities::product;
use crate::quantities::distance;
use crate::quantities;

pub type Area = product::Product<distance::Distance, distance::Distance>;

pub fn square_meters(n: f64) -> quantities::Quantity<Area> {
    distance::meters(n) * distance::meters(n)
}