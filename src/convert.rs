#[cfg(feature = "f64")]
use glam::DVec2;
#[cfg(feature = "i8")]
use glam::I8Vec2;
#[cfg(feature = "i16")]
use glam::I16Vec2;
#[cfg(feature = "i64")]
use glam::I64Vec2;
#[cfg(feature = "isize")]
use glam::ISizeVec2;
#[cfg(feature = "i32")]
use glam::IVec2;
#[cfg(feature = "f32")]
use glam::Vec2;

#[cfg(feature = "f32")]
use crate::f32::{Hex, hex};
#[cfg(feature = "f64")]
use crate::f64::{DHex, dhex};
#[cfg(feature = "i8")]
use crate::i8::{I8Hex, i8hex};
#[cfg(feature = "i16")]
use crate::i16::{I16Hex, i16hex};
#[cfg(feature = "i32")]
use crate::i32::{IHex, ihex};
#[cfg(feature = "i64")]
use crate::i64::{I64Hex, i64hex};
#[cfg(feature = "isize")]
use crate::isize::{ISizeHex, isizehex};

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_convert_for_hex {
    ($Self:ty, $T:ty, $new:path) => {
        impl core::convert::From<($T, $T)> for $Self {
            #[inline(always)]
            fn from(value: ($T, $T)) -> Self {
                $new(value.0, value.1)
            }
        }

        impl core::convert::From<$Self> for ($T, $T) {
            #[inline(always)]
            fn from(value: $Self) -> Self {
                (value.q, value.r)
            }
        }

        impl core::convert::From<[$T; 2]> for $Self {
            #[inline(always)]
            fn from(value: [$T; 2]) -> Self {
                <$Self>::from_array(value)
            }
        }

        impl core::convert::From<$Self> for [$T; 2] {
            #[inline(always)]
            fn from(value: $Self) -> Self {
                value.into_array()
            }
        }

        impl core::convert::AsRef<[$T; 2]> for $Self {
            #[inline(always)]
            fn as_ref(&self) -> &[$T; 2] {
                self.as_array()
            }
        }

        impl core::convert::AsMut<[$T; 2]> for $Self {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$T; 2] {
                self.as_mut_array()
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_convert_for_hex!(Hex, f32, hex);
#[cfg(feature = "f64")]
impl_convert_for_hex!(DHex, f64, dhex);
#[cfg(feature = "i8")]
impl_convert_for_hex!(I8Hex, i8, i8hex);
#[cfg(feature = "i16")]
impl_convert_for_hex!(I16Hex, i16, i16hex);
#[cfg(feature = "i32")]
impl_convert_for_hex!(IHex, i32, ihex);
#[cfg(feature = "i64")]
impl_convert_for_hex!(I64Hex, i64, i64hex);
#[cfg(feature = "isize")]
impl_convert_for_hex!(ISizeHex, isize, isizehex);

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_convert_glam_for_hex {
    ($Self:ty, $Vec2:ty, $from_vec2:ident, $into_vec2:ident) => {
        impl core::convert::From<$Vec2> for $Self {
            #[inline(always)]
            fn from(value: $Vec2) -> Self {
                <$Self>::$from_vec2(value)
            }
        }

        impl core::convert::From<&$Vec2> for $Self {
            #[inline(always)]
            fn from(value: &$Vec2) -> Self {
                <$Self>::$from_vec2(*value)
            }
        }

        impl core::convert::From<$Self> for $Vec2 {
            #[inline(always)]
            fn from(value: $Self) -> Self {
                value.$into_vec2()
            }
        }

        impl core::convert::From<&$Self> for $Vec2 {
            #[inline(always)]
            fn from(value: &$Self) -> Self {
                value.$into_vec2()
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_convert_glam_for_hex!(Hex, Vec2, from_vec2, into_vec2);
#[cfg(feature = "f64")]
impl_convert_glam_for_hex!(DHex, DVec2, from_dvec2, into_dvec2);
#[cfg(feature = "i8")]
impl_convert_glam_for_hex!(I8Hex, I8Vec2, from_i8vec2, into_i8vec2);
#[cfg(feature = "i16")]
impl_convert_glam_for_hex!(I16Hex, I16Vec2, from_i16vec2, into_i16vec2);
#[cfg(feature = "i32")]
impl_convert_glam_for_hex!(IHex, IVec2, from_ivec2, into_ivec2);
#[cfg(feature = "i64")]
impl_convert_glam_for_hex!(I64Hex, I64Vec2, from_i64vec2, into_i64vec2);
#[cfg(feature = "isize")]
impl_convert_glam_for_hex!(ISizeHex, ISizeVec2, from_isizevec2, into_isizevec2);

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_convert_hex_for_hex {
    ($Self:ty, $as_hex:ident; $( #[$meta:meta] $Other:ty; )*) => {
        $(
            #[$meta]
            impl core::convert::From<$Other> for $Self {
                #[inline(always)]
                fn from(value: $Other) -> Self {
                    value.$as_hex()
                }
            }

            #[$meta]
            impl core::convert::From<&$Other> for $Self {
                #[inline(always)]
                fn from(value: &$Other) -> Self {
                    value.$as_hex()
                }
            }
        )*
    };
}

#[cfg(feature = "f32")]
impl_convert_hex_for_hex! {
    Hex, as_hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "i64")] I64Hex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "f64")]
impl_convert_hex_for_hex! {
    DHex, as_dhex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "i64")] I64Hex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "i8")]
impl_convert_hex_for_hex! {
    I8Hex, as_i8hex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "i64")] I64Hex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "i16")]
impl_convert_hex_for_hex! {
    I16Hex, as_i16hex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "i64")] I64Hex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "i32")]
impl_convert_hex_for_hex! {
    IHex, as_ihex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i64")] I64Hex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "i64")]
impl_convert_hex_for_hex! {
    I64Hex, as_i64hex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "isize")] ISizeHex;
}

#[cfg(feature = "isize")]
impl_convert_hex_for_hex! {
    ISizeHex, as_isizehex;
    #[cfg(feature = "f32")] Hex;
    #[cfg(feature = "f64")] DHex;
    #[cfg(feature = "i8")] I8Hex;
    #[cfg(feature = "i16")] I16Hex;
    #[cfg(feature = "i32")] IHex;
    #[cfg(feature = "i64")] I64Hex;
}
