# Rust Newtype Macros

## Motivation

Rust newtypes are tuple `struct`s with a single member, and are intended to be used for wrapping types to create new semantics for an underlying type. For example, you might model meters as:

```rust
struct Meter(u32); 
```

By themselves, however, such newtypes are somewhat inconvenient to work with. There are a number of Rust traits that make working with newtypes much more convenient. For example, `From` and `Into` provide the Rust idiom for type conversions; `Deref` and `Deref` conversion allow easy access and delegation to the inner type member, and the other arithmetic operations in `std::ops` allow for mathematical operations on newtypes.

Unfortunately, Rust does not currently allow `#[derive(From)]` or similar annotations. There is [a tracking RFC](https://github.com/rust-lang/rfcs/issues/261) for making newtypes more convenient. Implementing the basic traits on newtypes is often a tedious exercise in cut and paste, particularly if you have a large set of newtypes to create.

Many people have created macros to automate this, and perform the equivalent of `[#derive]`. This repository tries to encapsulate a single reasonable way of automatically creating newtypes and newtype traits.

## Usage

This library provides two macros: `newtype_derive!` and `newtype!`. The first operates on an existing newtype definition and allows configurable derivation of the traits `Deref`, `From`, `Into`, `Display`, `Add`, `Sub`, `Mul`, `Div`, and `Neg`. The second creates the newtype, provides basic Rust derives (`Debug` and partial equality/ordering).

`From`, `Into`, and `Deref` provide (respectively) basic conversion from/to the underlying value, and provide access to a reference to the underlying value value. The other defaults delegate to the underlying value for display and arithmetic operations.



```rust
#[macro_use]
#[macro_use]
extern crate newtype_macros;
use std::convert::{From,Into};
use std::ops::{Add,Mul,Sub,Div,Neg,Deref};
use std::fmt::{self,Display};

fn main() {
	newtype!(Miles,u32,Display,From,Into,Deref,Add);
	let m = Miles::from(14);
	let m2:Miles = 7.into();
	assert_eq!(*m,14);
	assert_eq!(*m2,7);
	print!("{} miles ",m);
	print!("plus {} miles ",m2);
	println!("is {} miles", m+m2);
}
```

Arithmetic functions use `From` and `Into` to perform conversion to/from the underlying value. This is to allow, for example, preliminary conversion to be implemented via `From` and `Into` before delegating to the underlying type to perform the operations. Therefore, to add automatic derivations of these traits, either `From` and `Into` must also be derived, or must be manually implemented.

```rust
newtype!(Miles,u32,From,Into,Add);
let m = Miles::from(500);
let m2 = Miles::from(500);
print!("I would walk {} miles/",m);
print!("and I would walk {} more/,m2);
println("just to be the man who walked a {} miles/to fall down at your door", m+m2);
```

## Limitations

Hardwires the `#[define]`s in the `newtype!` macro. 
Does not work at all with references (because of the need to declare lifetime specifiers)
