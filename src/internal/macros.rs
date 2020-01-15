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
macro_rules! __api {
    () => {
        "https://discordapp.com/api/v6"
    };
    (@s $s:expr) => {
        concat!(__api!(), $s)
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
    () => {
        __api!()
    };
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

macro_rules! wrap {
    ($parent:ty => mut $field:ident: $child:ty) => {
        wrap!($parent => $field: $child);

        impl ::std::ops::DerefMut for $parent {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }

        impl ::std::convert::AsMut<$child> for $parent {
            fn as_mut(&mut self) -> &mut $child {
                &mut self.$field
            }
        }
    };
    ($parent:ty => $field:ident: $child:ty) => {
        impl ::std::ops::Deref for $parent {
            type Target = $child;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::convert::AsRef<$child> for $parent {
            fn as_ref(&self) -> &$child {
                &self.$field
            }
        }

        impl ::std::convert::From<$parent> for $child {
            fn from(parent: $parent) -> Self {
                parent.$field
            }
        }
    };
}

macro_rules! impl_to_snowflake {
    ($T:ident: |$_self:ident| $($map:tt)*) => {
        #[doc(hidden)]
        impl $crate::model::snowflake::private::Sealed for $T {}

        impl $crate::model::snowflake::ToSnowflake for $T {
            fn snowflake(&self) -> $crate::model::snowflake::Snowflake {
                match self {
                    $_self => { $($map)* }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    const ID: u64 = 80351110224678912;

    #[test]
    fn test_basic() {
        assert_eq!(concat!(api!(), "/guilds"), api!("/guilds"));
    }

    #[test]
    fn test_arg() {
        assert_eq!(
            format!("{}/guilds/{}/audit-logs", api!(), ID),
            api!("/guilds/{}/audit-logs", ID)
        );
        assert_eq!(
            format!("{}/guilds/{}/audit-logs", api!(), ID),
            api!("/guilds/{}/audit-logs", ID; [])
        );
    }

    #[test]
    fn test_query_basic() {
        let user_id: u64 = 123;
        let url = format!("{}/guilds/{}/audit-logs?&user_id={}", api!(), ID, user_id);

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
        let url = format!("{}/guilds/{}/audit-logs?", api!(), ID);

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
            api!(),
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
            api!(),
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
