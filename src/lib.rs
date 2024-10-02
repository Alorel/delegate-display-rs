//! Lets you derive `Display` & `Debug` traits on types wrapping types that already implement them.
//!
//! [![master CI badge](https://img.shields.io/github/actions/workflow/status/Alorel/delegate-display-rs/ci.yml?label=master%20CI)](https://github.com/Alorel/delegate-display-rs/actions/workflows/ci.yml?query=branch%3Amaster)
//! [![crates.io badge](https://img.shields.io/crates/v/delegate-display)](https://crates.io/crates/delegate-display)
//! [![docs.rs badge](https://img.shields.io/docsrs/delegate-display?label=docs.rs)](https://docs.rs/delegate-display)
//! [![dependencies badge](https://img.shields.io/librariesio/release/cargo/delegate-display)](https://libraries.io/cargo/delegate-display)
//!
//! # Examples

//! <details><summary>Newtype structs</summary>
//!
//! ```
//! # use delegate_display::*;
//! struct SomeType;
//! impl core::fmt::Display for SomeType {
//!   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!     f.write_str(">foo<")
//!   }
//! }
//!
//! #[derive(DelegateDisplay)]
//! struct Foo(SomeType);
//!
//! assert_eq!(format!("{}", Foo(SomeType)), ">foo<");
//! ```
//!
//! </details>

//! <details><summary>Structs with 0..=1 fields</summary>
//!
//! ```
//! # use delegate_display::*;
//! struct SomeType;
//! impl core::fmt::Debug for SomeType {
//!   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!     f.write_str(">foo<")
//!   }
//! }
//!
//! #[derive(DelegateDebug)]
//! struct Foo { some_field: SomeType }
//!
//! assert_eq!(format!("{:?}", Foo { some_field: SomeType }), ">foo<");
//! ```
//!
//! </details>

//! <details><summary>Enums with 0..=1 variants each</summary>
//!
//! ```
//! # use delegate_display::*;
//! struct SomeType;
//! struct AnotherType;
//!
//! impl core::fmt::Display for SomeType {
//!   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!     f.write_str(">foo<")
//!   }
//! }
//! impl core::fmt::Display for AnotherType {
//!   fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!     f.write_str(">bar<")
//!   }
//! }
//!
//! #[derive(DelegateDisplay)]
//! enum MyEnum {
//!   Foo,
//!   Bar(SomeType),
//!   Qux { baz: AnotherType }
//! }
//!
//! assert_eq!(format!("{}", MyEnum::Bar(SomeType)), ">foo<");
//! assert_eq!(format!("{}", MyEnum::Qux { baz: AnotherType }), ">bar<");
//! ```
//!
//! </details>

//! <details><summary>Generics</summary>
//!
//! Generics are handled automatically for you
//!
//! ```
//! # use delegate_display::*;
//! #
//! #[derive(DelegateDisplay)]
//! struct MyStruct<T>(T);
//!
//! #[derive(DelegateDisplay)]
//! enum MyEnum<A, B> {
//!   A(A),
//!   B { value: B },
//! }
//!
//! assert_eq!(format!("{}", MyStruct(50)), "50");
//! assert_eq!(format!("{}", MyEnum::<u8, i8>::A(75)), "75");
//! assert_eq!(format!("{}", MyEnum::<u8, i8>::B { value: -1 }), "-1");
//! ```
//!
//! </details>

//! <details><summary>Structs & enums with 2+ fields</summary>
//!
//! The field being delegated to must be marked with the appropriate attribute.
//!
//! ```
//! # use delegate_display::*;
//!
//! #[derive(DelegateDisplay)]
//! struct MyStruct<T> {
//!   label: String,
//!   #[ddisplay]
//!   value: T,
//! }
//!
//! #[derive(DelegateDebug)]
//! enum MyEnum {
//!   Foo(#[ddebug] String, u8),
//!   Bar { baz: u8, #[ddebug] qux: u8 }
//! }
//!
//! let my_struct = MyStruct { label: "foo".into(), value: 42 };
//! assert_eq!(format!("{}", my_struct), "42");
//!
//! let my_enum = MyEnum::Foo(".".into(), 1);
//! assert_eq!(format!("{:?}", my_enum), "\".\"");
//!
//! let my_enum = MyEnum::Bar { baz: 2, qux: 3 };
//! assert_eq!(format!("{:?}", my_enum), "3");
//! ```
//!
//! </details>

//! <details><summary>Empty structs</summary>
//!
//! ```
//! # use delegate_display::*;
//! #
//! #[derive(DelegateDebug, DelegateDisplay)]
//! struct Foo;
//!
//! #[derive(DelegateDebug, DelegateDisplay)]
//! struct Bar{}
//!
//! #[derive(DelegateDebug, DelegateDisplay)]
//! struct Qux();
//!
//! assert_eq!(format!("{}-{:?}", Foo, Foo), "-");
//! assert_eq!(format!("{}-{:?}", Bar{}, Bar{}), "-");
//! assert_eq!(format!("{}-{:?}", Qux(), Qux()), "-");
//! ```
//!
//! </details>

//! <details><summary>Typed delegations</summary>
//!
//! Can be useful for further prettifying the output.
//!
//! ```
//! # use delegate_display::*;
//! #
//! /// Some type that `Deref`s to the type we want to use in our formatting, in this case, `str`.
//! #[derive(Debug)]
//! struct Wrapper(&'static str);
//! impl std::ops::Deref for Wrapper {
//!   type Target = str;
//!   fn deref(&self) -> &Self::Target {
//!     self.0
//!   }
//! }
//!
//! #[derive(DelegateDebug)]
//! #[ddebug(delegate_to(str))] // ignore `Wrapper` and debug the `str` it `Deref`s instead
//! struct Typed(Wrapper);
//!
//! #[derive(DelegateDebug)] // Included for comparison
//! struct Base(Wrapper);
//!
//! assert_eq!(format!("{:?}", Typed(Wrapper("foo"))), "\"foo\"");
//! assert_eq!(format!("{:?}", Base(Wrapper("bar"))), "Wrapper(\"bar\")");
//! ```
//!
//! </details>

//! <details><summary>Custom generic bounds</summary>
//!
//! ```
//! # use delegate_display::*;
//! # use core::fmt::{Display, Formatter, self};
//! # use std::ops::Deref;
//! #
//! struct CopyDisplayable<T>(T);
//!
//! impl<T> Deref for CopyDisplayable<T> {
//!   type Target = T;
//!   fn deref(&self) -> &Self::Target {
//!     &self.0
//!   }
//! }
//!
//! impl<T: Copy> Display for CopyDisplayable<T> {
//!   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//!     unimplemented!("Nonsense generic bound - base bounds don't work.");
//!   }
//! }
//!
//! // Without these options the implementation would have a predicate of `CopyDisplayable<T>: Debug` which would
//! // effectively mean `T: Copy`; we can transform it to `T: Display` because `CopyDisplayable` derefs to `T`.
//! #[derive(DelegateDisplay)]
//! #[ddisplay(bounds(T: Display), delegate_to(T))]
//! struct Displayable<T>(CopyDisplayable<T>);
//!
//! let dbg = Displayable::<String>(CopyDisplayable("cdbg".into()));
//! assert_eq!(format!("{}", dbg), "cdbg");
//! ```
//!
//! </details>

//! <details><summary>Invalid inputs</summary>
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! struct TooManyFields1 {
//!   foo: u8,
//!   bar: u8, // No fields marked with `#[ddebug]` or `#[dboth]`
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! struct TooManyFields2(u8, u8); // No fields marked with `#[ddebug]` or `#[dboth]`
//! ```
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! enum SomeEnum {
//!   A, // this is ok
//!   B(u8), // this is ok
//!   C { foo: u8 }, // this is ok
//!   D(u8, u8), // ERR: No fields marked with `#[ddebug]` or `#[dboth]`
//!   E { foo: u8, bar: u8 } // ERR: No fields marked with `#[ddebug]` or `#[dboth]`
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! union Foo { bar: u8 } // Unions are not supported
//! ```
//!
//! ```compile_fail
//! # use delegate_display::*;
//! #
//! struct NonDebug;
//!
//! #[derive(DelegateDebug)]
//! struct Foo<A, B>(A, B);
//!
//! format!("{:?}", Foo(NonDebug, 1)); // NonDebug does not implement Debug
//! ```
//!
//! </details>

#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(
    clippy::wildcard_imports,
    clippy::default_trait_access,
    clippy::single_match_else
)]
#![warn(missing_docs)]

use proc_macro::TokenStream as TokenStream1;

mod implementation;
use implementation::Implementation;

const ATTR_BOTH: &str = "dboth";

/// Derive the [`Debug`](core::fmt::Debug) trait.
///
/// Its config attribute is `ddebug` or `dboth` can be used to configure both
/// [`Debug`](core::fmt::Debug) & [`Display`](core::fmt::Display).
///
/// See [crate-level documentation](crate) for information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDebug, attributes(ddebug, dboth))]
#[inline]
pub fn derive_debug(tokens: TokenStream1) -> TokenStream1 {
    Implementation::exec(tokens, "ddebug", "Debug")
}

/// Derive the [`Display`](core::fmt::Display) trait.
///
/// Its config attribute is `ddebug` or `dboth` can be used to configure both
/// [`Debug`](core::fmt::Debug) & [`Display`](core::fmt::Display).
///
/// See [crate-level documentation](crate) for information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDisplay, attributes(ddisplay, dboth))]
#[inline]
pub fn derive_display(tokens: TokenStream1) -> TokenStream1 {
    Implementation::exec(tokens, "ddisplay", "Display")
}
