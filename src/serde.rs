// imports `serde_core` as `serde`.
#[cfg(feature = "serde")]
extern crate serde_core as serde;

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
macro_rules! impl_serialize_for_hex {
    ($Self:ty) => {
        impl serde::Serialize for $Self {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeTupleStruct;
                let mut serializer = serializer.serialize_tuple_struct(stringify!($Self), 2)?;
                serializer.serialize_field(&self.q)?;
                serializer.serialize_field(&self.r)?;
                serializer.end()
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_serialize_for_hex!(Hex);
#[cfg(feature = "f64")]
impl_serialize_for_hex!(DHex);
#[cfg(feature = "i8")]
impl_serialize_for_hex!(I8Hex);
#[cfg(feature = "i16")]
impl_serialize_for_hex!(I16Hex);
#[cfg(feature = "i32")]
impl_serialize_for_hex!(IHex);
#[cfg(feature = "i64")]
impl_serialize_for_hex!(I64Hex);
#[cfg(feature = "isize")]
impl_serialize_for_hex!(ISizeHex);

#[cfg(any(
    feature = "f32",
    feature = "f64",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "isize",
))]
macro_rules! impl_deserialize_for_hex {
    ($Self:ty, $T:ty, $new:path) => {
        impl<'de> serde::Deserialize<'de> for $Self {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct HexVisitor;

                impl<'de> serde::de::Visitor<'de> for HexVisitor {
                    type Value = $Self;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str(stringify!($Self))?;
                        f.write_str(" (consists of two `")?;
                        f.write_str(stringify!($T))?;
                        f.write_str("` elements)")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<$Self, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let Some(q) = seq.next_element()? else {
                            return Err(serde::de::Error::invalid_length(0, &self));
                        };
                        let Some(r) = seq.next_element()? else {
                            return Err(serde::de::Error::invalid_length(1, &self));
                        };
                        let Some(serde::de::IgnoredAny) = seq.next_element()? else {
                            return Err(serde::de::Error::invalid_length(3, &self));
                        };
                        Ok($new(q, r))
                    }
                }

                deserializer.deserialize_tuple_struct(stringify!($Self), 2, HexVisitor)
            }
        }
    };
}

#[cfg(feature = "f32")]
impl_deserialize_for_hex!(Hex, f32, hex);
#[cfg(feature = "f64")]
impl_deserialize_for_hex!(DHex, f64, dhex);
#[cfg(feature = "i8")]
impl_deserialize_for_hex!(I8Hex, i8, i8hex);
#[cfg(feature = "i16")]
impl_deserialize_for_hex!(I16Hex, i16, i16hex);
#[cfg(feature = "i32")]
impl_deserialize_for_hex!(IHex, i32, ihex);
#[cfg(feature = "i64")]
impl_deserialize_for_hex!(I64Hex, i64, i64hex);
#[cfg(feature = "isize")]
impl_deserialize_for_hex!(ISizeHex, isize, isizehex);
