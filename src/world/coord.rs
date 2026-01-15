use crate::quantities::{
    Quantity,
    distance::Distance,
    direction::{
        DirectionHorizontalOrVertical,
        DirectionVertical,
        DirectionHorizontal,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct WorldCoord {
    pub x: Quantity<Distance>,
    pub y: Quantity<Distance>
}

impl WorldCoord {
    pub fn new(x: Quantity<Distance>, y: Quantity<Distance>) -> WorldCoord {
        WorldCoord { x, y }
    }

    pub fn translate_direction(
        &mut self,
        direction: &DirectionHorizontalOrVertical,
        distance: &Quantity<Distance>
    ) {
        match &direction {
            &DirectionHorizontalOrVertical::Vertical(vertical_direction) => {
                match vertical_direction {
                    DirectionVertical::Up => self.y = &self.y + distance,
                    DirectionVertical::Down => self.y = &self.y - distance
                }
            },
            &DirectionHorizontalOrVertical::Horizontal(horizontal_direction) => {
                match horizontal_direction {
                    DirectionHorizontal::Right => self.x = &self.x + distance,
                    DirectionHorizontal::Left => self.x = &self.x - distance
                }
            },
        }
    }
}