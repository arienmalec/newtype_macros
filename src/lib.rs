#![no_implicit_prelude]
#![deny(missing_docs)]

//! Rust Newtype Macros
//!
//! This library provides two macros: `newtype_derive!` and `newtype!`.
//! The first operates on an existing newtype while the second creates the newtype.
//!
//! ## Limitations
//! Does not work at all with references (because of the need to declare lifetime specifiers)


/// Expands to a set of trait implementations for a newtype definition.
///
/// Supports the following traits:
/// - From -- converts from the wrapped type to the newtype
/// - Into -- consumes the alias type and returns the wrapped type
/// - Deref -- provides a reference to the wrapped type
/// - DerefMut -- provides a mutable reference to the wrapped type
/// - Display -- delegates to the wrapped type for display
/// - The following arithmetic traits which delegate to the wrapped type
///   (and which require implementations of From and Into):
/// -- Add
/// -- Sub
/// -- Mul
/// -- Div
/// -- Neg
///
/// # Examples
/// ```
/// # #[macro_use] extern crate newtype_macros;
/// # fn main() {
/// struct Miles(u32);
/// newtype_derive!(Miles(u32): Display, From, Into, Deref);
///
/// let m = Miles::from(14);
/// let m2: Miles = 14.into();
/// assert_eq!(*m, 14);
/// assert_eq!(*m2, 14);
/// assert_eq!(String::from("14"), format!("{}", m));
/// # }
/// ```
#[macro_export]
macro_rules! newtype_derive {
    ($alias:ident($t:ty): ) => { };
    ($alias:ident($t:ty): Deref) => {
        impl ::std::ops::Deref for $alias {
            type Target = $t;
            fn deref<'a>(&'a self) -> &'a $t {
                let &$alias(ref v) = self;
                v
            }
        }
    };
    ($alias:ident($t:ty): DerefMut) => {
        impl ::std::ops::DerefMut for $alias {
            fn deref_mut<'a>(&'a mut self) -> &'a mut $t {
                let &mut $alias(ref mut v) = self;
                v
            }
        }
    };
    ($alias:ident($t:ty): From) => {
        impl ::std::convert::From<$t> for $alias {
            fn from(v: $t) -> Self {
                $alias(v)
            }
        }
    };
    ($alias:ident($t:ty): Into) => {
        impl ::std::convert::Into<$t> for $alias {
            fn into(self) -> $t {
                let $alias(v) = self;
                v
            }
        }
    };
    ($alias:ident($t:ty): Display) => {
        impl ::std::fmt::Display for $alias {
             fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let $alias(ref v) = *self;
                <$t as ::std::fmt::Display>::fmt(v, f)
            }
        }
    };
    ($alias:ident($t:ty): Add) => {
        impl ::std::ops::Add for $alias {
            type Output = $alias;
            fn add(self, rhs: $alias) -> Self {
                let l = ::std::convert::Into::<$t>::into(self);
                let r = ::std::convert::Into::<$t>::into(rhs);
                ::std::convert::From::<$t>::from(l.add(r))
            }
        }
    };
    ($alias:ident($t:ty): Sub) => {
        impl ::std::ops::Sub for $alias {
            type Output = $alias;
            fn sub(self, rhs: $alias) -> Self {
                let l = ::std::convert::Into::<$t>::into(self);
                let r = ::std::convert::Into::<$t>::into(rhs);
                ::std::convert::From::<$t>::from(l.sub(r))
            }
        }
    };
    ($alias:ident($t:ty): Mul) => {
        impl ::std::ops::Mul for $alias {
            type Output = $alias;
            fn mul(self, rhs: $alias) -> Self {
                let l = ::std::convert::Into::<$t>::into(self);
                let r = ::std::convert::Into::<$t>::into(rhs);
                ::std::convert::From::<$t>::from(l.mul(r))
            }
        }
    };
    ($alias:ident($t:ty): Div) => {
        impl ::std::ops::Div for $alias {
            type Output = $alias;
            fn div(self, rhs: $alias) -> Self {
                let l = ::std::convert::Into::<$t>::into(self);
                let r = ::std::convert::Into::<$t>::into(rhs);
                ::std::convert::From::<$t>::from(l.div(r))
            }
        }
    };
    ($alias:ident($t:ty): Rem) => {
        impl ::std::ops::Rem for $alias {
            type Output = $alias;
            fn rem(self, rhs: $alias) -> Self {
                let l = ::std::convert::Into::<$t>::into(self);
                let r = ::std::convert::Into::<$t>::into(rhs);
                ::std::convert::From::<$t>::from(l.rem(r))
            }
        }
    };
    ($alias:ident($t:ty): Neg) => {
        impl ::std::ops::Neg for $alias {
            type Output = $alias;
            fn neg(self) -> Self {
                let v = ::std::convert::Into::<$t>::into(self);
                ::std::convert::From::<$t>::from(v.neg())
            }
        }
    };
    ($alias:ident($t:ty): $keyword:ident) => { unrecognized derive keyword };
    ($alias:ident($t:ty): $($keyword:ident),*) => {
        $(newtype_derive!($alias($t): $keyword);)*
    };
}

/// Expands to a newtype defintion with basic derives, and uses newtype_derive! to derive traits
///
/// Supports same traits as newtype_derive!
///
/// # Examples
/// ```
/// # #[macro_use] extern crate newtype_macros;
/// newtype!(#[derive(Debug, PartialEq)] pub struct Miles(u32): From, Into, Add);
/// # fn main() {
/// let m = Miles::from(14);
/// let m2 = Miles::from(20);
/// assert_eq!(Miles::from(34), m + m2);
/// # }
/// ```
#[macro_export]
macro_rules! newtype {
    ($(#[$meta:meta])* struct $alias:ident($t:ty): $($keyword:ident),*) => {
        $(#[$meta])*
        struct $alias($t);

        $(newtype_derive!($alias($t): $keyword);)*
    };
    ($(#[$meta:meta])* pub struct $alias:ident(pub $t:ty): $($keyword:ident),*) => {
        $(#[$meta])*
        pub struct $alias(pub $t);

        $(newtype_derive!($alias($t): $keyword);)*
    };
    ($(#[$meta:meta])* pub struct $alias:ident($t:ty): $($keyword:ident),*) => {
        $(#[$meta])*
        pub struct $alias($t);

        $(newtype_derive!($alias($t): $keyword);)*
    };
}

#[test]
#[allow(dead_code)]
fn test_no_prelude() {
    newtype!(struct M1(i32): Deref);
    newtype!(struct M2(i32): Deref, DerefMut);
    newtype!(struct M3(i32): From);
    newtype!(struct M4(i32): Into);
    newtype!(struct M5(i32): Display);
    newtype!(struct M6(i32): From, Into, Add);
    newtype!(struct M7(i32): From, Into, Sub);
    newtype!(struct M8(i32): From, Into, Mul);
    newtype!(struct M9(i32): From, Into, Div);
    newtype!(struct M10(i32): From, Into, Neg);
    newtype!(#[derive(Hash)] struct M11(i32): Deref);
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    #[test]
    fn test_newtype_derive() {
        struct Miles(u32);
        newtype_derive!(Miles(u32): Display, From, Into, Deref);
        let m = Miles::from(14);
        let m2: Miles = 14.into();
        assert_eq!(*m, 14);
        assert_eq!(*m2, 14);
        assert_eq!(String::from("14"), format!("{}", m));

    }

    #[test]
    fn test_newtype() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(u32): Display, From, Into, Deref);
        let m = Miles::from(14);
        let m2: Miles = 14.into();
        assert_eq!(*m,14);
        assert_eq!(*m2,14);
        assert_eq!(String::from("14"), format!("{}", m));

    }

    #[test]
    fn test_add() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(u32): From, Into, Add);
        let m = Miles::from(14);
        let m2 = Miles::from(20);
        assert_eq!(Miles::from(34), m + m2);
    }

    #[test]
    fn test_sub() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(u32): From, Into, Sub);
        let m = Miles::from(20);
        let m2 = Miles::from(14);
        assert_eq!(Miles::from(6), m - m2);
    }

    #[test]
    fn test_mul() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(u32): From, Into, Mul);
        let m = Miles::from(14);
        let m2 = Miles::from(20);
        assert_eq!(Miles::from(280), m * m2);
    }

    #[test]
    fn test_div() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(f64): From, Into, Div);
        let m = Miles::from(20f64);
        let m2 = Miles::from(5f64);
        assert_eq!(Miles::from(4f64), m / m2);
    }

    #[test]
    fn test_rem() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(f64): From, Into, Rem);
        let m = Miles::from(20f64);
        let m2 = Miles::from(5f64);
        assert_eq!(Miles::from(0f64), m % m2);
    }

    #[test]
    fn test_neg() {
        newtype!(#[derive(Debug, PartialEq)] struct Miles(i32): From, Into, Neg);
        let m = Miles::from(20);
        assert_eq!(Miles::from(-20), -m);
    }
}
