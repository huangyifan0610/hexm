//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct Hex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: f32,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: f32,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<f32>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`Hex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = hex(3.141, -2.718);
/// assert_eq!(h.q, 3.141);
/// assert_eq!(h.r, -2.718);
/// ```
#[inline(always)]
#[must_use]
pub const fn hex(q: f32, r: f32) -> Hex {
    Hex {
        q,
        r,
        s: PhantomData,
    }
}
