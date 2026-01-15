pub mod per;
pub mod product;

pub mod distance;
pub mod area;
pub mod volume;

pub mod mass;
pub mod density;

use serde::Serialize;
use serde::Deserialize;

pub mod direction;
pub mod speed;
pub mod duration;
pub mod acceleration;
pub mod force;

use std::ops;
use std::cmp;
use std::fmt;

pub trait QuantityTrait: Clone + PartialEq + TryFrom<serde_json::Value> {
    fn add(&self, other: &Self) -> Quantity<Self>;
    fn neg(&self) -> Quantity<Self>;
    fn div(&self, other: &Self) -> Quantity<Unitless>;
    fn mul(&self, other: &Quantity<Unitless>) -> Quantity<Self>;
    fn cmp(&self, other: &Self) -> cmp::Ordering;
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Unitless (pub f64);

impl TryFrom<serde_json::Value> for Unitless {
    type Error = &'static str;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Unitless(value.as_f64().ok_or("failed to parse unitless")?))
    }
}

impl QuantityTrait for Unitless {
    fn add(&self, other: &Self) -> Quantity<Self> {
        Quantity(Unitless(self.0 + other.0))
    }

    fn neg(&self) -> Quantity<Self> {
        Quantity(Unitless(-self.0))
    }

    fn div(&self, other: &Self) -> Quantity<Unitless> {
        Quantity(Unitless(self.0 / other.0))
    }

    fn mul(&self, other: &Quantity<Unitless>) -> Quantity<Self> {
        Quantity(Unitless(self.0 * other.0.0))
    }

    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl fmt::Debug for Unitless {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Unitless {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quantity<T: QuantityTrait + ?Sized + TryFrom<serde_json::Value>>(pub T);

impl<T: QuantityTrait + ?Sized + TryFrom<serde_json::Value>> TryFrom<serde_json::Value> for Quantity<T>
where T::Error: fmt::Display
{
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        T::try_from(value).map_err(|err| format!("failed to parse quantity: {}", err))
            .map(|t| Quantity(t))
    }
}

impl<T: QuantityTrait> ops::Add<&Quantity<T>> for &Quantity<T> {
    type Output = Quantity<T>;

    fn add(self, other: &Quantity<T>) -> Quantity<T> {
        self.0.add(&other.0)
    }
}

impl<T: QuantityTrait> ops::Add<Quantity<T>> for Quantity<T> {
    type Output = Quantity<T>;

    fn add(self, other: Quantity<T>) -> Quantity<T> {
        self.0.add(&other.0)
    }
}

impl<T: QuantityTrait> ops::Sub<&Quantity<T>> for &Quantity<T> {
    type Output = Quantity<T>;

    fn sub(self, other: &Quantity<T>) -> Quantity<T> {
        self + &-other
    }
}

impl<T: QuantityTrait> ops::Sub<Quantity<T>> for Quantity<T> {
    type Output = Quantity<T>;

    fn sub(self, other: Quantity<T>) -> Quantity<T> {
        self + -other
    }
}

impl<T: QuantityTrait> ops::Neg for &Quantity<T> {
    type Output = Quantity<T>;

    fn neg(self) -> Quantity<T> {
        self.0.neg()
    }
}

impl<T: QuantityTrait> ops::Neg for Quantity<T> {
    type Output = Quantity<T>;

    fn neg(self) -> Quantity<T> {
        self.0.neg()
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> ops::Mul<&Quantity<Q2>> for &Quantity<Q1>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    type Output = Quantity<product::Product<Q1, Q2>>;

    fn mul(self, other: &Quantity<Q2>) -> Quantity<product::Product<Q1, Q2>> {
        product::product(self.clone(), other.clone())
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> ops::Mul<Quantity<Q2>> for Quantity<Q1>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    type Output = Quantity<product::Product<Q1, Q2>>;

    fn mul(self, other: Quantity<Q2>) -> Quantity<product::Product<Q1, Q2>> {
        product::product(self.clone(), other.clone())
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> ops::Div<Quantity<Q2>> for Quantity<Q1>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    type Output = Quantity<product::Product<Q1, per::Per<Q2>>>;

    fn div(self, other: Quantity<Q2>) -> Quantity<product::Product<Q1, per::Per<Q2>>> {
        product::product(self.clone(), per::per(other.clone()))
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> ops::Div<&Quantity<Q2>> for &Quantity<Q1>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    type Output = Quantity<product::Product<Q1, per::Per<Q2>>>;

    fn div(self, other: &Quantity<Q2>) -> Quantity<product::Product<Q1, per::Per<Q2>>> {
        product::product(self.clone(), per::per(other.clone()))
    }
}

impl<Q: QuantityTrait> cmp::PartialOrd<Quantity<Q>> for Quantity<Q> {
    fn partial_cmp(&self, other: &Quantity<Q>) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<Q: QuantityTrait> Quantity<product::Product<Q, per::Per<Q>>>
where Q::Error: fmt::Display
{
    pub fn cancel(self) -> Quantity<Unitless> {
        self.0.a.0.div(&self.0.b.0.denom.0)
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> Quantity<product::Product<Q1, product::Product<Q2, per::Per<Q2>>>>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    pub fn cancel(self) -> Quantity<Q1> {
        let uncancelled = self.0.a;

        let cancelled = self.0.b.0;

        let q2_ratio = &cancelled.a.0.div(&cancelled.b.0.denom.0);

        (&uncancelled * &q2_ratio).cancel()
    }
}

impl<Q: QuantityTrait> Quantity<product::Product<Q, Unitless>>
where Q::Error: fmt::Display
{
    pub fn cancel(self) -> Quantity<Q> {
        self.0.a.0.mul(&self.0.b)
    }
}


impl<Q1: QuantityTrait, Q2: QuantityTrait, Q3: QuantityTrait> Quantity<product::Product<Q1, product::Product<Q2, Q3>>>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display, Q3::Error: fmt::Display
{
    pub fn associate_left(&self) -> Quantity<product::Product<product::Product<Q1, Q2>, Q3>> {
        product::product(product::product(self.0.a.clone(), self.0.b.0.a.clone()), self.0.b.0.b.clone())
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait, Q3: QuantityTrait> Quantity<product::Product<product::Product<Q1, Q2>, Q3>>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display, Q3::Error: fmt::Display
{
    pub fn associate_right(&self) -> Quantity<product::Product<Q1, product::Product<Q2, Q3>>> {
        product::product(self.0.a.0.a.clone(), product::product(self.0.a.0.b.clone(), self.0.b.clone()))
    }
}

impl<Q1: QuantityTrait, Q2: QuantityTrait> Quantity<product::Product<Q1, Q2>>
where Q1::Error: fmt::Display, Q2::Error: fmt::Display
{
    pub fn commute(&self) -> Quantity<product::Product<Q2, Q1>> {
        product::product(self.0.b.clone(), self.0.a.clone())
    }
}

impl<T: QuantityTrait + fmt::Debug> fmt::Debug for Quantity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T:QuantityTrait + fmt::Display> fmt::Display for Quantity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}