//! Just check that these compile to valid output

#![allow(dead_code)]

use delegate_display::*;

#[derive(DelegateDisplay, DelegateDebug)]
struct SA;

#[derive(DelegateDisplay, DelegateDebug)]
struct SB {}

#[derive(DelegateDisplay, DelegateDebug)]
struct SC {}

#[derive(DelegateDisplay, DelegateDebug)]
struct SD(SA);

#[derive(DelegateDisplay, DelegateDebug)]
struct SE {
    x: SB,
}

#[derive(DelegateDisplay)]
struct SF<T>(T)
where
    T: core::fmt::Display;

#[derive(DelegateDisplay, DelegateDebug)]
enum EA {}

#[derive(DelegateDisplay, DelegateDebug)]
enum EB {
    A,
    B(EA),
    C { x: SE },
}

#[derive(DelegateDebug)]
enum EC<T: core::fmt::Debug> {
    A,
    B(T),
    C { val: String },
}
