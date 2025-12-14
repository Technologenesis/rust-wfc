use crate::quantities::per;
use crate::quantities::product;
use crate::quantities::speed;
use crate::quantities::duration;
use crate::quantities;

pub type Acceleration = product::Product<speed::Speed, per::Per<duration::Duration>>;

pub fn meters_per_second_squared(n: f64) -> quantities::Quantity<Acceleration> {
    speed::meters_per_second(n) / duration::seconds(1.0)
}