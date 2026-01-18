pub mod gender;
pub mod body;

use serde::{Serialize, Deserialize};

use crate::{
    quantities::{self, Quantity, mass::Mass, speed::Speed, direction::DirectionHorizontal},
    worldobject::components::inventory::{Inventory}
};

use gender::Gender;
use body::arm::Arm;

// An UnsouledHuman is just a human character without a controller.
// The `dummy` method of human returns an UnsouledHuman.
#[derive(Serialize)]
pub struct UnsouledHuman {
    // identity
    pub name: String,
    pub gender: Gender,

    // body
    pub mass: Quantity<Mass>,

    // legs
    pub speed: Quantity<Speed>,

    // arms
    pub dominant_arm: DirectionHorizontal,
    pub arm_left: Option<Arm>,
    pub arm_right: Option<Arm>,

    // state
    pub inventory: Inventory,
}

impl UnsouledHuman {
    pub fn new(
        name: String,
        gender: Gender,
        speed: Quantity<Speed>,
        arm_left: Option<Arm>,
        arm_right: Option<Arm>,
        dominant_arm: DirectionHorizontal,
        mass: quantities::Quantity<Mass>,
        inventory: Inventory,
    ) -> UnsouledHuman {
        UnsouledHuman {
            name,
            mass,
            gender,
            speed,
            arm_left,
            arm_right,
            dominant_arm,
            inventory,
        }
    }
}
