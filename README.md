Lets you derive `Display` and `Debug` traits by delegating them to the only member of `0..1`-member structs & enums.

[![master CI badge](https://img.shields.io/github/actions/workflow/status/Alorel/delegate-display-rs/ci.yml?label=master%20CI)](https://github.com/Alorel/delegate-display-rs/actions/workflows/ci.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/delegate-display)](https://crates.io/crates/delegate-display)
[![docs.rs badge](https://img.shields.io/docsrs/delegate-display?label=docs.rs)](https://docs.rs/delegate-display)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/delegate-display)](https://libraries.io/cargo/delegate-display)

```rust
use delegate_display::{DelegateDebug, DelegateDisplay};
use std::fmt;

// Input
#[derive(DelegateDebug, DelegateDisplay)]
enum MyEnum {
  Foo,
  Bar(SomeType),
  Qux { baz: SomeType }, 
}

// Generated output
impl fmt::Display for MyEnum {
  // Equivalent implementation for Debug & for structs
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Foo => f.write_str("Foo"),
      Self::Bar(value) => fmt::Display::fmt(value, f),
      Self::Qux { baz } => fmt::Display::fmt(baz, f),    
    }
  }
}
```

See [module-level documentation](https://docs.rs/delegate-display) for more examples, what's allowed and what isn't. 
