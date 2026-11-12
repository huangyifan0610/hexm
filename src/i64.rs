//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct I64Hex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: i64,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: i64,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<i64>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`I64Hex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = i64hex(314159, -271828);
/// assert_eq!(h.q, 314159);
/// assert_eq!(h.r, -271828);
/// ```
#[inline(always)]
#[must_use]
pub const fn i64hex(q: i64, r: i64) -> I64Hex {
    I64Hex {
        q,
        r,
        s: PhantomData,
    }
}
