//!
//! Declares `is_not_empty` trait (opposite of `is_empty()` call present on most data structures).
//!

use std::collections::{HashMap, HashSet, VecDeque};

/// Trait that declares `is_not_empty()` method.
pub trait IsNotEmpty {
    fn is_not_empty(&self) -> bool;
}

macro_rules! is_not_empty {
    ($type:ty) => {
        impl IsNotEmpty for $type {
            #[inline(always)]
            fn is_not_empty(&self) -> bool {
                !self.is_empty()
            }
        }
    };
}

macro_rules! is_not_empty_generic {
    ($type:ident<$($t:ident),+>) => (
        impl<$($t),+> IsNotEmpty for $type<$($t),+> {
            #[inline(always)]
            fn is_not_empty(&self) -> bool {
                !self.is_empty()
            }
        }
    )
}

is_not_empty!(&str);
is_not_empty!(String);
is_not_empty_generic!(Vec<T>);
is_not_empty_generic!(VecDeque<T>);
is_not_empty_generic!(HashMap<K,V>);
is_not_empty_generic!(HashSet<T>);
