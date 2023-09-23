//! Lets you derive [`Display`](::core::fmt::Display) & [`Debug`](::core::fmt::Debug) traits on structs with
//! `0..1` fields & enums where each variant has `0..1` fields - see input/output examples below.
//!
//! [![master CI badge](https://img.shields.io/github/actions/workflow/status/Alorel/delegate-display-rs/ci.yml?label=master%20CI)](https://github.com/Alorel/delegate-display-rs/actions/workflows/ci.yml?query=branch%3Amaster)
//! [![crates.io badge](https://img.shields.io/crates/v/delegate-display)](https://crates.io/crates/delegate-display)
//! [![docs.rs badge](https://img.shields.io/docsrs/delegate-display?label=docs.rs)](https://docs.rs/delegate-display)
//! [![dependencies badge](https://img.shields.io/librariesio/release/cargo/delegate-display)](https://libraries.io/cargo/delegate-display)
//!
//! # Newtype structs
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! // Input
//! #[derive(delegate_display::DelegateDisplay)]
//! struct Foo(SomeType);
//!
//! // Output
//! impl fmt::Display for Foo {
//!   #[inline]
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     fmt::Display::fmt(&self.0, f)
//!   }
//! }
//! ````
//!
//! # Structs with one field
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! // Input
//! #[derive(delegate_display::DelegateDebug)]
//! struct Foo { some_field: SomeType }
//!
//! // Output
//! impl fmt::Debug for Foo {
//!   #[inline]
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     fmt::Debug::fmt(&self.some_field, f)
//!   }
//! }
//! ````
//!
//! # Enums
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! // Input
//! enum MyEnum {
//!   Foo,
//!   Bar(SomeType),
//!   Qux { baz: SomeType }
//! }
//!
//! // Output
//! fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!   match self {
//!     Self::Foo => f.write_str("Foo"),
//!     Self::Bar(inner) => DebugOrDisplay::fmt(inner, f),
//!     Self::Qux { baz } => DebugOrDisplay::fmt(baz, f),
//!   }
//! }
//! ````
//!
//! # Empty structs & enums
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! // Input
//! struct Foo;
//! struct Bar{}
//! struct Qux();
//! enum Baz {}
//!
//! // Output
//! fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
//!   Ok(())
//! }
//! ````
//!
//! # Invalid inputs
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! struct TooManyFields1 {
//!   foo: u8,
//!   bar: u8, // Only one field permitted
//! }
//! ```
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! struct TooManyFields2(u8, u8); // too many fields
//! ```
//!
//! ```compile_fail
//! #[derive(delegate_display::DelegateDebug)]
//! enum SomeEnum {
//!   A, // this is ok
//!   B(u8), // this is ok
//!   C { foo: u8 }, // this is ok
//!   D(u8, u8), // Only one field permitted
//!   E { foo: u8, bar: u8 } // Only one field permitted
//! }
//! ```

#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::default_trait_access)]
#![warn(missing_docs)]

use proc_macro::TokenStream as BaseTokenStream;

mod parse;
mod tokenise;

/// Derive the [Debug](core::fmt::Debug) trait - see [module-level documentation](self) for
/// information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDebug)]
#[inline]
pub fn derive_debug(tokens: BaseTokenStream) -> BaseTokenStream {
    ParsedData::process("Debug", tokens)
}

/// Derive the [Display](core::fmt::Display) trait - see [module-level documentation](self) for
/// information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDisplay)]
#[inline]
pub fn derive_display(tokens: BaseTokenStream) -> BaseTokenStream {
    ParsedData::process("Display", tokens)
}

struct ParsedData {
    ident: syn::Ident,
    generics: syn::Generics,
    first_field: FirstField,
}

enum FieldLike {
    Indexed,
    Ident(syn::Ident),
}
type EnumData = (syn::Ident, Option<FieldLike>);
enum FirstField {
    Struct(Option<FieldLike>),
    Enum(Vec<EnumData>),
}
