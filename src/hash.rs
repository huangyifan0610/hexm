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
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_hash_for_integral_hex {
    ($Self:ty) => {
        impl core::hash::Hash for $Self {
            #[inline(always)]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.q.hash(state);
                self.r.hash(state);
            }
        }
    };
}

#[cfg(feature = "i8")]
impl_hash_for_integral_hex!(I8Hex);
#[cfg(feature = "i16")]
impl_hash_for_integral_hex!(I16Hex);
#[cfg(feature = "i32")]
impl_hash_for_integral_hex!(IHex);
#[cfg(feature = "i64")]
impl_hash_for_integral_hex!(I64Hex);
#[cfg(feature = "isize")]
impl_hash_for_integral_hex!(ISizeHex);
