pub mod head;
pub mod torso;
pub mod legs;

use serde::Serialize;

use crate::quantities::{
    Quantity,
    mass::Mass,
};

use {
    legs::Legs,
    torso::Torso,
    head::Head,
};

#[derive(Serialize)]
pub struct Body {
    pub base_mass: Quantity<Mass>,

    pub head: Head,
    pub torso: Torso,
    pub legs: Legs,
}

impl Body {
    pub fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone() + self.torso.mass() + self.legs.mass() + self.head.mass()
    }

    pub fn dummy(&self) -> Body {
        Body {
            base_mass: self.base_mass.clone(),
            head: self.head.dummy(),
            torso: self.torso.dummy(),
            legs: self.legs.dummy(),
        }
    }
}

impl TryFrom<&serde_json::Value> for Body {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = Quantity::<Mass>::try_from(value.get("base_mass").ok_or("base_mass not found")?.clone()).map_err(|err| format!("failed to parse base_mass: {}", err))?;
        let head = Head::try_from(value.get("head").ok_or("head not found")?).map_err(|err| format!("failed to parse head: {}", err))?;
        let torso = Torso::try_from(value.get("torso").ok_or("torso not found")?).map_err(|err| format!("failed to parse torso: {}", err))?;
        let legs = Legs::try_from(value.get("legs").ok_or("legs not found")?).map_err(|err| format!("failed to parse legs: {}", err))?;
        Ok(Body { base_mass, head, torso, legs })
    }
}