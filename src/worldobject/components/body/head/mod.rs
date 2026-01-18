use serde::Serialize;

use crate::quantities::{
    Quantity,
    distance::Distance,
    mass::{
        Mass,
        grams,
    },
};

#[derive(Serialize)]
pub struct Head {
    pub base_mass: Quantity<Mass>,
    pub mouth: Mouth,
}

impl Head {
    pub fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone() + self.mouth.mass()
    }

    pub fn dummy(&self) -> Head {
        Head {
            base_mass: self.base_mass.clone(),
            mouth: self.mouth.dummy(),
        }
    }
}

impl TryFrom<&serde_json::Value> for Head {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = Quantity::<Mass>::try_from(value.get("base_mass").ok_or("base_mass not found")?.clone()).map_err(|err| format!("failed to parse base_mass: {}", err))?;
        let mouth = Mouth::try_from(value.get("mouth").ok_or("mouth not found")?).map_err(|err| format!("failed to parse mouth: {}", err))?;
        Ok(Head { base_mass, mouth })
    }
}

#[derive(Serialize)]
pub struct Mouth {
    pub base_mass: Quantity<Mass>,
    pub teeth: Vec<Tooth>,
}

impl Mouth {
    pub fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone() + self.teeth.iter().fold(grams(0.0), |acc, tooth| acc + tooth.mass())
    }

    pub fn dummy(&self) -> Mouth {
        Mouth {
            base_mass: self.base_mass.clone(),
            teeth: self.teeth.iter().map(|tooth| tooth.dummy()).collect(),
        }
    }
}

impl TryFrom<&serde_json::Value> for Mouth {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = Quantity::<Mass>::try_from(value.get("base_mass").ok_or("base_mass not found")?.clone()).map_err(|err| format!("failed to parse base_mass: {}", err))?;
        let teeth = value.get("teeth").ok_or("teeth not found")?.as_array().ok_or("teeth not found")?.iter().map(|tooth| Tooth::try_from(tooth)).collect::<Result<Vec<Tooth>, String>>()?;
        Ok(Mouth { base_mass, teeth })
    }
}

#[derive(Serialize)]
pub struct Tooth {
    pub mass: Quantity<Mass>,
    pub length: Quantity<Distance>,
    pub sharp: bool,
}

impl Tooth {
    pub fn mass(&self) -> Quantity<Mass> {
        self.mass.clone()
    }

    pub fn dummy(&self) -> Tooth {
        Tooth {
            mass: self.mass.clone(),
            length: self.length.clone(),
            sharp: self.sharp,
        }
    }
}

impl TryFrom<&serde_json::Value> for Tooth {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mass = Quantity::<Mass>::try_from(value.get("mass").ok_or("mass not found")?.clone()).map_err(|err| format!("failed to parse mass: {}", err))?;
        let length = Quantity::<Distance>::try_from(value.get("length").ok_or("length not found")?.clone()).map_err(|err| format!("failed to parse length: {}", err))?;
        let sharp = value.get("sharp").ok_or("sharp not found")?.as_bool().ok_or("sharp not found")?;
        Ok(Tooth { mass, length, sharp })
    }
}