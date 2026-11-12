//! An axial-coordinate vector.

use core::marker::PhantomData;

/// An axial-coordinate vector.
#[repr(C)]
pub struct ISizeHex {
    /// The *Q* component.
    #[doc(alias = "x")]
    pub q: isize,
    /// The *R* component.
    #[doc(alias = "y")]
    pub r: isize,
    /// The *S* component which is derived from `q + r + s === 0`.
    #[doc(alias = "z")]
    pub s: PhantomData<isize>,
}

/// Crates an axial-coordinate vector.
///
/// The `q` and `r` parameters represent the *Q* and *R* components of the
/// result vector respectively.
///
/// See also [`ISizeHex::new`] for an equivalent function.
///
/// # Examples
///
/// ```
/// # use hexm::prelude::*;
/// let h = isizehex(314159, -271828);
/// assert_eq!(h.q, 314159);
/// assert_eq!(h.r, -271828);
/// ```
#[inline(always)]
#[must_use]
pub const fn isizehex(q: isize, r: isize) -> ISizeHex {
    ISizeHex {
        q,
        r,
        s: PhantomData,
    }
}
