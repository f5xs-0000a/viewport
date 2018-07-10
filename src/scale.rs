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

        let unit_length = match &self.scale {
            Scale => return Matrix3::new(width, 0., 0.,
                                         0., height, 0.,
                                         0., 0., 1.),

            Square(Width) => target_width,
            Square(Height) => target_height,
            Square(MaxSide) => target_width.max(target_height),
            Square(MinSide) => target_width.min(target_height),
        };

        Matrix3::new(unit_length, 0., 0.,
                     0., unit_length, 0.,
                     0., 0., 1.)
    }
}
