/// Used in serde `skip_serializing_if` attribute.
#[allow(clippy::trivially_copy_pass_by_ref)]
#[inline]
pub fn is_false(b: &bool) -> bool {
    !b
}

/// Used in serde `default` attribute.
#[inline]
pub fn default_true() -> bool {
    true
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
    // ToSnowflakeId::Id is marked as interior mutable by clippy, but due to the
    // trait being sealed we can assert it is not.
    #![allow(clippy::mutable_key_type)]

    use std::collections::HashMap;
    use std::fmt;
    use std::marker::PhantomData;

    use serde::de;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::model::id::ToSnowflakeId;

    struct Visitor<'de, V: 'de>
    where
        V: ToSnowflakeId + Deserialize<'de>,
    {
        _value: PhantomData<&'de V>,
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
            // Default to `0` to avoid allocations for empty maps.
            let size = seq.size_hint().unwrap_or(0);

            let mut map: HashMap<V::Id, V> = HashMap::with_capacity(size);

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

    pub fn deserialize<'de, D, V: 'de>(deserializer: D) -> Result<HashMap<V::Id, V>, D::Error>
    where
        D: Deserializer<'de>,
        V: ToSnowflakeId + Deserialize<'de>,
    {
        deserializer.deserialize_seq(Visitor {
            _value: PhantomData,
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

pub mod serde_option_timestamp {
    use chrono::serde::ts_milliseconds;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct OptionWrapper(
            #[serde(deserialize_with = "ts_milliseconds::deserialize")] DateTime<Utc>,
        );

        let v = Option::deserialize(deserializer)?;
        Ok(v.map(|OptionWrapper(dt)| dt))
    }

    pub fn serialize<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match dt {
            Some(dt) => ts_milliseconds::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }
}
