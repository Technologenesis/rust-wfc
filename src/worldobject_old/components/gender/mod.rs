use serde::Serialize;
use serde::Deserialize;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl TryFrom<&str> for Gender {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            "other" => Ok(Self::Other),
            _ => Err("invalid gender"),
        }
    }
}

impl TryFrom<&serde_json::Value> for Gender {
    type Error = &'static str;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let value = value.as_str().ok_or("invalid gender")?;
        Self::try_from(value)
    }
}

impl Gender {
    pub fn noun(&self) -> &str {
        match self {
            Self::Male => "man",
            Self::Female => "woman",
            Self::Other => "person",
        }
    }

    pub fn subject_pronoun(&self) -> &str {
        match self {
            Self::Male => "he",
            Self::Female => "she",
            Self::Other => "they",
        }
    }

    pub fn object_pronoun(&self) -> &str {
        match self {
            Self::Male => "him",
            Self::Female => "her",
            Self::Other => "them",
        }
    }

    pub fn possessive_pronoun(&self) -> &str {
        match self {
            Self::Male => "his",
            Self::Female => "her",
            Self::Other => "their",
        }
    }
}