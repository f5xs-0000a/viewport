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

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone)]
pub struct ScaleScope {
    pub scale: Scale,
    pub scope: Scope,
}

impl ScaleScope {
    pub fn output_matrix(&self, size: &(f64, f64)) -> Matrix3<f64> {
        let output_matrix = ;

        let effective_rendering_size = size,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct CoordinateSystem {
    pub scale: Scale,
    pub scope: Scope,
    pub input_origin: Origin,
    pub output_origin: Origin,
}

impl CoordinateSystem {
    pub fn io_transform_matrix(
        &self,
        size: &(f64, f64),
    ) -> Matrix3<f64> {
        Matrix3::identity()

            // move the center origin to desired output origin
            * self.output_origin.from_center_matrix()
            
            // offset the origin based on the direction of alignment on aspect
            // ratio scope
            
            // move the input origin to center
            * self.input_origin.to_center_matrix()
        unimplemented!()
    }
}
