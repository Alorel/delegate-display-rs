use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

use super::util::{ident, punct};

/// Basically prepends the ident with `std::fmt::`
pub struct TraitName(pub Ident);

impl ToTokens for TraitName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        {
            let col = punct(':');
            ident("std").to_tokens(tokens);
            col.to_tokens(tokens);
            col.to_tokens(tokens);
            ident("fmt").to_tokens(tokens);
            col.to_tokens(tokens);
            col.to_tokens(tokens);
        }
        self.0.to_tokens(tokens)
    }
}
