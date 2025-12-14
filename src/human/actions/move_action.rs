use crate::quantities;
use crate::quantities::distance;
use crate::quantities::direction;

use serde::Serialize;
use serde::Deserialize;

use std::fmt;

#[derive(Debug)]
pub enum MoveActionParseError {
    NoDirectionProvided,
    InvalidDirection(String),
    NoDistanceProvided,
    InvalidDistance(String),
}

impl fmt::Display for MoveActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDirectionProvided => write!(f, "no direction provided"),
            Self::InvalidDirection(dir_str) => write!(f, "invalid direction \"{}\"", dir_str),
            Self::NoDistanceProvided => write!(f, "no distance provided"),
            Self::InvalidDistance(dist_str) => write!(f, "invalid distance \"{}\"", dist_str)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MoveAction {
    pub direction: direction::DirectionHorizontalOrVertical,
    pub distance: quantities::Quantity<distance::Distance>
}

impl MoveAction {
    pub fn parse<'a, I: Iterator<Item = &'a str>>(words: &mut std::iter::Peekable<I>) -> Result<Self, MoveActionParseError> {
        let direction = words.next().ok_or(MoveActionParseError::NoDirectionProvided)?;
        let direction = direction::DirectionHorizontalOrVertical::try_from(direction)
            .map_err(|_| MoveActionParseError::InvalidDirection(direction.to_string()))?;

        let distance_meters_str = words.next().ok_or(MoveActionParseError::NoDistanceProvided)?;

        let distance_meters = distance_meters_str.parse::<f64>()
            .map_err(|_| MoveActionParseError::InvalidDistance(distance_meters_str.to_string()))?;

        Ok(MoveAction { direction, distance: distance::meters(distance_meters) })
    }
}