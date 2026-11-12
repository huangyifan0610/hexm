//! An axial-coordinate vector library.
//!
//! See also <https://redblobgames.com/grids/hexagons/>.
//!
//! TODO: crate-level documentation

#![cfg_attr(not(feature = "std"), no_std)]

// re-exports `glam`.
pub extern crate glam;

#[cfg(feature = "f32")]
pub mod f32;
#[cfg(feature = "f64")]
pub mod f64;
#[cfg(feature = "i16")]
pub mod i16;
#[cfg(feature = "i32")]
pub mod i32;
#[cfg(feature = "i64")]
pub mod i64;
#[cfg(feature = "i8")]
pub mod i8;
#[cfg(feature = "isize")]
pub mod isize;

/// The [crate] prelude.
pub mod prelude {
    #[cfg(feature = "f32")]
    pub use crate::f32::{Hex, hex};
    #[cfg(feature = "f64")]
    pub use crate::f64::{DHex, dhex};
    #[cfg(feature = "i8")]
    pub use crate::i8::{I8Hex, i8hex};
    #[cfg(feature = "i16")]
    pub use crate::i16::{I16Hex, i16hex};
    #[cfg(feature = "i32")]
    pub use crate::i32::{IHex, ihex};
    #[cfg(feature = "i64")]
    pub use crate::i64::{I64Hex, i64hex};
    #[cfg(feature = "isize")]
    pub use crate::isize::{ISizeHex, isizehex};
}

#[cfg(feature = "approx")]
mod approx;
mod clone;
mod cmp;
mod convert;
mod default;
mod fmt;
mod hash;
mod r#impl;
mod iter;
mod marker;
mod ops;
#[cfg(feature = "serde")]
mod serde;
