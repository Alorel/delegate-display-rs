use macroific::elements::{ImplFor, ModulePrefix, SimpleAttr};
use macroific::prelude::*;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_quote, Token, Type, WherePredicate};

use crate::{BaseTokenStream, EnumData, FieldLike, FirstField, ParsedData};

impl ParsedData {
    pub fn process(trait_name: &str, tokens: BaseTokenStream) -> BaseTokenStream {
        let parsed = match Self::parse(tokens, trait_name) {
            Ok(p) => p,
            Err(e) => return e.to_compile_error().into(),
        };

        parsed.into_token_stream(trait_name).into()
    }

    fn into_token_stream(self, trait_basename: &str) -> TokenStream {
        let Self {
            ident,
            mut generics,
            first_field,
            options,
        } = self;

        let trait_name = ["core", "fmt", trait_basename];
        let trait_name = ModulePrefix::new(&trait_name);

        if options.bounds.is_empty() {
            let predicates = first_field.predicates();
            if !predicates.is_empty() {
                let iter = predicates
                    .into_iter()
                    .map::<WherePredicate, _>(move |ty| parse_quote!(#ty: #trait_name));
                generics.make_where_clause().predicates.extend(iter);
            }
        } else {
            generics
                .make_where_clause()
                .predicates
                .extend(options.bounds);
        }

        let mut out = SimpleAttr::AUTO_DERIVED.into_token_stream();

        ImplFor::new(&generics, &trait_name, &ident).to_tokens(&mut out);

        let mut inner = if first_field.is_inlinable() {
            SimpleAttr::INLINE.into_token_stream()
        } else {
            TokenStream::new()
        };

        inner.append(Ident::create("fn"));
        inner.append(Ident::create("fmt"));

        let (formatter_name, body) = match first_field.into_tokens(&trait_name, options.delegate_to)
        {
            Some(b) => (Ident::create("f"), b),
            None => {
                let res = &ModulePrefix::RESULT;
                (Ident::create("_"), quote! { #res::Ok(()) })
            }
        };

        inner.append(Group::new(
            Delimiter::Parenthesis,
            quote! {
                &self, #formatter_name: &mut ::core::fmt::Formatter<'_>
            },
        ));

        <Token![->]>::default().to_tokens(&mut inner);
        ModulePrefix::new(&["core", "fmt", "Result"]).to_tokens(&mut inner);

        inner.append(Group::new(Delimiter::Brace, body));

        out.append(Group::new(Delimiter::Brace, inner));

        out
    }
}

impl FieldLike {
    fn as_type(&self) -> &Type {
        match self {
            Self::Indexed(ty) | Self::Ident(_, ty) => ty,
        }
    }
}

impl FirstField {
    /// Whether we should include `#[inline]` or not
    fn is_inlinable(&self) -> bool {
        match *self {
            Self::Struct(_) => true,
            Self::Enum(ref v) => v.is_empty(),
        }
    }

    pub fn predicates(&self) -> Vec<&Type> {
        match self {
            Self::Struct(Some(field)) => vec![field.as_type()],
            Self::Enum(variants) => variants
                .iter()
                .filter_map(move |(_, field)| field.as_ref().map(|f| f.as_type()))
                .collect(),
            Self::Struct(None) => Vec::new(),
        }
    }

    /// Like [`ToTokens::to_token_stream`], but accepts the trait name to derive for
    ///
    /// # Returns
    ///
    /// `Some` if we should call `fmt()`, `None` if we shouldn't
    pub fn into_tokens(
        self,
        trait_name: &impl ToTokens,
        delegate_to: Option<syn::Type>,
    ) -> Option<TokenStream> {
        Some(match self {
            Self::Struct(None) => return None,
            Self::Struct(Some(data)) => {
                let mut out = if let Some(delegate_to) = delegate_to {
                    quote! { <#delegate_to as #trait_name> }
                } else {
                    trait_name.to_token_stream()
                };

                out.extend(quote!(::fmt(&#data, f)));

                out
            }
            Self::Enum(data) => {
                if data.is_empty() {
                    return None;
                }

                Self::tokenise_enum(trait_name, data)
            }
        })
    }

    /// Non-empty `enum` handler for [`Self::to_tokens_opt`]
    fn tokenise_enum(trait_name: &impl ToTokens, data: Vec<EnumData>) -> TokenStream {
        let mut out = TokenStream::new();
        out.append(Ident::create("match"));
        out.append(Ident::create("self"));

        let mut body = TokenStream::new();
        body.append_separated(
            data.into_iter().map(move |(variant_name, first_field)| {
                let mut out = quote!(Self::);

                let first_field = match first_field {
                    Some(field) => *field,
                    None => {
                        let lit = Literal::string(&variant_name.to_string()).into_token_stream();
                        out.append(variant_name);

                        <Token![=>]>::default().to_tokens(&mut out);
                        out.append(Ident::create("f"));
                        out.append(Punct::new_joint('.'));
                        out.append(Ident::create("write_str"));
                        out.append(Group::new(Delimiter::Parenthesis, lit));

                        return out;
                    }
                };

                out.append(variant_name);

                match first_field {
                    FieldLike::Ident(id, _) => {
                        let mut stream = TokenStream::new();
                        stream.append(id);
                        stream.append(Punct::new_alone(':'));
                        stream.append(Ident::create("inner"));

                        out.append(Group::new(Delimiter::Brace, stream));
                    }
                    FieldLike::Indexed(_) => {
                        let stream = quote!(inner);
                        out.append(Group::new(Delimiter::Parenthesis, stream));
                    }
                }

                out.extend(quote!(=> #trait_name::fmt(inner, f)));

                out
            }),
            Punct::new_joint(','),
        );

        out.append(Group::new(Delimiter::Brace, body));

        out
    }
}

impl ToTokens for FieldLike {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::create("self"));
        tokens.append(Punct::new_joint('.'));

        match *self {
            Self::Indexed(_) => tokens.append(Literal::usize_unsuffixed(0)),
            Self::Ident(ref id, _) => id.to_tokens(tokens),
        }
    }
}
