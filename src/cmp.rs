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
macro_rules! impl_partial_eq_for_hex {
    ($Self:ty) => {
        impl core::cmp::PartialEq for $Self {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                self.q == other.q && self.r == other.r
            }
        }

        impl core::cmp::PartialEq<&Self> for $Self {
            #[inline(always)]
            fn eq(&self, other: &&Self) -> bool {
                *self == **other
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_partial_eq_for_hex!(Hex);
#[cfg(feature = "f64")]
impl_partial_eq_for_hex!(DHex);
#[cfg(feature = "i8")]
impl_partial_eq_for_hex!(I8Hex);
#[cfg(feature = "i16")]
impl_partial_eq_for_hex!(I16Hex);
#[cfg(feature = "i32")]
impl_partial_eq_for_hex!(IHex);
#[cfg(feature = "i64")]
impl_partial_eq_for_hex!(I64Hex);
#[cfg(feature = "isize")]
impl_partial_eq_for_hex!(ISizeHex);

#[cfg(any(
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_eq_for_integral_hex {
    ($Self:ty) => {
        impl core::cmp::Eq for $Self {}
    };
}

#[cfg(feature = "i8")]
impl_eq_for_integral_hex!(I8Hex);
#[cfg(feature = "i16")]
impl_eq_for_integral_hex!(I16Hex);
#[cfg(feature = "i32")]
impl_eq_for_integral_hex!(IHex);
#[cfg(feature = "i64")]
impl_eq_for_integral_hex!(I64Hex);
#[cfg(feature = "isize")]
impl_eq_for_integral_hex!(ISizeHex);
