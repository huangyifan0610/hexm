//! TODO: module-level documentation

#[cfg(feature = "f64")]
use glam::{DMat2, dmat2, dvec2};
#[cfg(feature = "f32")]
use glam::{Mat2, mat2, vec2};

// TODO: Replace `SQRT_3_F32` with `f32::consts::SQRT_3` once stabilized.
#[cfg(feature = "f32")]
const SQRT_3_F32: f32 = 1.732050807568877293527446341505872367_f32;

// TODO: Replace `SQRT_3_F64` with `f64::consts::SQRT_3` once stabilized.
#[cfg(feature = "f64")]
const SQRT_3_F64: f64 = 1.732050807568877293527446341505872367_f64;

// TODO: Replace `FRAC_1_SQRT_3_F32` with `f32::consts::FRAC_1_SQRT_3` once stabilized.
#[cfg(feature = "f32")]
const FRAC_1_SQRT_3_F32: f32 = 0.577350269189625764509148780501957456_f32;

// TODO: Replace `FRAC_1_SQRT_3_F64` with `f64::consts::FRAC_1_SQRT_3` once stabilized.
#[cfg(feature = "f64")]
const FRAC_1_SQRT_3_F64: f64 = 0.577350269189625764509148780501957456_f64;

/// Transformation vectors between axial-coordinate and cartesian-coordinate.
pub trait Orientation {
    /// Transforms vectors from cartesian-coordinate to axial-coordinate.
    #[cfg(feature = "f32")]
    const FROM_WORLD_POSITION_F32: Mat2;

    /// Transforms vectors from cartesian-coordinate to axial-coordinate.
    #[cfg(feature = "f64")]
    const FROM_WORLD_POSITION_F64: DMat2;

    /// Transforms vectors from axial-coordinate to cartesian-coordinate.
    #[cfg(feature = "f32")]
    const INTO_WORLD_POSITION_F32: Mat2;

    /// Transforms vectors from axial-coordinate to cartesian-coordinate.
    #[cfg(feature = "f64")]
    const INTO_WORLD_POSITION_F64: DMat2;
}

/// The pointy-top orientation.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Pointy;

impl Orientation for Pointy {
    #[cfg(feature = "f32")]
    const FROM_WORLD_POSITION_F32: Mat2 =
        mat2(vec2(FRAC_1_SQRT_3_F32, 0.), vec2(-1. / 3., 2. / 3.));

    #[cfg(feature = "f64")]
    const FROM_WORLD_POSITION_F64: DMat2 =
        dmat2(dvec2(FRAC_1_SQRT_3_F64, 0.), dvec2(-1. / 3., 2. / 3.));

    #[cfg(feature = "f32")]
    const INTO_WORLD_POSITION_F32: Mat2 =
        mat2(vec2(SQRT_3_F32, 0.), vec2(SQRT_3_F32 / 2., 3. / 2.));

    #[cfg(feature = "f64")]
    const INTO_WORLD_POSITION_F64: DMat2 =
        dmat2(dvec2(SQRT_3_F64, 0.), dvec2(SQRT_3_F64 / 2., 3. / 2.));
}

/// The flat-top orientation.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Flat;

impl Orientation for Flat {
    #[cfg(feature = "f32")]
    const FROM_WORLD_POSITION_F32: Mat2 =
        mat2(vec2(2. / 3., -1. / 3.), vec2(0., FRAC_1_SQRT_3_F32));

    #[cfg(feature = "f64")]
    const FROM_WORLD_POSITION_F64: DMat2 =
        dmat2(dvec2(2. / 3., -1. / 3.), dvec2(0., FRAC_1_SQRT_3_F64));

    #[cfg(feature = "f32")]
    const INTO_WORLD_POSITION_F32: Mat2 =
        mat2(vec2(3. / 2., SQRT_3_F32 / 2.), vec2(0.0, SQRT_3_F32));

    #[cfg(feature = "f64")]
    const INTO_WORLD_POSITION_F64: DMat2 =
        dmat2(dvec2(3. / 2., SQRT_3_F64 / 2.), dvec2(0.0, SQRT_3_F64));
}

#[cfg(test)]
mod tests {

    use crate::orientation::{Flat, Orientation, Pointy};

    #[test]
    #[cfg(feature = "f32")]
    fn test_orientation_f32() {
        use glam::Mat2;

        assert!(Mat2::IDENTITY.abs_diff_eq(
            Pointy::FROM_WORLD_POSITION_F32 * Pointy::INTO_WORLD_POSITION_F32,
            f32::EPSILON
        ));

        assert!(Mat2::IDENTITY.abs_diff_eq(
            Flat::FROM_WORLD_POSITION_F32 * Flat::INTO_WORLD_POSITION_F32,
            f32::EPSILON
        ));
    }

    #[test]
    #[cfg(feature = "f64")]
    fn test_orientation_f64() {
        use glam::DMat2;

        assert!(DMat2::IDENTITY.abs_diff_eq(
            Pointy::FROM_WORLD_POSITION_F64 * Pointy::INTO_WORLD_POSITION_F64,
            f64::EPSILON
        ));

        assert!(DMat2::IDENTITY.abs_diff_eq(
            Flat::FROM_WORLD_POSITION_F64 * Flat::INTO_WORLD_POSITION_F64,
            f64::EPSILON
        ));
    }
}
