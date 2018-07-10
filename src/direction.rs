#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Direction {
    Center,
    Top,
    Bottom,
    Left,
    Right,
}

use self::Direction::*;

impl Direction {
    pub fn is_center(&self) -> bool {
        *self == Center
    }

    pub fn is_top(&self) -> bool {
        *self == Top
    }

    pub fn is_bottom(&self) -> bool {
        *self == Bottom
    }

    pub fn is_left(&self) -> bool {
        *self == Left
    }

    pub fn is_right(&self) -> bool {
        *self == Right
    }
}
