//! Private macros used throughout the library.

macro_rules! pkg_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

macro_rules! pkg_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

macro_rules! pkg_repo {
    () => {
        env!("CARGO_PKG_REPOSITORY")
    };
}

#[doc(hidden)]
macro_rules! api_base {
    () => {
        "https://discordapp.com/api/v6"
    };
}

#[doc(hidden)]
macro_rules! __api {
    (@s $s:expr) => {
        concat!(api_base!(), $s)
    };
    (@s $s:expr; @a $($arg:expr),*) => {
        format!(__api!(@s $s), $($arg),*)
    };
    (@s $s:expr; @a $($arg:expr),* ; @p []; @o []) => {
        __api!(@s $s; @a $($arg),*)
    };
    (@d $dst:expr; @o []) => {};
    (@d $dst:expr; @o [($key: literal, $value:tt?), $($tail:tt)*]) => {
        __api!(@d $dst; @o [$($tail)*]);
        if let Some(value) = $value {
            let _ = write!($dst, concat!("&", $key, "={}"), value);
        }
    };
    (@s $s:expr; @a $($arg:expr),* ; @p []; @o [$($opts:tt)*]) => {{
        use ::std::fmt::Write;
        let mut s = __api!(@s $s; @a $($arg),*);
        __api!(@d s; @o [$($opts)*]);
        s
    }};
    (@s $s:expr; @a $($arg:expr),* ; @p [($($param:tt)*)]; @o [$($opts:tt)*]) => {
        __api!(@s $s; @a $($arg),*; @p [($($param)*),]; @o [$($opts)*])
    };
    (@s $s:expr; @a $($arg:expr),* ; @p [($key:literal, $value:tt), $($tail:tt)*]; @o [$($opts:tt)*]) => {
        __api!(@s concat!($s, "&", $key, "={}"); @a $($arg,)* $value; @p [$($tail)*]; @o [$($opts)*])
    };
    (@s $s:expr; @a $($arg:expr),* ; @p [($key:literal, $value:tt?), $($tail:tt)*]; @o [$($opts:tt)*]) => {
        __api!(@s $s; @a $($arg),*; @p [$($tail)*]; @o [($key, $value?), $($opts)*])
    };
}

macro_rules! api {
    ($s:expr) => {
        __api!(@s $s)
    };
    ($s:expr, $($arg:expr),* $(,)*) => {
        __api!(@s $s; @a $($arg),*)
    };
    ($s:expr, $($arg:expr),* $(,)*; []) => {
        api!($s, $($arg),*)
    };
    ($s:expr, $($arg:expr),* $(,)*; [$($params:tt)*]) => {
        __api!(@s concat!($s, "?"); @a $($arg),*; @p [$($params)*]; @o [])
    };
}

#[doc(hidden)]
macro_rules! __serialize_as {
    ($s:expr, $v:ident as u8) => {
        $s.serialize_u8($v)
    };
    ($s:expr, $v:ident as u16) => {
        $s.serialize_u16($v)
    };
    ($s:expr, $v:ident as u32) => {
        $s.serialize_u32($v)
    };
    ($s:expr, $v:ident as u64) => {
        $s.serialize_u64($v)
    };
}

#[doc(hidden)]
macro_rules! __deserialize_as {
    ($d:expr, $v:ident as u8) => {
        $d.deserialize_u8($v)
    };
    ($d:expr, $v:ident as u16) => {
        $d.deserialize_u16($v)
    };
    ($d:expr, $v:ident as u32) => {
        $d.deserialize_u32($v)
    };
    ($d:expr, $v:ident as u64) => {
        $d.deserialize_u64($v)
    };
}

#[doc(hidden)]
macro_rules! __visit_as {
    ($v:ident: u8 => $($f:tt)*) => {
        fn visit_u8<E: ::serde::de::Error>(self, $v: u8) -> ::std::result::Result<Self::Value, E> {
            $($f)*
        }
    };
    ($v:ident: u16 => $($f:tt)*) => {
        fn visit_u16<E: ::serde::de::Error>(self, $v: u16) -> ::std::result::Result<Self::Value, E> {
            $($f)*
        }
    };
    ($v:ident: u32 => $($f:tt)*) => {
        fn visit_u32<E: ::serde::de::Error>(self, $v: u32) -> ::std::result::Result<Self::Value, E> {
            $($f)*
        }
    };
    ($v:ident: u64 => $($f:tt)*) => {
        fn visit_u64<E: ::serde::de::Error>(self, $v: u64) -> ::std::result::Result<Self::Value, E> {
            $($f)*
        }
    };
}

macro_rules! int_enum {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident: $T:tt {
            $($inner:tt)*
        }
    ) => {
        #[int_enum::int_enum($T)]
        $(#[$attrs])*
        $vis enum $name {
            $($inner)*
        }

        impl ::std::convert::From<$T> for $name {
            fn from(n: $T) -> Self {
                match ::int_enum::IntEnum::from_int(n) {
                    Some(n) => n,
                    None => unreachable!(),
                }
            }
        }

        impl ::std::convert::From<$name> for $T {
            fn from(n: $name) -> Self {
                match ::int_enum::IntEnum::as_int(&n) {
                    Some(n) => n,
                    None => unreachable!(),
                }
            }
        }

        impl<'a> ::std::convert::From<&'a $name> for $T {
            fn from(n: &'a $name) -> Self {
                match ::int_enum::IntEnum::as_int(n) {
                    Some(n) => n,
                    None => unreachable!(),
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match ::int_enum::IntEnum::as_int(self) {
                    Some(v) => __serialize_as!(serializer, v as $T),
                    None => unreachable!(),
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    __visit_as! { v: $T => {
                        let from_int: Option<$name> = ::int_enum::IntEnum::from_int(v);
                        match from_int {
                            Some(v) => Ok(v),
                            None => Err(E::custom(format!(
                                concat!("unknown value for ", stringify!($name), ": {}"),
                                v
                            ))),
                        }
                    }}

                    fn expecting(
                        &self,
                        formatter: &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result {
                        formatter.write_str(concat!(stringify!($T), " integer"))
                    }
                }

                __deserialize_as!(deserializer, Visitor as $T)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    const ID: u64 = 80351110224678912;

    #[test]
    fn test_basic() {
        assert_eq!(api_base!(), api!(""));
        assert_eq!(concat!(api_base!(), "/guilds"), api!("/guilds"));
    }

    #[test]
    fn test_arg() {
        assert_eq!(
            format!("{}/guilds/{}/audit-logs", api_base!(), ID),
            api!("/guilds/{}/audit-logs", ID)
        );
        assert_eq!(
            format!("{}/guilds/{}/audit-logs", api_base!(), ID),
            api!("/guilds/{}/audit-logs", ID; [])
        );
    }

    #[test]
    fn test_query_basic() {
        let user_id: u64 = 123;
        let url = format!(
            "{}/guilds/{}/audit-logs?&user_id={}",
            api_base!(),
            ID,
            user_id
        );

        assert_eq!(
            url,
            api!("/guilds/{}/audit-logs", ID; [
                ("user_id", user_id),
            ])
        );
    }

    #[test]
    fn test_query_none() {
        let user_id: Option<u64> = None;
        let url = format!("{}/guilds/{}/audit-logs?", api_base!(), ID);

        assert_eq!(
            url,
            api!("/guilds/{}/audit-logs", ID; [
                ("user_id", user_id?),
            ])
        );
    }

    #[test]
    fn test_query_some() {
        let user_id: Option<u64> = Some(456);
        let url = format!(
            "{}/guilds/{}/audit-logs?&user_id={}",
            api_base!(),
            ID,
            user_id.unwrap()
        );

        assert_eq!(
            url,
            api!("/guilds/{}/audit-logs", ID; [
                ("user_id", user_id?)
            ])
        );
    }

    #[test]
    fn test_query_complex() {
        let user_id: u64 = 789;
        let action_type: Option<u64> = Some(5);
        let before: Option<u64> = None;
        let limit: u64 = 42;

        let url = format!(
            "{}/guilds/{}/audit-logs?&user_id={}&limit={}&action_type={}",
            api_base!(),
            ID,
            user_id,
            limit,
            action_type.unwrap(),
        );

        assert_eq!(
            url,
            api!("/guilds/{}/audit-logs", ID; [
                ("user_id", user_id),
                ("action_type", action_type?),
                ("before", before?),
                ("limit", limit),
            ])
        );
        assert_eq!(
            url,
            api!("/guilds/{}/audit-logs", ID; [
                ("action_type", action_type?),
                ("before", before?),
                ("user_id", user_id),
                ("limit", limit),
            ])
        );
    }
}
