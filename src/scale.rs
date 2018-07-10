use na::Matrix3;

use unit_length::UnitLength;

////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Scale {
    Square(UnitLength),
    Scale,
}

use self::Scale::*;

impl Scale {
    pub fn is_square(&self) -> bool {
        match self {
            Square(_) => true,
            _ => false,
        }
    }

    pub fn is_scale(&self) -> bool {
        *self == Scale
    }

    pub fn output_matrix(
        &self,
        target_dimensions: &(f64, f64),
    ) -> Matrix3<f64> {
        use UnitLength::*;

        let unit_length = match self {
            Scale => return Matrix3::new(target_dimensions.0, 0., 0.,
                                         0., target_dimensions.1, 0.,
                                         0., 0., 1.),

            Square(Width) => target_dimensions.0,
            Square(Height) => target_dimensions.1,
            Square(MaxSide) => target_dimensions.0.max(target_dimensions.1),
            Square(MinSide) => target_dimensions.0.min(target_dimensions.1),
        };

        Matrix3::new(unit_length, 0., 0.,
                     0., unit_length, 0.,
                     0., 0., 1.)
    }
}
