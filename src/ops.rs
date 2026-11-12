#[cfg(feature = "f32")]
use crate::f32::{Hex, hex};
#[cfg(feature = "f64")]
use crate::f64::{DHex, dhex};
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
macro_rules! impl_ops_for_hex {
    ($Self:ty, $T:ty) => {
        impl core::ops::Neg for $Self {
            type Output = Self;

            #[inline(always)]
            fn neg(self) -> Self {
                self.opposite()
            }
        }

        impl core::ops::Neg for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn neg(self) -> $Self {
                self.opposite()
            }
        }

        impl core::ops::Add for $Self {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                self.offset(rhs)
            }
        }

        impl core::ops::Add for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> $Self {
                self.offset(*rhs)
            }
        }

        impl core::ops::Add<&Self> for $Self {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: &Self) -> Self {
                self.offset(*rhs)
            }
        }

        impl core::ops::Add<$Self> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn add(self, rhs: $Self) -> $Self {
                self.offset(rhs)
            }
        }

        impl core::ops::AddAssign for $Self {
            #[inline(always)]
            fn add_assign(&mut self, rhs: Self) {
                self.q += rhs.q;
                self.r += rhs.r;
            }
        }

        impl core::ops::AddAssign<&Self> for $Self {
            #[inline(always)]
            fn add_assign(&mut self, rhs: &Self) {
                *self += *rhs;
            }
        }

        impl core::ops::Sub for $Self {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                self.offset_from(rhs)
            }
        }

        impl core::ops::Sub for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> $Self {
                self.offset_from(*rhs)
            }
        }

        impl core::ops::Sub<&Self> for $Self {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: &Self) -> Self {
                self.offset_from(*rhs)
            }
        }

        impl core::ops::Sub<$Self> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn sub(self, rhs: $Self) -> $Self {
                self.offset_from(rhs)
            }
        }

        impl core::ops::SubAssign for $Self {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: Self) {
                self.q -= rhs.q;
                self.r -= rhs.r;
            }
        }

        impl core::ops::SubAssign<&Self> for $Self {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: &Self) {
                *self -= *rhs;
            }
        }

        impl core::ops::Mul<$T> for $Self {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: $T) -> Self {
                self.scale(rhs)
            }
        }

        impl core::ops::Mul<$T> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn mul(self, rhs: $T) -> $Self {
                self.scale(rhs)
            }
        }

        impl core::ops::Mul<&$T> for $Self {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: &$T) -> Self {
                self.scale(*rhs)
            }
        }

        impl core::ops::Mul<&$T> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn mul(self, rhs: &$T) -> $Self {
                self.scale(*rhs)
            }
        }

        impl core::ops::MulAssign<$T> for $Self {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: $T) {
                self.q *= rhs;
                self.r *= rhs;
            }
        }

        impl core::ops::MulAssign<&$T> for $Self {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: &$T) {
                *self *= *rhs;
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_ops_for_hex!(Hex, f32);
#[cfg(feature = "f64")]
impl_ops_for_hex!(DHex, f64);
#[cfg(feature = "i8")]
impl_ops_for_hex!(I8Hex, i8);
#[cfg(feature = "i16")]
impl_ops_for_hex!(I16Hex, i16);
#[cfg(feature = "i32")]
impl_ops_for_hex!(IHex, i32);
#[cfg(feature = "i64")]
impl_ops_for_hex!(I64Hex, i64);
#[cfg(feature = "isize")]
impl_ops_for_hex!(ISizeHex, isize);

#[cfg(any(feature = "f32", feature = "f64",))]
macro_rules! impl_ops_for_floating_point_hex {
    ($Self:ty, $T:ty, $new:path) => {
        impl core::ops::Div<$T> for $Self {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: $T) -> Self {
                $new(self.q / rhs, self.r / rhs)
            }
        }

        impl core::ops::Div<$T> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn div(self, rhs: $T) -> $Self {
                *self / rhs
            }
        }

        impl core::ops::Div<&$T> for $Self {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: &$T) -> Self {
                self / *rhs
            }
        }

        impl core::ops::Div<&$T> for &$Self {
            type Output = $Self;

            #[inline(always)]
            fn div(self, rhs: &$T) -> $Self {
                *self / *rhs
            }
        }

        impl core::ops::DivAssign<$T> for $Self {
            #[inline(always)]
            fn div_assign(&mut self, rhs: $T) {
                self.q /= rhs;
                self.r /= rhs;
            }
        }

        impl core::ops::DivAssign<&$T> for $Self {
            #[inline(always)]
            fn div_assign(&mut self, rhs: &$T) {
                *self /= *rhs;
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_ops_for_floating_point_hex!(Hex, f32, hex);
#[cfg(feature = "f64")]
impl_ops_for_floating_point_hex!(DHex, f64, dhex);
