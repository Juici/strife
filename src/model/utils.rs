/// Used in serde `skip_serializing_if` attribute.
pub fn is_false(b: &bool) -> bool {
    !b
}

macro_rules! int_visitor {
    (($($vis:tt)*) $name:ident: $type:ty) => {
        #[derive(Debug)]
        $($vis)* struct $name;

        impl<'de> ::serde::de::Visitor<'de> for $name {
            type Value = $type;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                formatter.write_str(concat!("a ", stringify!($type), " integer"))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(v as $type)
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                Ok(v as $type)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                v.parse::<$type>().map_err(::serde::de::Error::custom)
            }
        }
    };
    ($(pub $name:ident: $type:ty;)*) => {$(
        int_visitor!((pub) $name: $type);
    )*};
    ($(pub(crate) $name:ident: $type:ty;)*) => {$(
        int_visitor!((pub(crate)) $name: $type);
    )*};
}

int_visitor! {
    pub U8Visitor: u8;
    pub U16Visitor: u16;
    pub U64Visitor: u64;
}

/// Serde mappings of sequences of objects with Snowflake IDs to HashMaps keyed
/// with by the Snowflake IDs.
pub mod serde_id_map {
    use std::collections::HashMap;
    use std::fmt;
    use std::marker::PhantomData;

    use serde::{de, Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    use crate::model::id::ToSnowflakeId;

    pub fn deserialize<'de, D, V: 'de>(deserializer: D) -> Result<HashMap<V::Id, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: ToSnowflakeId + Deserialize<'de>,
    {
        struct Visitor<'de, V: 'de>
        where
            V: ToSnowflakeId + Deserialize<'de>,
        {
            _value: &'de PhantomData<V>,
        }

        impl<'de, V: 'de> de::Visitor<'de> for Visitor<'de, V>
        where
            V: ToSnowflakeId + Deserialize<'de>,
        {
            type Value = HashMap<V::Id, V>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of objects with snowflake ids")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut map: HashMap<V::Id, V> =
                    HashMap::with_capacity(seq.size_hint().unwrap_or_default());

                while let Some(value) = seq.next_element::<V>()? {
                    if let Some(existing) = map.insert(<V as ToSnowflakeId>::id(&value), value) {
                        return Err(de::Error::custom(format_args!(
                            "duplicate snowflake id: {}",
                            <V as ToSnowflakeId>::id(&existing),
                        )));
                    }
                }

                Ok(map)
            }
        }

        deserializer.deserialize_seq(Visitor {
            _value: &PhantomData,
        })
    }

    pub fn serialize<S, V>(map: &HashMap<V::Id, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: ToSnowflakeId + Serialize,
    {
        serializer.collect_seq(map.values())
    }
}
