//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct I16Hex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: i16,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: i16,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<i16>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`I16Hex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = i16hex(31415, -27182);
/// assert_eq!(h.q, 31415);
/// assert_eq!(h.r, -27182);
/// ```
#[inline(always)]
#[must_use]
pub const fn i16hex(q: i16, r: i16) -> I16Hex {
    I16Hex {
        q,
        r,
        s: PhantomData,
    }
}
