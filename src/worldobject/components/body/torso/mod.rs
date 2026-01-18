pub mod arm;

use serde::Serialize;

use crate::{
    quantities::{
        Quantity,
        mass::Mass,
    },
    worldobject::{
        TypedWorldObject
    },
};

use arm::Arm;

#[derive(Serialize)]
pub struct Torso {
    pub base_mass: Quantity<Mass>,
    pub left_arm: Arm,
    pub right_arm: Arm,
}

impl Torso {
    pub fn mass(&self) -> Quantity<Mass> {
        self.base_mass.clone() + self.left_arm.mass() + self.right_arm.mass()
    }

    pub fn dummy(&self) -> Torso {
        Torso {
            base_mass: self.base_mass.clone(),
            left_arm: <Arm as TypedWorldObject>::dummy(&self.left_arm),
            right_arm: <Arm as TypedWorldObject>::dummy(&self.right_arm),
        }
    }
}

impl TryFrom<&serde_json::Value> for Torso {
    type Error = String;

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let base_mass = Quantity::<Mass>::try_from(value.get("base_mass").ok_or("base_mass not found")?.clone()).map_err(|err| format!("failed to parse base_mass: {}", err))?;
        let left_arm = Arm::try_from(value.get("left_arm").ok_or("left_arm not found")?).map_err(|err| format!("failed to parse left_arm: {}", err))?;
        let right_arm = Arm::try_from(value.get("right_arm").ok_or("right_arm not found")?).map_err(|err| format!("failed to parse right_arm: {}", err))?;
        Ok(Torso { base_mass, left_arm, right_arm })
    }
}