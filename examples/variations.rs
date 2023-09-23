fn main() {
    panic!("Run me with `cargo test --example variations`")
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Display};

    use static_assertions::assert_impl_all;

    use delegate_display::*;

    macro_rules! check {
        ($src: expr; $exp_display: expr, $exp_debug: expr) => {
            assert_eq!($src.to_string(), $exp_display, "Display");
            assert_eq!(format!("{:?}", $src), $exp_debug, "Debug");
        };
        ($src: expr => $exp: expr) => {
            check!($src; $exp, $exp);
        };
    }

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

            assert_eq!(format!("{:?}", Generic::A(true)), "true");
        }
    }

    mod structs {
        use super::*;

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
            struct BasicGenerics<T: Debug, const N: usize> {
                _data: [T; N],
            }

            let inst = BasicGenerics { _data: [5] };
            assert_eq!(format!("{:?}", inst), "[5]");
        }
    }
}
