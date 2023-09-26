fn main() {
    panic!("Run me with `cargo test --example variations`")
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Display};

    use static_assertions::assert_impl_all;

    use delegate_display::*;

    #[derive(Debug)]
    struct DebugOnly;

    macro_rules! check {
        (dbg $src: expr => $exp: expr) => {
            assert_eq!(format!("{:?}", $src), $exp, "Debug");
        };
        ($src: expr; $exp_display: expr, $exp_debug: expr) => {
            assert_eq!($src.to_string(), $exp_display, "Display");
            check!(dbg $src => $exp_debug);
        };
        ($src: expr => $exp: expr) => {
            check!($src; $exp, $exp);
        };
    }

    trait Displayable: Debug + Display {}
    impl<T: Debug + Display> Displayable for T {}

    mod enums {
        use super::*;

        #[derive(DelegateDebug, DelegateDisplay)]
        enum Empty {}
        assert_impl_all!(Empty: Debug, Display); // Can only really compile-check these

        #[derive(DelegateDebug, DelegateDisplay)]
        enum Variants {
            A,
            B(u8),
            C { d: &'static str },
        }

        #[derive(DelegateDisplay, DelegateDebug)]
        #[dboth(base_bounds)]
        #[ddebug(bounds(A: Displayable, B: Debug))]
        enum GenericB<A, B> {
            A(A),
            B(B),
        }

        #[test]
        fn variant_unit() {
            check!(Variants::A => "A");
        }

        #[test]
        fn variant_tuple() {
            check!(Variants::B(37) => "37");
        }

        #[test]
        fn variant_field() {
            check!(Variants::C { d: "deep down" }; "deep down", r#""deep down""#);
        }

        #[test]
        fn generic() {
            #[derive(DelegateDebug)]
            enum Generic<T: Debug> {
                A(T),
            }

            check!(dbg Generic::A(true) => "true");
        }

        #[test]
        fn generic_bounds_display() {
            check!(GenericB::<u8, u8>::A(5) => "5");
        }

        #[test]
        fn generic_bounds_debug() {
            check!(dbg GenericB::<u8, DebugOnly>::B(DebugOnly) => "DebugOnly");
        }
    }

    mod structs {
        use super::*;

        #[derive(DelegateDebug, DelegateDisplay)]
        #[dboth(bounds(T: Debug))]
        #[ddisplay(base_bounds)]
        struct GenericB<T>(T);

        #[test]
        fn unit() {
            #[derive(DelegateDebug, DelegateDisplay)]
            struct Unit;

            check!(Unit => "");
        }

        #[test]
        fn empty() {
            #[derive(DelegateDebug, DelegateDisplay)]
            struct Empty {}

            check!(Empty {} => "");
        }

        #[test]
        fn empty_newtype() {
            #[derive(DelegateDebug, DelegateDisplay)]
            struct EmptyNewtype();

            check!(EmptyNewtype() => "");
        }

        #[test]
        fn newtype() {
            #[derive(DelegateDebug, DelegateDisplay)]
            #[repr(transparent)]
            struct Newtype(String);

            const SRC: &'static str = "Lick the frog";

            check!(Newtype(SRC.into()); SRC, format!("{:?}", SRC));
        }

        #[test]
        fn fields() {
            #[derive(DelegateDebug, DelegateDisplay)]
            struct Basic {
                foo: i8,
            }

            check!(Basic { foo: -100 } => "-100");
        }

        #[test]
        fn generics() {
            #[derive(DelegateDebug)]
            struct Generics<T: Debug, const N: usize> {
                _data: [T; N],
            }

            check!(dbg Generics { _data: [5] } => "[5]");
        }

        #[test]
        fn generic_bounds() {
            check!(GenericB(5) => "5");
        }

        #[test]
        fn generic_debug_only() {
            check!(dbg GenericB(DebugOnly) => "DebugOnly");
        }
    }
}
