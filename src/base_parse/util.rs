use proc_macro2::{Ident, Punct, Spacing, Span};

pub fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

pub fn punct(ch: char) -> Punct {
    Punct::new(ch, Spacing::Joint)
}
