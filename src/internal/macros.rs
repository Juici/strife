#[doc(hidden)]
#[macro_export]
macro_rules! enum_constant {
    ($name:ident) => {
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match ::int_enum::IntEnum::as_int(self) {
                    Some(v) => serializer.serialize_u64(v),
                    None => unreachable!(),
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        formatter.write_str("positive integer")
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let from_int: Option<$name> = ::int_enum::IntEnum::from_int(v);
                        match from_int {
                            Some(v) => Ok(v),
                            None => Err(E::custom(format!(
                                "unknown {} value: {}",
                                stringify!($name),
                                v
                            ))),
                        }
                    }
                }

                deserializer.deserialize_u64(Visitor)
            }
        }
    };
}
