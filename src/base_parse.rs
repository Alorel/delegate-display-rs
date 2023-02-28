use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Generics};

use first_field::FirstField;
use trait_name::TraitName;
use util::ident;

use crate::BaseTokenStream;

mod first_field;
mod trait_name;
mod util;

pub(crate) struct BaseParse {
    ident: Ident,
    generics: Generics,
    first_field: FirstField,
}

impl BaseParse {
    pub fn for_trait(name: &str, input: BaseTokenStream) -> BaseTokenStream {
        parse_macro_input!(input as BaseParse)
            .to_tokens_as(name)
            .into()
    }

    fn to_tokens_as(&self, trait_name: &str) -> TokenStream {
        let trait_name = TraitName(ident(trait_name));
        let src_struct = &self.ident;
        let (gen_impl, gen_type, gen_where) = self.generics.split_for_impl();

        let inline_declaration = if self.first_field.is_inlinable() {
            Some(quote! { #[inline] })
        } else {
            None
        };

        let (formatter_name, body) = match self.first_field.to_tokens_opt(&trait_name) {
            Some(b) => (ident("f"), b),
            None => (ident("_"), quote! { Ok(()) }),
        };

        quote! {
            #[automatically_derived]
            impl #gen_impl #trait_name for #src_struct #gen_type #gen_where {
                #inline_declaration
                fn fmt(&self, #formatter_name: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #body
                }
            }
        }
    }
}

impl Parse for BaseParse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = input.parse::<DeriveInput>()?;
        Ok(Self {
            first_field: input.data.try_into()?,
            generics: input.generics,
            ident: input.ident,
        })
    }
}

#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;

    mod should_error_on {
        use quote::quote;

        fn errs(inp: super::TokenStream) {
            assert!(syn::parse2::<super::super::BaseParse>(inp).is_err());
        }

        #[test]
        fn multi_field_idx_struct() {
            errs(quote! { struct X(u8, u8); })
        }

        #[test]
        fn multi_field_named_struct() {
            errs(quote! { struct X { x: u8, y: u8 } })
        }

        #[test]
        fn multi_field_unnamed_enum() {
            errs(quote! {
                enum X {
                    A,
                    B(u8, u16),
                }
            });
        }

        #[test]
        fn multi_field_named_enum() {
            errs(quote! {
                enum X {
                    A,
                    B { x: String, y: u8 },
                }
            });
        }
    }
}
