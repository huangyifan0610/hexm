//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct IHex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: i32,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: i32,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<i32>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`IHex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = ihex(314159, -271828);
/// assert_eq!(h.q, 314159);
/// assert_eq!(h.r, -271828);
/// ```
#[inline(always)]
#[must_use]
pub const fn ihex(q: i32, r: i32) -> IHex {
    IHex {
        q,
        r,
        s: PhantomData,
    }
}
