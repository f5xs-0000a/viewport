extern crate nalgebra as na;

////////////////////////////////////////////////////////////////////////////////

mod scope;
mod direction;
mod scale;
mod unit_length;
mod origin;

pub use self::scope::Scope;
pub use self::direction::Direction;
pub use self::scale::Scale;
pub use self::unit_length::UnitLength;
pub use self::origin::Origin;

use na::Matrix3;
use na::Vector3;

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone)]
pub struct ScaleScope {
    pub scale: Scale,
    pub scope: Scope,
}

impl ScaleScope {
    pub fn output_matrix(&self, size: &(f64, f64)) -> Matrix3<f64> {
        self.scale.output_matrix(
            &self.scope.effective_rendering_size(size)
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CoordinateSystem {
    pub scale_scope: ScaleScope,
    pub input_origin: Origin,
    pub output_origin: Origin,
}

impl CoordinateSystem {
    pub fn io_transform_matrix(
        &self,
        size: &(f64, f64),
    ) -> Matrix3<f64> {
        Matrix3::<f64>::identity()
            // multiply by the size of the effective rendering area
            * self.scale_scope.output_matrix(size)

            // move the center origin to desired output origin
            * self.output_origin.from_center_matrix()
            
            // offset the origin based on the direction of alignment on aspect
            // ratio scope
            
            
            // move the input origin to center
            * self.input_origin.to_center_matrix()
    }

    pub fn into_output_coordinates(
        &self,
        input: (f64, f64),
        size: &(f64, f64),
    ) -> (f64, f64) {
        let transform_matrix = self.io_transform_matrix(size);
        let vector = transform_matrix * Vector3::new(input.0, input.1, 1.);

        (vector[0], vector[1])
    }

    pub fn into_input_coordinates(
        &self,
        output: (f64, f64),
        size: &(f64, f64)
    ) -> (f64, f64) {
        let transform_matrix = self
            .io_transform_matrix(size)
            .try_inverse()
            .unwrap();

        let vector = transform_matrix * Vector3::new(output.0, output.1, 1.);

        (vector[0], vector[1])
    }
}
