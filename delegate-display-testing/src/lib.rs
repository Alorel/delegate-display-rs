//! Just check that these compile to valid output

#![allow(dead_code)]

pub mod enums {
    use delegate_display::*;

    #[derive(DelegateDisplay, DelegateDebug)]
    pub enum EA {}

    #[derive(DelegateDisplay, DelegateDebug)]
    pub enum EB {
        A,
        B(EA),
        C { x: super::structs::SE },
    }

    #[derive(DelegateDebug)]
    pub enum EC<T: core::fmt::Debug> {
        A,
        B(T),
        C { val: String },
    }
}

pub mod structs {
    use delegate_display::*;

    #[derive(DelegateDisplay, DelegateDebug)]
    pub struct SA;

    #[derive(DelegateDisplay, DelegateDebug)]
    pub struct SB {}

    #[derive(DelegateDisplay, DelegateDebug)]
    pub struct SC {}

    #[derive(DelegateDisplay, DelegateDebug)]
    pub struct SD(SA);

    #[derive(DelegateDisplay, DelegateDebug)]
    pub struct SE {
        x: SB,
    }

    #[derive(DelegateDisplay)]
    pub struct SF<T>(T)
    where
        T: core::fmt::Display;

    #[derive(DelegateDisplay)]
    pub struct SG<T: core::fmt::Display = u8>(T);

    pub struct SHInner<T: core::fmt::Display, const C: usize>([T; C]);
    impl<T: core::fmt::Display, const C: usize> core::fmt::Display for SHInner<T, C> {
        fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            Ok(())
        }
    }

    #[derive(DelegateDisplay)]
    pub struct SH<T: core::fmt::Display, const C: usize = 5>(SHInner<T, C>);
}
