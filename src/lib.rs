extern crate nalgebra;

use nalgebra as na;

use na::Matrix2;
use na::Vector2;

////////////////////////////////////////////////////////////////////////////////

pub enum CoordinateStyle {
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
    pub style: CoordinateStyle,
    pub input_dominance: CoordinateDominance,
    pub output_dominance: CoordinateDominance,
}

impl CoordinateSystem {
    pub fn io_transform_matrix(
        &self,
        size: &(f64, f64),
    ) -> Matrix2<f64> {
        // okay, don't worry about the manual mathematics of these. hopefully,
        // the compiler would be smart enough to optimize this whole function.
        
        // set everything to square
        let transform_to_square = match self.scale {
            Square => Matrix2::identity(),
            Scale => match (self.input_dominance, size.0, size.1) {
                (WidthNormalized, x, y) => Matrix2::new(1., 0.,
                                                        0., y / x),
                (MaxNormalized, x, y) if x > y => Matrix2::new(1., 0.,
                                                               0., y / x),
                (MinNormalized, x, y) if x < y => Matrix2::new(1., 0.,
                                                               0., y / x),

                // too lazy to do the other condition
                | (_, x, y) => Matrix2::new(x / y, 0.,
                                            0.   , 1.),
            }
        };

        fn flip_y(style: &CoordinateStyle) -> bool {
            use CoordinateStyle::*;

            match style {
                TopLeft | TopRight => true,
                _ => false,
            }
        }

        fn flip_x(style: &CoordinateStyle) -> bool {
            use CoordinateStyle::*;

            match style {
                TopRight | BottomRight => true,
                _ => false,
            }
        };

        fn scale_half(style: &CoordinateStyle) -> bool {
            use CoordinateStyle::*;

            match style {
                Center => false,
                _ => true,
            }
        }

        fn x_offset(style: &CoordinateStyle) -> Option<bool> {
            use CoordinateStyle::*;

            match style {
                TopLeft | BottomLeft => Some(true),
                BottomLeft | BottomRight => Some(false),
                Center => None,
            }
        }

        fn y_offset(style: &CoordinateStyle) -> Option<bool> {
            use CoordinateStyle::*;

            match style {
                TopLeft | TopRight => Some(false),
                BottomLeft | BottomRight => Some(true),
                Center => None,
            }
        }

        fn transform_to_center(style: &CoordinateStyle) -> Matrix2<f64>
        {
            let mut indom_to_center = Matrix2::identity();

            if flip_y(style) {
                indom_to_center *= Matrix2::new(1., 0.,
                                                0., -1.);
            }

            if flip_x(style) {
                indom_to_center *= Matrix2::new(-1., 0.,
                                                0., 1.);
            }

            if scale_half(style) {
                indom_to_center *= Matrix2::new(0.5, 0.,
                                                0., 0.5);
            }

            if let Some(x_offset) = x_offset(style) {
                let x_offset = if x_offset { 1. } else { -1. };

                indom_to_center *= Matrix2::new(x_offset, 0.,
                                                0., 0.);
            }

            if let Some(y_offset) = y_offset(style) {
                let y_offset = if y_offset { 1. } else { -1. };

                indom_to_center *= Matrix2::new(0., 0.,
                                                0., y_offset);
            }

            indom_to_center
        };

        let center_to_transform = transform_to_center(&self.style)
            .try_inverse()
            .unwrap();
        let transform_to_center = transform_to_center(&self.style);

        transform_to_square * transform_to_center * center_to_transform

    }

    pub fn into_y(
        &self,
        input: (f64, f64),
        size: (f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self.io_transform_matrix(&size);
        let vector = transform_matrix * Vector2::new(input.0, input.1);

        (vector[(0, 0)], vector[(0, 1)])
    }

    pub fn into_x(
        &self,
        output: (f64, f64),
        size: (f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self
            .io_transform_matrix(&size)
            .try_inverse()
            .unwrap();

        let vector = transform_matrix * Vector2::new(output.0, output.1);

        (vector[(0, 0)], vector[(0, 1)])
    }
}