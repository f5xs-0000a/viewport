use direction::Direction;
use na::Matrix3;

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone)]
pub enum Scope {
    Whole,
    AspectRatio(f64, f64, Direction),
}

use self::Scope::*;

impl Scope {
    pub fn is_whole(&self) -> bool {
        *self == Whole
    }

    pub fn is_aspect_ratio(&self) -> bool {
        match self {
            AspectRatio(_, _, _) => true,
            _ => false,
        }
    }

    pub fn effective_rendering_size(&self, size: &(f64, f64)) -> (f64, f64) {
        match self {
            Whole => size.clone(),
            AspectRatio(ar_x, ar_y, _) => {
                if size.0 / ar_x < size.1 / ar_y {
                    (size.0, ar_y * size.0 / ar_x)
                }

                else {
                    (ar_x * size.1 / ar_y, size.1)
                }
            }
        }
    }

    pub fn effective_unit_offset_matrix(&self, size: &(f64, f64))
    -> Matrix3<f64> {
        let width_limited = size.0 / ar_x < size.1 / ar_y;

        match self {
            AspectRatio(_, _, Center) | Whole => Matrix3::identity(),

            AspectRatio(input_ar_width, input_ar_height, direction) => {
                let input_ar_width = input_ar_width / input_ar_height;
                let input_ar_height = 1.;

                let output_ar_width = size.0 / size.1;
                let output_ar_height = 1.;

                
            },
        }
    }

    pub fn alignment_matrix(&self) -> Matrix3<f64> {

    }
}
