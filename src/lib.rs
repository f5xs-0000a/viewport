extern crate nalgebra;

use nalgebra as na;

use na::Matrix3;
use na::Matrix2;
use na::Vector2;
use na::Vector3;

////////////////////////////////////////////////////////////////////////////////

pub enum CoordinateOrigin {
    /// zero at the center, positive towards top-right
    Center,

    /// zero at the top left, positive towards bottom-right
    TopLeft,

    /// zero at the top right, positive towards bottom-left
    TopRight,

    /// zero at the bottom left, positive towards top-right
    BottomLeft,

    /// zero at the bottom right, positive towards top-left
    BottomRight,
}

pub enum CoordinateScale {
    /// unit interval in the x-axis is equidistant to the unit interval in the
    /// y-axis, thus a rectangular viewport may not have a complete unit
    /// distance on one side
    Square,

    /// ratio of unit interval in the x-axis to the unit interval in the y-axis
    /// is the same as the ratio of viewport width to viewport height, thus a
    /// rectangular viewport will have a complete unit distance on either side
    Scale,
}

pub enum CoordinateDominance {
    /// the width of the viewport will have a unit distance
    WidthNormalized,

    /// the height of the viewport will have a unit distance
    HeightNormalized,

    /// the side that has the greater length will have the unit distance
    MaxNormalized,

    /// the side that has the lesser length will have the unit distance
    MinNormalized,
}

pub struct CoordinateSystem {
    pub scale: CoordinateScale,
    pub dominance: CoordinateDominance,
    pub input_origin: CoordinateOrigin,
    pub output_origin: CoordinateOrigin,
}

impl CoordinateSystem {
    pub fn io_transform_matrix(
        &self,
        size: &(f64, f64),
    ) -> Matrix3<f64> {
        use CoordinateDominance::*;
        use CoordinateScale::*;

        // okay, don't worry about the manual mathematics of these. hopefully,
        // the compiler would be smart enough to optimize this whole function.
        
        // set everything to square
        let transform_to_square = match self.scale {
            Square => Matrix3::identity(),
            Scale => match (&self.dominance, size.0, size.1) {
                (WidthNormalized, x, y) => Matrix3::new(1., 0., 0.,
                                                        0., y / x, 0.,
                                                        0., 0., 1.),
                (MaxNormalized, x, y) if x > y => Matrix3::new(1., 0., 0.,
                                                               0., y / x, 0.,
                                                               0., 0., 1.),
                (MinNormalized, x, y) if x < y => Matrix3::new(1., 0., 0.,
                                                               0., y / x, 0.,
                                                               0., 0., 1.),

                // too lazy to do the other condition
                | (_, x, y) => Matrix3::new(x / y, 0., 0.,
                                            0.   , 1., 0.,
                                            0., 0., 1.),
            }
        };

        fn flip_y(origin: &CoordinateOrigin) -> bool {
            use CoordinateOrigin::*;

            match origin {
                TopLeft | TopRight => true,
                _ => false,
            }
        }

        fn flip_x(origin: &CoordinateOrigin) -> bool {
            use CoordinateOrigin::*;

            match origin {
                TopRight | BottomRight => true,
                _ => false,
            }
        };

        fn scale_half(origin: &CoordinateOrigin) -> bool {
            use CoordinateOrigin::*;

            match origin {
                Center => false,
                _ => true,
            }
        }

        fn x_offset(origin: &CoordinateOrigin) -> Option<bool> {
            use CoordinateOrigin::*;

            match origin {
                TopLeft | BottomLeft => Some(true),
                TopRight | BottomRight => Some(false),
                Center => None,
            }
        }

        fn y_offset(origin: &CoordinateOrigin) -> Option<bool> {
            use CoordinateOrigin::*;

            match origin {
                TopLeft | TopRight => Some(false),
                BottomLeft | BottomRight => Some(true),
                Center => None,
            }
        }

        fn transform_to_center(origin: &CoordinateOrigin) -> Matrix3<f64> {
            let mut affine = Matrix3::identity();

            if flip_y(origin) {
                affine = Matrix3::new(1., 0., 0.,
                                      0., -1., 0.,
                                      0., 0., 1.)
                    * affine;
            }

            if flip_x(origin) {
                affine = Matrix3::new(-1., 0., 0.,
                                      0., 1., 0.,
                                      0., 0., 1.)
                    * affine;
            }

            if scale_half(origin) {
                affine = Matrix3::new(0.5, 0., 0.,
                                      0., 0.5, 0.,
                                      0., 0., 1.)
                    * affine;
            }

            if let Some(x_offset) = x_offset(origin) {
                let x_offset = if x_offset { 0.5 } else { -0.5 };

                affine = Matrix3::new(1., 0., x_offset,
                                      0., 1., 0.,
                                      0., 0., 1.)
                    * affine;
            }

            if let Some(y_offset) = y_offset(origin) {
                let y_offset = if y_offset { 0.5 } else { -0.5 };

                affine = Matrix3::new(1., 0., 0.,
                                      0., 1., y_offset,
                                      0., 0., 1.)
                    * affine;
            }

            affine
        };

        let center_to_transform = transform_to_center(&self.output_origin)
            .try_inverse()
            .unwrap();
        let transform_to_center = transform_to_center(&self.input_origin);

        let dominant_length = match &self.dominance {
            WidthDominant => size.0,
            HeightDominant => size.1,
            MaxDominant => {
                if size.0 > size.1 {
                    size.0
                }

                else {
                    size.1
                }
            },

            MinDominant => {
                if size.0 < size.1 {
                    size.0
                }

                else {
                    size.1
                }
            },
        };

        Matrix3::new(dominant_length / 2., 0., 0.,
                     0., dominant_length / 2., 0.,
                     0., 0., 1.)
            * transform_to_square
            * transform_to_center
            * center_to_transform

    }

    pub fn into_y(
        &self,
        input: (f64, f64),
        size: &(f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self.io_transform_matrix(size);
        let vector = transform_matrix * Vector3::new(input.0, input.1, 0.);

        (vector[(0, 0)], vector[(1, 0)])
    }

    pub fn into_x(
        &self,
        output: (f64, f64),
        size: &(f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self
            .io_transform_matrix(size)
            .try_inverse()
            .unwrap();

        let vector = transform_matrix * Vector3::new(output.0, output.1, 0.);

        (vector[(0, 0)], vector[(1, 0)])
    }
}
