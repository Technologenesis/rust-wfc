use serde::Serialize;
use serde::Deserialize;

use std::fmt;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum DirectionHorizontal {
    Left,
    Right,
}

impl TryFrom<&serde_json::Value> for DirectionHorizontal {
    type Error = InvalidHorizontalDirectionError;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        match value.as_str().unwrap_or("invalid horizontal direction") {
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            _ => Err(InvalidHorizontalDirectionError(value.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct InvalidHorizontalDirectionError(String);

impl fmt::Display for InvalidHorizontalDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid horizontal direction: {}", self.0)
    }
}

impl std::error::Error for InvalidHorizontalDirectionError {}

impl TryFrom<&str> for DirectionHorizontal {
    type Error = InvalidHorizontalDirectionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(InvalidHorizontalDirectionError(value.to_string())),
        }
    }
}

impl fmt::Display for DirectionHorizontal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DirectionVertical {
    Up,
    Down,
}

#[derive(Debug)]
pub struct InvalidVerticalDirectionError(String);

impl TryFrom<&str> for DirectionVertical {
    type Error = InvalidVerticalDirectionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(InvalidVerticalDirectionError(value.to_string())),
        }
    }
}

impl fmt::Display for DirectionVertical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Up => write!(f, "up"),
            Self::Down => write!(f, "down"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DirectionHorizontalOrVertical {
    Horizontal(DirectionHorizontal),
    Vertical(DirectionVertical),
}

impl fmt::Display for DirectionHorizontalOrVertical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Horizontal(horizontal) => write!(f, "{}", horizontal),
            Self::Vertical(vertical) => write!(f, "{}", vertical),
        }
    }
}

pub struct InvalidDirectionHorizontalOrVerticalError(String);

impl fmt::Display for InvalidDirectionHorizontalOrVerticalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid direction horizontal or vertical: {}", self.0)
    }
}

impl TryFrom<&str> for DirectionHorizontalOrVertical {
    type Error = InvalidDirectionHorizontalOrVerticalError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DirectionHorizontal::try_from(value)
            .map(Self::Horizontal)
            .or_else(|_| DirectionVertical::try_from(value)
                .map(Self::Vertical))
            .map_err(|_| InvalidDirectionHorizontalOrVerticalError(value.to_string()))
    }
}