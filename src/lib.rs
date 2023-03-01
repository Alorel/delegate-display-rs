//! Lets you derive [Display](std::fmt::Display) & [Debug](std::fmt::Debug) traits on structs with
//! `0..1` fields & enums where each variant has `0..1` fields - see input/output examples below.
//!
//! # Newtype structs
//!
//! ```
//! # use std::fmt;
//! # type SomeType = u8;
//! #
//! // Input
//! #[derive(delegate_display::DelegateDisplay)]
//! # struct TheOlSwitcheroo;
//! struct Foo(SomeType);
//!
//! // Output
//! impl fmt::Display for Foo {
//!   #[inline]
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     fmt::Display::fmt(&self.0, f)
//!   }
//! }
//! ```
//!
//! # Structs with one field
//!
//! ```
//! # type SomeType = u8;
//! # use std::fmt;
//! #
//! // Input
//! #[derive(delegate_display::DelegateDebug)]
//! # struct TheOlSwitcheroo;
//! struct Foo { some_field: SomeType }
//!
//! // Output
//! impl fmt::Debug for Foo {
//!   #[inline]
//!   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!     fmt::Debug::fmt(&self.some_field, f)
//!   }
//! }
//! ```
//!
//! # Enums
//!
//! ```
//! # #[derive(Debug)]
//! # struct SomeType;
//! # struct DebugOrDisplay;
//! # impl DebugOrDisplay {
//! #  fn fmt(_: &SomeType, _: &mut fmt::Formatter<'_>) -> fmt::Result { Ok(()) }
//! # }
//! # use std::fmt;
//! #
//! // Input
//! enum MyEnum {
//!   Foo,
//!   Bar(SomeType),
//!   Qux { baz: SomeType }
//! }
//!
//! // Output
//! # impl fmt::Debug for MyEnum {
//! fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!   match self {
//!     Self::Foo => f.write_str("Foo"),
//!     Self::Bar(inner) => DebugOrDisplay::fmt(inner, f),
//!     Self::Qux { baz } => DebugOrDisplay::fmt(baz, f),
//!   }
//! }
//! # }
//! ```
//!
//! # Empty structs & enums
//!
//! ```
//! # use std::fmt;
//! #
//! // Input
//! struct Foo;
//! struct Bar{}
//! struct Qux();
//! enum Baz {}
//!
//! // Output
//! # impl fmt::Debug for Foo {
//! fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
//!   Ok(())
//! }
//! # }
//! ```
//!
//! # Invalid inputs
//!
//! ```
//! # type SomeType = u8;
//! #
//! struct TooManyFields1 {
//!   foo: SomeType,
//!   bar: SomeType, // Only one field permitted
//! }
//!
//! struct TooManyFields2(SomeType, SomeType);
//!
//! enum SomeEnum {
//!   A, // this is ok
//!   B(SomeType), // this is ok
//!   C { foo: SomeType }, // this is ok
//!   D(SomeType, SomeType), // Only one field permitted
//!   E { foo: SomeType, bar: SomeType } // Only one field permitted
//! }
//! ```

use proc_macro::TokenStream as BaseTokenStream;

use base_parse::BaseParse;

mod base_parse;

/// Derive the [Debug](std::fmt::Debug) trait - see [module-level documentation](self) for
/// information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDebug)]
#[inline]
pub fn derive_debug(tokens: BaseTokenStream) -> BaseTokenStream {
    BaseParse::for_trait("Debug", tokens)
}

/// Derive the [Display](std::fmt::Display) trait - see [module-level documentation](self) for
/// information on what's acceptable and what's not.
#[proc_macro_derive(DelegateDisplay)]
#[inline]
pub fn derive_display(tokens: BaseTokenStream) -> BaseTokenStream {
    BaseParse::for_trait("Display", tokens)
}
