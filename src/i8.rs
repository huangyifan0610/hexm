//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct I8Hex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: i8,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: i8,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<i8>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`I8Hex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = i8hex(31, -27);
/// assert_eq!(h.q, 31);
/// assert_eq!(h.r, -27);
/// ```
#[inline(always)]
#[must_use]
pub const fn i8hex(q: i8, r: i8) -> I8Hex {
    I8Hex {
        q,
        r,
        s: PhantomData,
    }
}
