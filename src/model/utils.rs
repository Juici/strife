/// Used in serde `skip_serializing_if` attribute.
pub fn is_false(b: &bool) -> bool {
    !b
}

macro_rules! int_visitor {
    ($vis:vis $name:ident : $type:ty) => {
        #[derive(Debug)]
        $vis struct $name;

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
    ($($vis:vis $name:ident : $type:ty ;)*) => {$(
        int_visitor!($vis $name: $type);
    )*}
}

int_visitor! {
    pub U8Visitor: u8;
    pub U16Visitor: u16;
    pub U64Visitor: u64;
}
