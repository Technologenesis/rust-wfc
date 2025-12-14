use crate::quantities::distance;
use crate::quantities::per;
use crate::quantities::product;
use crate::quantities::duration;
use crate::quantities;

pub type Speed = product::Product<distance::Distance, per::Per<duration::Duration>>;

pub fn meters_per_second(n: f64) -> quantities::Quantity<Speed> {
    distance::meters(n) / duration::seconds(1.0)
}