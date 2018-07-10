use na::Matrix3;

////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Origin {
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

use self::Origin::*;

impl Origin {
    pub fn is_center(&self) -> bool {
        *self == Center
    }

    pub fn is_top_left(&self) -> bool {
        *self == TopLeft
    }

    pub fn is_top_right(&self) -> bool {
        *self == TopRight
    }

    pub fn is_bottom_left(&self) -> bool {
        *self == BottomLeft
    }

    pub fn is_bottom_right(&self) -> bool {
        *self == BottomRight
    }

    fn will_flip_x(&self) -> bool {
        match self {
            TopRight | BottomRight => true,
            _ => false,
        }
    }

    fn will_flip_y(&self) -> bool {
        match self {
            TopLeft | TopRight => true,
            _ => false,
        }
    }

    pub fn from_center_matrix(&self) -> Matrix3<f64> {
        match self {
            let mut affine = Matrix3::identity();

            if self.will_flip_x() {
                affine = Matrix3::new(-1., 0., 0.,
                                      0., 1., 0.,
                                      0., 0., 1.)
                    * affine;
            }

            if self.will_flip_y() {
                affine = Matrix3::new(1., 0., 0.,
                                      0., -1., 0.,
                                      0., 0., 1.);
            }

            if !self.is_center() {
                // xy offset
                affine = Matrix3::new(1., 0., 1.,
                                      0., 1., 1.,
                                      0., 0., 1.)
                    * affine;
                
                // scale half
                affine = Matrix3::new(0.5, 0., 0.,
                                      0., 0.5, 0.,
                                      0., 0., 1.)
                    * affine
            }

            affine
        }
    }

    pub fn to_center_matrix(&self) -> Matrix3<f64> {
        self.from_center_matrix
            .try_inverse()
            .unwrap()
    }
}
