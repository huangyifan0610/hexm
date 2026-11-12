#[cfg(feature = "f64")]
use glam::{DVec2, dvec2};
#[cfg(feature = "i8")]
use glam::{I8Vec2, i8vec2};
#[cfg(feature = "i16")]
use glam::{I16Vec2, i16vec2};
#[cfg(feature = "i64")]
use glam::{I64Vec2, i64vec2};
#[cfg(feature = "isize")]
use glam::{ISizeVec2, isizevec2};
#[cfg(feature = "i32")]
use glam::{IVec2, ivec2};
#[cfg(feature = "f32")]
use glam::{Vec2, vec2};

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
macro_rules! impl_hex {
    ($new:path, $T:ty, 0 = $_0:literal, 1 = $_1:literal) => {
        /// A zero vector, i.e., `(0, 0, 0)`.
        pub const ZERO: Self = $new($_0, $_0);

        /// A unit vector between `+Q` and `-R` axes, i.e., `(1, -1, 0)`.
        pub const QR: Self = $new($_1, -$_1);

        /// A unit vector between `+Q` and `-S` axes, i.e., `(1, 0, -1)`.
        pub const QS: Self = $new($_1, $_0);

        /// A unit vector between `+R` and `-Q` axes, i.e., `(-1, 1, 0)`.
        pub const RQ: Self = $new(-$_1, $_1);

        /// A unit vector between `+R` and `-S` axes, i.e., `(0, 1, -1)`.
        pub const RS: Self = $new($_0, $_1);

        /// A unit vector between `+S` and `-Q` axes, i.e., `(-1, 0, 1)`.
        pub const SQ: Self = $new(-$_1, $_0);

        /// A unit vector between `+S` and `-R` axes, i.e., `(0, -1, 1)`.
        pub const SR: Self = $new($_0, -$_1);

        /// Crates an axial-coordinate vector.
        ///
        /// The `q` and `r` parameters represent the *Q* and *R* components of the
        /// result vector respectively.
        ///
        #[doc = concat!("See also [`", stringify!($new), "`] for an equivalent function.")]
        #[inline(always)]
        #[must_use]
        pub const fn new(q: $T, r: $T) -> Self {
            $new(q, r)
        }

        /// Converts the 2-dimensional array into an axial-coordinate vector.
        ///
        /// The first and second elements of the input array will become the *Q* and
        /// *R* components of the result vector respectively.
        ///
        /// See also [`Self::into_array`] for an inverse function;
        /// and [`From::from`] for an equivalent function.
        #[inline(always)]
        #[must_use]
        pub const fn from_array(array: [$T; 2]) -> Self {
            $new(array[0], array[1])
        }

        /// Converts the axial-coordinate vector into a 2-dimensional array.
        ///
        /// The *Q* and *R* components of the `self` vector will become the first
        /// and second elements of the result array respectively.
        ///
        /// See also [`Self::from_array`] for an inverse function;
        /// and [`Into::into`] for an equivalent function.
        #[inline(always)]
        #[must_use]
        pub const fn into_array(self) -> [$T; 2] {
            [self.q, self.r]
        }

        /// Reinterprets the reference to the axial-coordinate vector into reference
        /// to a 2-dimensional array.
        ///
        /// The *Q* and *R* components of the `self` vector will become the first
        /// and second elements of the result array respectively.
        ///
        /// See also [`AsRef::as_ref`] for an equivalent function;
        /// and [`Self::as_mut_array`] for a mutable version.
        #[inline(always)]
        #[must_use]
        pub const fn as_array(&self) -> &[$T; 2] {
            // SAFETY: We can transmute the reference because the `Self` struct
            // consists of exactly two scalars and has `repr(C)` representation.
            unsafe { &*(self as *const Self as *const [$T; 2]) }
        }

        /// Reinterprets the mutable reference to the axial-coordinate vector into
        /// mutable reference to a 2-dimensional array.
        ///
        /// The *Q* and *R* components of the `self` vector will become the first
        /// and second elements of the result array respectively.
        ///
        /// See also [`AsMut::as_mut`] for an equivalent function;
        /// and [`Self::as_array`] for an immutable version.
        #[inline(always)]
        #[must_use]
        pub const fn as_mut_array(&mut self) -> &mut [$T; 2] {
            // SAFETY: We can transmute the reference because the `Self` struct
            // consists of exactly two scalars and has `repr(C)` representation.
            unsafe { &mut *(self as *mut Self as *mut [$T; 2]) }
        }

        /// Returns the *Q* component of the `self` vector.
        ///
        /// This function identically returns the struct member `q`.
        #[inline(always)]
        #[must_use]
        pub const fn q(&self) -> $T {
            self.q
        }

        /// Returns the *R* component of the `self` vector.
        ///
        /// This function identically returns the struct member `r`.
        #[inline(always)]
        #[must_use]
        pub const fn r(&self) -> $T {
            self.r
        }

        /// Returns the *S* component of the `self` vector.
        ///
        /// The result is derived by form `q + r + s === 0`.
        #[inline(always)]
        #[must_use]
        pub const fn s(&self) -> $T {
            -(self.q + self.r)
        }

        /// Reflects the axial-coordinate vector across the *Q* axis.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(q, s, r)`.
        #[inline(always)]
        #[must_use]
        pub const fn reflect_q(self) -> Self {
            $new(self.q, self.s())
        }

        /// Reflects the axial-coordinate vector across the *R* axis.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(s, r, q)`.
        #[inline(always)]
        #[must_use]
        pub const fn reflect_r(self) -> Self {
            $new(self.s(), self.r)
        }

        /// Reflects the axial-coordinate vector across the *S* axis.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(r, q, s)`.
        #[inline(always)]
        #[must_use]
        pub const fn reflect_s(self) -> Self {
            $new(self.r, self.q)
        }

        /// Rotates the axial-coordinate vector around the zero vector by **60**
        /// degrees clockwise.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(-r, -s, -q)`.
        #[inline(always)]
        #[must_use]
        pub const fn rotate_60_degrees(self) -> Self {
            $new(-self.r, self.q + self.r)
        }

        /// Rotates the axial-coordinate vector around the zero vector by **120**
        /// degrees clockwise.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(s, q, r)`.
        #[inline(always)]
        #[must_use]
        pub const fn rotate_120_degrees(self) -> Self {
            $new(self.s(), self.q)
        }

        /// Rotates the axial-coordinate vector around the zero vector by **240**
        /// degrees clockwise.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(r, s, q)`.
        #[inline(always)]
        #[must_use]
        pub const fn rotate_240_degrees(self) -> Self {
            $new(self.r, self.s())
        }

        /// Rotates the axial-coordinate vector around the zero vector by **240**
        /// degrees clockwise.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(-s, -q, -r)`.
        #[inline(always)]
        #[must_use]
        pub const fn rotate_300_degrees(self) -> Self {
            $new(self.q + self.r, -self.q)
        }

        /// Returns the opposite of the axial-coordinate vector.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `(-q, -r, -s)`.
        ///
        /// This function is equivalent to unary negation, i.e.,
        /// [`-self`](core::ops::Neg).
        #[inline(always)]
        #[must_use]
        pub const fn opposite(self) -> Self {
            $new(-self.q, -self.r)
        }

        /// Adds an offset to the axial-coordinate vector.
        ///
        /// Given two axial-coordinate vectors `(q₁, r₁, s₁)` and ,`(q₂, r₂, s₂)`,
        /// this function returns `(q₁ + q₂, r₁ + r₂, s₁ + s₂)`.
        ///
        /// This function is equivalent to addition, i.e.
        /// [`self + rhs`](core::ops::Add);
        ///
        /// And it satisfies the commutative law, i.e., `self.offset(rhs)` is
        /// equivalent to `rhs.offset(self)`.
        #[inline(always)]
        #[must_use]
        pub const fn offset(self, rhs: Self) -> Self {
            $new(self.q + rhs.q, self.r + rhs.r)
        }

        /// Returns the offset between two axial-coordinate vectors.
        ///
        /// Given two axial-coordinate vectors `(q₁, r₁, s₁)` and `(q₂, r₂, s₂)`,
        /// this function returns `(q₁ - q₂, r₁ - r₂, s₁ - s₂)`.
        ///
        /// This function is equivalent to subtraction, i.e.
        /// [`self - rhs`](core::ops::Sub).
        ///
        /// It does **NOT** satisfy the commutative law. Specially,
        /// `self.offset_from(rhs)` is equivalent to `-rhs.offset_from(self)`.
        #[inline(always)]
        #[must_use]
        pub const fn offset_from(self, rhs: Self) -> Self {
            $new(self.q - rhs.q, self.r - rhs.r)
        }

        /// Scales the axial-coordinate vector by a scalar.
        ///
        /// Given axial-coordinate `(q, r, s)` and scalar `k`, this function returns
        /// `(q * k, r * k, s * k)`.
        ///
        /// This function is equivalent to multiplication, i.e.
        /// [`self * rhs`](core::ops::Mul).
        #[inline(always)]
        #[must_use]
        pub const fn scale(self, rhs: $T) -> Self {
            $new(self.q * rhs, self.r * rhs)
        }

        /// Calculates the midpoint (average) between two axial-coordinate vectors.
        ///
        /// Given two axial-coordinate vectors `(q₁, r₁, s₁)` and `(q₂, r₂, s₂)`, this
        /// function returns `(q₁.midpoint(q₂), r₁.midpoint(r₂), s₁.midpoint(s₂))`.
        ///
        /// See also [`mindpoint`](f32::midpoint) for more information.
        #[inline(always)]
        #[must_use]
        pub const fn midpoint(self, rhs: Self) -> Self {
            $new(self.q.midpoint(rhs.q), self.r.midpoint(rhs.r))
        }
    };
}

#[cfg(any(feature = "f32", feature = "f64"))]
macro_rules! impl_floating_point_hex {
    ($T:ty) => {
        /// Performs a linear interpolation between two axial-coordinate vectors
        /// based on a scalar value.
        ///
        /// When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`,
        /// the result will be equal to `rhs`. When s is outside of the range [0, 1],
        /// the result is linearly extrapolated.
        ///
        /// This function is equivalent to `self * (1.0 - s) + rhs * s`.+
        #[inline(always)]
        #[must_use]
        pub const fn lerp(self, rhs: Self, s: $T) -> Self {
            self.scale(1. - s).offset(rhs.scale(s))
        }

        /// Calculates the length of an axial-coordinate vector, i.e. the distance
        /// between the `self` vector and the zero vector.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `max{|q|, |r|, |s|}`.
        #[must_use]
        pub const fn length(self) -> $T {
            if self.q.is_sign_negative() ^ self.r.is_sign_negative() {
                // The signs of `q` and `r` are different, therefore
                // `|q + r| < max{|q|, |r|}`.
                self.q.abs().max(self.r.abs())
            } else {
                // The signs of `q` and `r` are the same, therefore
                // `|q + r| >= max{|q|, |r|}`.
                (self.q + self.r).abs()
            }
        }

        /// Calculates the distance between two axial-coordinate vectors.
        ///
        /// Given two axial-coordinate vectors `(q₁, r₁, s₁)` and ,`(q₂, r₂, s₂)`,
        /// this function returns `max{|q₁ - q₂|, |r₁ - r₂|, |s₁ - s₂|}`.
        ///
        /// This function satisfies the commutative law, i.e., `self.distance(rhs)`
        /// is equivalent to `rhs.distance(self)`.
        #[inline(always)]
        #[must_use]
        pub const fn distance(self, rhs: Self) -> $T {
            self.offset_from(rhs).length()
        }
    };
}

#[cfg(any(
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_integral_hex {
    ($T:ty, $U:ty) => {
        /// Calculates the length of an axial-coordinate vector, i.e. the distance
        /// between the `self` vector and the zero vector.
        ///
        /// Given axial-coordinate vector `(q, r, s)`, this function returns
        /// `max{|q|, |r|, |s|}`.
        #[must_use]
        pub const fn length(self) -> $U {
            #[inline(always)]
            const fn const_max(x: $U, y: $U) -> $U {
                if x < y { y } else { x }
            }

            if (self.q < 0) ^ (self.r < 0) {
                // The signs of `q` and `r` are different, therefore
                // `|q + r| < max{|q|, |r|}`.
                const_max(self.q.unsigned_abs(), self.r.unsigned_abs())
            } else {
                // The signs of `q` and `r` are the same, therefore
                // `|q + r| >= max{|q|, |r|}`.
                (self.q + self.r).unsigned_abs()
            }
        }

        /// Calculates the distance between two axial-coordinate vectors.
        ///
        /// Given two axial-coordinate vectors `(q₁, r₁, s₁)` and ,`(q₂, r₂, s₂)`,
        /// this function returns `max{|q₁ - q₂|, |r₁ - r₂|, |s₁ - s₂|}`.
        ///
        /// This function satisfies the commutative law, i.e., `self.distance(rhs)`
        /// is equivalent to `rhs.distance(self)`.
        #[inline(always)]
        #[must_use]
        pub const fn distance(self, rhs: Self) -> $U {
            #[inline(always)]
            const fn const_max(x: $U, y: $U) -> $U {
                if x < y { y } else { x }
            }

            if (self.q < rhs.q) ^ (self.r < rhs.r) {
                const_max(self.q.abs_diff(rhs.q), self.r.abs_diff(rhs.r))
            } else {
                (self.q + self.r).abs_diff(rhs.q + rhs.r)
            }
        }
    };
}

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_hex_convert_glam {
    ($Vec2:ty, $vec2:path, $new:path, $from_vec2:ident, $into_vec2:ident) => {
        /// Converts the 2-dimensional vector into an axial-coordinate vector.
        ///
        /// The *X* and *Y* components of the input vector will become the *Q* and
        /// *R* components of the result vector respectively.
        ///
        #[doc = concat!("See also [`Self::", stringify!($into_vec2), "`] for an inverse function;")]
        /// and [`From::from`] for an equivalent function.
        #[inline(always)]
        #[must_use]
        pub const fn $from_vec2(vector: $Vec2) -> Self {
            $new(vector.x, vector.y)
        }

        /// Converts the axial-coordinate vector into a [2-dimensional vector].
        ///
        /// The *Q* and *R* components of the `self` vector will become the *X* and
        /// *Y* components of the result vector respectively.
        ///
        #[doc = concat!("See also [`Self::", stringify!($from_vec2), "`] for an inverse function;")]
        /// and [`Into::into`] for an equivalent function.
        #[inline(always)]
        #[must_use]
        pub const fn $into_vec2(self) -> $Vec2 {
            $vec2(self.q, self.r)
        }
    };
}

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_hex_convert_hex {
    ($( #[$meta:meta] $Other:ty, $other:path, $as_hex:ident, $U:ty; )*) => {
        $(
            #[$meta]
            /// Converts the scalar type of the axial-coordinate vector.
            ///
            /// See also [`Into::into`] for an equivalent function.
            #[inline(always)]
            #[must_use]
            pub const fn $as_hex(self) -> $Other {
                $other(self.q as $U, self.r as $U)
            }
        )*
    };
}

#[cfg(feature = "f32")]
impl Hex {
    impl_hex!(hex, f32, 0 = 0., 1 = 1.);
    impl_floating_point_hex!(f32);
    impl_hex_convert_glam!(Vec2, vec2, hex, from_vec2, into_vec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "f64")]
impl DHex {
    impl_hex!(dhex, f64, 0 = 0., 1 = 1.);
    impl_floating_point_hex!(f64);
    impl_hex_convert_glam!(DVec2, dvec2, dhex, from_dvec2, into_dvec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "i8")]
impl I8Hex {
    impl_hex!(i8hex, i8, 0 = 0, 1 = 1);
    impl_integral_hex!(i8, u8);
    impl_hex_convert_glam!(I8Vec2, i8vec2, i8hex, from_i8vec2, into_i8vec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "i16")]
impl I16Hex {
    impl_hex!(i16hex, i16, 0 = 0, 1 = 1);
    impl_integral_hex!(i16, u16);
    impl_hex_convert_glam!(I16Vec2, i16vec2, i16hex, from_i16vec2, into_i16vec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "i32")]
impl IHex {
    impl_hex!(ihex, i32, 0 = 0, 1 = 1);
    impl_integral_hex!(i32, u32);
    impl_hex_convert_glam!(IVec2, ivec2, ihex, from_ivec2, into_ivec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "i64")]
impl I64Hex {
    impl_hex!(i64hex, i64, 0 = 0, 1 = 1);
    impl_integral_hex!(i64, u64);
    impl_hex_convert_glam!(I64Vec2, i64vec2, i64hex, from_i64vec2, into_i64vec2);
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "isize")] ISizeHex, isizehex, as_isizehex, isize;
    }
}

#[cfg(feature = "isize")]
impl ISizeHex {
    impl_hex!(isizehex, isize, 0 = 0, 1 = 1);
    impl_integral_hex!(isize, usize);
    impl_hex_convert_glam!(
        ISizeVec2,
        isizevec2,
        isizehex,
        from_isizevec2,
        into_isizevec2
    );
    impl_hex_convert_hex! {
        #[cfg(feature = "f32")] Hex, hex, as_hex, f32;
        #[cfg(feature = "f64")] DHex, dhex, as_dhex, f64;
        #[cfg(feature = "i8")] I8Hex, i8hex, as_i8hex, i8;
        #[cfg(feature = "i16")] I16Hex, i16hex, as_i16hex, i16;
        #[cfg(feature = "i32")] IHex, ihex, as_ihex, i32;
        #[cfg(feature = "i64")] I64Hex, i64hex, as_i64hex, i64;
    }
}
