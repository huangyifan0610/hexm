#[cfg(feature = "f32")]
use crate::f32::Hex;
#[cfg(feature = "f64")]
use crate::f64::DHex;
#[cfg(feature = "i8")]
use crate::i8::I8Hex;
#[cfg(feature = "i16")]
use crate::i16::I16Hex;
#[cfg(feature = "i32")]
use crate::i32::IHex;
#[cfg(feature = "i64")]
use crate::i64::I64Hex;
#[cfg(feature = "isize")]
use crate::isize::ISizeHex;

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_copy_for_hex {
    ($Self:ty) => {
        impl core::marker::Copy for $Self {}
    };
}

#[cfg(feature = "f32")]
impl_copy_for_hex!(Hex);
#[cfg(feature = "f64")]
impl_copy_for_hex!(DHex);
#[cfg(feature = "i8")]
impl_copy_for_hex!(I8Hex);
#[cfg(feature = "i16")]
impl_copy_for_hex!(I16Hex);
#[cfg(feature = "i32")]
impl_copy_for_hex!(IHex);
#[cfg(feature = "i64")]
impl_copy_for_hex!(I64Hex);
#[cfg(feature = "isize")]
impl_copy_for_hex!(ISizeHex);
