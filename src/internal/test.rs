#[cfg(test)]
#[macro_use]
mod inner {
    macro_rules! assert_eq_fields {
        ($left:expr, $right:expr) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    // Use a hack with trait method resolution to allow pseudo-specialization.
                    #[allow(unused_imports)]
                    use $crate::internal::test::{EqFields, DelegateEqFields};

                    left_val.eq_fields(right_val);
                }
            }
        };
        ($left:expr, $right:expr, [$($field:ident),* $(,)*]) => {
            $(
                assert_eq_fields!($left.$field, $right.$field);
            )*
        };
    }

    macro_rules! panic_ne_fields {
        ($left:expr, $right:expr) => {
            match (&$left, &$right) {
                (left_val, right_val) => panic!(
                    r#"assertion failed: `(left == right) by fields`
  left: `{:?}`,
 right: `{:?}`"#,
                    &*left_val, &*right_val
                ),
            }
        };
    }

    pub trait EqFields<Rhs: ?Sized = Self>: std::fmt::Debug
    where
        Rhs: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &Rhs);
    }

    impl<A: ?Sized, B: ?Sized> EqFields<&B> for &A
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &&B) {
            EqFields::eq_fields(*self, *other);
        }
    }

    impl<A: ?Sized, B: ?Sized> EqFields<&mut B> for &mut A
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &&mut B) {
            EqFields::eq_fields(*self, *other);
        }
    }

    impl<A: ?Sized, B: ?Sized> EqFields<&B> for &mut A
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &&B) {
            EqFields::eq_fields(*self, *other);
        }
    }

    impl<A: ?Sized, B: ?Sized> EqFields<&mut B> for &A
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &&mut B) {
            EqFields::eq_fields(*self, *other);
        }
    }

    impl<A, B> EqFields<Option<B>> for Option<A>
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &Option<B>) {
            match (self, other) {
                (Some(left_val), Some(right_val)) => EqFields::eq_fields(left_val, right_val),
                (None, None) => {}
                (left_val, right_val) => panic_ne_fields!(left_val, right_val),
            }
        }
    }

    impl<A, B, E> EqFields<Result<B, E>> for Result<A, E>
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
        E: EqFields<E>,
    {
        fn eq_fields(&self, other: &Result<B, E>) {
            match (self, other) {
                (Ok(left_val), Ok(right_val)) => EqFields::eq_fields(left_val, right_val),
                (Err(left_val), Err(right_val)) => EqFields::eq_fields(left_val, right_val),
                (left_val, right_val) => panic_ne_fields!(left_val, right_val),
            }
        }
    }

    impl<A, B> EqFields<Vec<B>> for Vec<A>
    where
        A: EqFields<B>,
        B: std::fmt::Debug,
    {
        fn eq_fields(&self, other: &Vec<B>) {
            for (a, b) in self.iter().zip(other.iter()) {
                assert_eq_fields!(a, b);
            }
        }
    }

    pub trait DelegateEqFields {
        fn eq_fields(&self, other: &Self);
    }

    impl<T: ?Sized> DelegateEqFields for T
    where
        T: std::fmt::Debug + PartialEq,
    {
        fn eq_fields(&self, other: &Self) {
            assert_eq!(self, other);
        }
    }
}

#[cfg(test)]
#[doc(hidden)]
pub use self::inner::*;

#[doc(hidden)]
macro_rules! __impl_eq_fields {
    ($name:ident: ($a:ident, $b:ident) => { $($inner:tt)* }) => {
        #[cfg(test)]
        const _: () = {
            use $crate::internal::test::EqFields;

            impl EqFields for $name {
                fn eq_fields(&self, other: &Self) {
                    match (self, other) {
                        ($a, $b) => { $($inner)* }
                    }
                }
            }
        };
    };
}

macro_rules! impl_eq_fields {
    ($name:ident: ($a:ident, $b:ident) => { $($inner:tt)* }) => {
        __impl_eq_fields!($name: ($a, $b) => {
            $($inner)*
        });
    };
    ($name:ident: [$($field:ident),* $(,)*]) => {
        __impl_eq_fields!($name: (_a, _b) => {
            $(
                assert_eq_fields!(_a.$field, _b.$field);
            )*
        });
    }
}
