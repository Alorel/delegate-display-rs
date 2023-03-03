use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};

use super::util::{ident, punct};

/// Basically prepends the ident with `core::fmt::`
pub struct TraitName(pub Ident);

impl ToTokens for TraitName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let col = punct(':');
        tokens.append(ident("core"));
        col.to_tokens(tokens);
        col.to_tokens(tokens);
        tokens.append(ident("fmt"));
        col.to_tokens(tokens);
        tokens.append(col);
        self.0.to_tokens(tokens)
    }
}
