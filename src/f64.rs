//! An axial-coordinate vector.

use core::marker::PhantomData;

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
