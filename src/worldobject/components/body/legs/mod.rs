use serde::Serialize;

use crate::quantities::{
    Quantity,
    speed::Speed,
    mass::Mass,
};

#[derive(Serialize)]
pub struct Legs {
    pub base_mass: Quantity<Mass>,
    pub speed: Quantity<Speed>,
}

impl Legs {
    pub fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone()
    }

    pub fn dummy(&self) -> Legs {
        Legs {
            base_mass: self.base_mass.clone(),
            speed: self.speed.clone(),
        }
    }
}

impl TryFrom<&serde_json::Value> for Legs {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = Quantity::<Mass>::try_from(value.get("base_mass").ok_or("base_mass not found")?.clone()).map_err(|err| format!("failed to parse base_mass: {}", err))?;
        let speed = Quantity::<Speed>::try_from(value.get("speed").ok_or("speed not found")?.clone()).map_err(|err| format!("failed to parse speed: {}", err))?;
        Ok(Legs { base_mass, speed })
    }
}