use crate::quantities::product;
use crate::quantities::mass;
use crate::quantities::acceleration;
use crate::quantities;

pub type Force = product::Product<mass::Mass, acceleration::Acceleration>;

pub fn newtons(n: f64) -> quantities::Quantity<Force> {
    mass::kilograms(n) * acceleration::meters_per_second_squared(1.0)
}