extern crate nalgebra;

use nalgebra as na;

use na::Matrix3;
use na::Vector3;

////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
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

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
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

#[derive(Eq, PartialEq, Debug, Clone)]
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

#[derive(Eq, PartialEq)]
pub struct CoordinateSystem {
    pub scale: CoordinateScale,
    pub dominance: CoordinateDominance,
    pub input_origin: CoordinateOrigin,
    pub output_origin: CoordinateOrigin,
}

use CoordinateScale as CS;
use CoordinateOrigin as CO;
use CoordinateDominance as CD;

impl CoordinateSystem {
    pub fn io_transform_matrix(
        &self,
        size: &(f64, f64),
    ) -> Matrix3<f64> {
        // okay, don't worry about the manual mathematics of these. hopefully,
        // the compiler would be smart enough to optimize this whole function.
        
        // set everything to square
        let transform_to_square = match self.scale {
            CS::Square => Matrix3::identity(),
            CS::Scale => match (&self.dominance, size.0, size.1) {
                (CD::WidthNormalized, x, y) => Matrix3::new(1., 0., 0.,
                                                            0., y / x, 0.,
                                                            0., 0., 1.),
                (CD::MaxNormalized, x, y) if x > y => Matrix3::new(1., 0., 0.,
                                                                   0., y / x, 0.,
                                                                   0., 0., 1.),
                (CD::MinNormalized, x, y) if x < y => Matrix3::new(1., 0., 0.,
                                                                   0., y / x, 0.,
                                                                   0., 0., 1.),

                // too lazy to do the other condition
                | (_, x, y) => Matrix3::new(x / y, 0., 0.,
                                            0.   , 1., 0.,
                                            0., 0., 1.),
            }
        };

        fn flip_y(origin: &CoordinateOrigin) -> bool {
            match origin {
                CO::TopLeft | CO::TopRight => true,
                _ => false,
            }
        }

        fn flip_x(origin: &CoordinateOrigin) -> bool {
            match origin {
                CO::TopRight | CO::BottomRight => true,
                _ => false,
            }
        };

        // this function returns the transformation matrix that will adjust
        // the input coordinates, given the input origin specified, into center
        //
        // this function can also be used for transforming a center-fixed
        // coordinate into some coordinate given the specified input origin, by
        // inverting the returned matrix
        fn center_to_transform(origin: &CoordinateOrigin) -> Matrix3<f64> {
            let mut affine = Matrix3::identity();
            let not_center = *origin != CO::Center;

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
            
            if not_center {
                // xy offset
                affine = Matrix3::new(1., 0., 1.,
                                      0., 1., 1.,
                                      0., 0., 1.)
                    * affine;
                println!("{:?}", affine);
                    
                // scale half
                affine = Matrix3::new(0.5, 0., 0.,
                                      0., 0.5, 0.,
                                      0., 0., 1.)
                    * affine;
            }
            
            affine
        };

        let transform_to_center = center_to_transform(&self.input_origin)
            .try_inverse()
            .unwrap();
            
        let center_to_transform = center_to_transform(&self.output_origin);

        let dominant_length = match &self.dominance {
            CD::WidthNormalized => size.0,
            CD::HeightNormalized => size.1,
            CD::MaxNormalized => {
                if size.0 > size.1 {
                    size.0
                }

                else {
                    size.1
                }
            },

            CD::MinNormalized => {
                if size.0 < size.1 {
                    size.0
                }

                else {
                    size.1
                }
            },
        };

        Matrix3::new(dominant_length, 0., 0.,
                     0., dominant_length, 0.,
                     0., 0., 1.)
            * center_to_transform
            * transform_to_center
            * transform_to_square
    }

    pub fn into_y(
        &self,
        input: (f64, f64),
        size: &(f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self.io_transform_matrix(size);
        let vector = transform_matrix * Vector3::new(input.0, input.1, 1.);

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
