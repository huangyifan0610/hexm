//! Conversion and transformation between axial-coordinate vectors and
//! 2-dimensional vectors in [`glam`].

#[cfg(feature = "f64")]
use glam::{DMat2, DVec2, dmat2, dvec2};
#[cfg(feature = "i8")]
use glam::{I8Vec2, i8vec2};
#[cfg(feature = "i16")]
use glam::{I16Vec2, i16vec2};
#[cfg(feature = "i64")]
use glam::{I64Vec2, i64vec2};
#[cfg(feature = "isize")]
use glam::{ISizeVec2, isizevec2};
#[cfg(feature = "i32")]
use glam::{IVec2, ivec2};
#[cfg(feature = "f32")]
use glam::{Mat2, Vec2, mat2, vec2};

#[cfg(feature = "f32")]
use crate::f32::{Hex, hex};
#[cfg(feature = "f64")]
use crate::f64::{DHex, dhex};
#[cfg(feature = "i8")]
use crate::i8::{I8Hex, i8hex};
#[cfg(feature = "i16")]
use crate::i16::{I16Hex, i16hex};
#[cfg(feature = "i32")]
use crate::i32::{IHex, ihex};
#[cfg(feature = "i64")]
use crate::i64::{I64Hex, i64hex};
#[cfg(feature = "isize")]
use crate::isize::{ISizeHex, isizehex};

#[cfg(feature = "f32")]
impl Hex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_vec2(vector: Vec2) -> Self {
        hex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_vec2(self) -> Vec2 {
        vec2(self.q, self.r)
    }

    /// Transforms from a cartesian-coordinate vector into an axial-coordinate
    /// vector in pointy-top orientation.
    ///
    /// See also [`Self::into_world_position_pointy`] for an inverse function.
    #[must_use]
    pub fn from_world_position_pointy(world_position: Vec2) -> Self {
        const FRAC_1_SQRT_3: f32 = 0.577350269189625764509148780501957456_f32;
        const FROM_WORLD_POSITION_POINTY: Mat2 =
            mat2(vec2(FRAC_1_SQRT_3, 0.), vec2(-1. / 3., 2. / 3.));
        Self::from_vec2(FROM_WORLD_POSITION_POINTY * world_position)
    }

    /// Transforms from an axial-coordinate vector in pointy-top orientation
    /// into a cartesian-coordinate vector.
    ///
    /// See also [`Self::from_world_position_pointy`] for an inverse function.
    #[must_use]
    pub fn into_world_position_pointy(self) -> Vec2 {
        const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
        const INTO_WORLD_POSITION_POINTY: Mat2 = mat2(vec2(SQRT_3, 0.), vec2(SQRT_3 / 2., 3. / 2.));
        INTO_WORLD_POSITION_POINTY * self.into_vec2()
    }

    /// Transforms from a cartesian-coordinate vector into an axial-coordinate
    /// vector in flat-top orientation.
    ///
    /// See also [`Self::into_world_position_flat`] for an inverse function.
    #[must_use]
    pub fn from_world_position_flat(world_position: Vec2) -> Self {
        const FRAC_1_SQRT_3: f32 = 0.577350269189625764509148780501957456_f32;
        const FROM_WORLD_POSITION_FLAT: Mat2 =
            mat2(vec2(2. / 3., -1. / 3.), vec2(0., FRAC_1_SQRT_3));
        Self::from_vec2(FROM_WORLD_POSITION_FLAT * world_position)
    }

    /// Transforms from an axial-coordinate vector in flat-top orientation
    /// into a cartesian-coordinate vector.
    ///
    /// See also [`Self::from_world_position_flat`] for an inverse function.
    #[must_use]
    pub fn into_world_position_flat(self) -> Vec2 {
        const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;
        const INTO_WORLD_POSITION_FLAT: Mat2 = mat2(vec2(3. / 2., SQRT_3 / 2.), vec2(0.0, SQRT_3));
        INTO_WORLD_POSITION_FLAT * self.into_vec2()
    }
}

#[cfg(feature = "f64")]
impl DHex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_dvec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_dvec2(vector: DVec2) -> Self {
        dhex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_dvec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_dvec2(self) -> DVec2 {
        dvec2(self.q, self.r)
    }

    /// Transforms from a cartesian-coordinate vector into an axial-coordinate
    /// vector in pointy-top orientation.
    ///
    /// See also [`Self::into_world_position_pointy`] for an inverse function.
    #[must_use]
    pub fn from_world_position_pointy(world_position: DVec2) -> Self {
        const FRAC_1_SQRT_3: f64 = 0.577350269189625764509148780501957456_f64;
        const FROM_WORLD_POSITION_POINTY: DMat2 =
            dmat2(dvec2(FRAC_1_SQRT_3, 0.), dvec2(-1. / 3., 2. / 3.));
        Self::from_dvec2(FROM_WORLD_POSITION_POINTY * world_position)
    }

    /// Transforms from an axial-coordinate vector in pointy-top orientation
    /// into a cartesian-coordinate vector.
    ///
    /// See also [`Self::from_world_position_pointy`] for an inverse function.
    #[must_use]
    pub fn into_world_position_pointy(self) -> DVec2 {
        const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;
        const INTO_WORLD_POSITION_POINTY: DMat2 =
            dmat2(dvec2(SQRT_3, 0.), dvec2(SQRT_3 / 2., 3. / 2.));
        INTO_WORLD_POSITION_POINTY * self.into_dvec2()
    }

    /// Transforms from a cartesian-coordinate vector into an axial-coordinate
    /// vector in flat-top orientation.
    ///
    /// See also [`Self::into_world_position_flat`] for an inverse function.
    #[must_use]
    pub fn from_world_position_flat(world_position: DVec2) -> Self {
        const FRAC_1_SQRT_3: f64 = 0.577350269189625764509148780501957456_f64;
        const FROM_WORLD_POSITION_FLAT: DMat2 =
            dmat2(dvec2(2. / 3., -1. / 3.), dvec2(0., FRAC_1_SQRT_3));
        Self::from_dvec2(FROM_WORLD_POSITION_FLAT * world_position)
    }

    /// Transforms from an axial-coordinate vector in flat-top orientation
    /// into a cartesian-coordinate vector.
    ///
    /// See also [`Self::from_world_position_flat`] for an inverse function.
    #[must_use]
    pub fn into_world_position_flat(self) -> DVec2 {
        const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;
        const INTO_WORLD_POSITION_FLAT: DMat2 =
            dmat2(dvec2(3. / 2., SQRT_3 / 2.), dvec2(0.0, SQRT_3));
        INTO_WORLD_POSITION_FLAT * self.into_dvec2()
    }
}

#[cfg(feature = "i8")]
impl I8Hex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_i8vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_i8vec2(vector: I8Vec2) -> Self {
        i8hex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_i8vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_i8vec2(self) -> I8Vec2 {
        i8vec2(self.q, self.r)
    }
}

#[cfg(feature = "i16")]
impl I16Hex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_i16vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_i16vec2(vector: I16Vec2) -> Self {
        i16hex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_i16vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_i16vec2(self) -> I16Vec2 {
        i16vec2(self.q, self.r)
    }
}

#[cfg(feature = "i32")]
impl IHex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_ivec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_ivec2(vector: IVec2) -> Self {
        ihex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_ivec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_ivec2(self) -> IVec2 {
        ivec2(self.q, self.r)
    }
}

#[cfg(feature = "i64")]
impl I64Hex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_i64vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_i64vec2(vector: I64Vec2) -> Self {
        i64hex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_i64vec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_i64vec2(self) -> I64Vec2 {
        i64vec2(self.q, self.r)
    }
}

#[cfg(feature = "isize")]
impl ISizeHex {
    /// Converts the 2-dimensional vector into an axial-coordinate vector.
    ///
    /// The *X* and *Y* components of the input vector will become the *Q* and
    /// *R* components of the result vector respectively.
    ///
    /// See also [`Self::into_isizevec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn from_isizevec2(vector: ISizeVec2) -> Self {
        isizehex(vector.x, vector.y)
    }

    /// Converts the axial-coordinate vector into a [2-dimensional vector].
    ///
    /// The *Q* and *R* components of the `self` vector will become the *X* and
    /// *Y* components of the result vector respectively.
    ///
    /// See also [`Self::from_isizevec2`] for an inverse function.
    #[inline(always)]
    #[must_use]
    pub const fn into_isizevec2(self) -> ISizeVec2 {
        isizevec2(self.q, self.r)
    }
}
