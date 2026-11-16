//! An axial-coordinate vector.

use core::marker::PhantomData;

use glam::DVec2;

use crate::orientation::Orientation;

/// An axial-coordinate vector.
#[repr(C)]
pub struct DHex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: f64,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: f64,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<f64>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`DHex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = dhex(3.14159, -2.71828);
/// assert_eq!(h.q, 3.14159);
/// assert_eq!(h.r, -2.71828);
/// ```
#[inline(always)]
#[must_use]
pub const fn dhex(q: f64, r: f64) -> DHex {
    DHex {
        q,
        r,
        s: PhantomData,
    }
}

impl DHex {
    /// Transforms from a cartesian-coordinate vector into an axial-coordinate
    /// vector.
    ///
    /// See also [`Self::into_world_position`] for an inverse function.
    #[must_use]
    pub fn from_world_position<O: Orientation>(world_position: DVec2) -> Self {
        DHex::from_dvec2(<O as Orientation>::FROM_WORLD_POSITION_F64 * world_position)
    }

    /// Transforms from an axial-coordinate vector into a cartesian-coordinate
    /// vector.
    ///
    /// See also [`Self::from_world_position`] for an inverse function.
    #[must_use]
    pub fn into_world_position<O: Orientation>(self) -> DVec2 {
        <O as Orientation>::INTO_WORLD_POSITION_F64 * self.into_dvec2()
    }
}
