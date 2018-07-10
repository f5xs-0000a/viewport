#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum UnitLength {
    Width,
    Height,
    MaxSide,
    MinSide,
}

use self::UnitLength::*;

impl UnitLength {
    pub fn is_width(&self) -> bool {
        *self == Width
    }

    pub fn is_height(&self) -> bool {
        *self == Height
    }

    pub fn is_max_side(&self) -> bool {
        *self == MaxSide
    }

    pub fn is_min_side(&self) -> bool {
        *self == MinSide
    }
}
