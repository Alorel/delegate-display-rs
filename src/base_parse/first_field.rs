use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DataStruct, Error, Field, Fields};

use super::trait_name::TraitName;
use super::util::{ident, punct};

pub enum FieldLike {
    Ident(Ident),
    Indexed,
}

type EnumData = Vec<(Ident, Option<FieldLike>)>;
pub enum FirstField {
    Struct(Option<FieldLike>),
    Enum(EnumData),
}

impl FirstField {
    /// Whether we should include `#[inline]` or not
    pub fn is_inlinable(&self) -> bool {
        match self {
            Self::Struct(_) => true,
            Self::Enum(v) => v.is_empty(),
        }
    }

    /// Like [`ToTokens::to_token_stream`], but accepts the trait name to derive for
    ///
    /// # Returns
    ///
    /// `Some` if we should call `fmt()`, `None` if we shouldn't
    pub fn to_tokens_opt(&self, trait_name: &TraitName) -> Option<TokenStream> {
        Some(match self {
            Self::Struct(None) => return None,
            Self::Struct(Some(data)) => quote! { #trait_name::fmt(&#data, f) },
            Self::Enum(data) => {
                if data.is_empty() {
                    return None;
                } else {
                    Self::tokenise_enum(trait_name, data)
                }
            }
        })
    }

    /// Non-empty `enum` handler for [`Self::to_tokens_opt`]
    fn tokenise_enum(trait_name: &TraitName, data: &EnumData) -> TokenStream {
        let mut body = TokenStream::new();
        body.append_separated(
            data.iter()
                .map(move |(variant_name, first_field)| match first_field {
                    Some(FieldLike::Ident(id)) => {
                        quote! {
                            Self::#variant_name { #id: _a } => #trait_name::fmt(_a, f)
                        }
                    }
                    Some(FieldLike::Indexed) => {
                        quote! { Self::#variant_name(inner) => #trait_name::fmt(inner, f) }
                    }
                    None => {
                        let lit = Literal::string(&variant_name.to_string());
                        quote! { Self::#variant_name => f.write_str(#lit) }
                    }
                }),
            punct(','),
        );

        quote! {
            match self {
                #body
            }
        }
    }
}

impl ToTokens for FieldLike {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(ident("self"));
        tokens.append(punct('.'));

        match self {
            Self::Indexed => Literal::usize_unsuffixed(0).to_tokens(tokens),
            Self::Ident(id) => id.to_tokens(tokens),
        }
    }
}

impl TryFrom<Data> for FirstField {
    type Error = Error;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        Ok(match data {
            Data::Enum(data) => data.try_into()?,
            Data::Struct(data) => data.try_into()?,
            Data::Union(s) => {
                return Err(Error::new(s.union_token.span(), "Unions not supported"));
            }
        })
    }
}

impl FirstField {
    fn load_first_field<T>(fields: Punctuated<Field, T>) -> syn::Result<Option<FieldLike>> {
        let mut fields = fields.into_iter();
        let first = match fields.next() {
            Some(f) => f,
            None => return Ok(None),
        };

        if let Some(f) = fields.next() {
            const MSG: &str = "The struct/enum can only have one member";
            Err(Error::new(f.span(), MSG))
        } else {
            Ok(Some(match first.ident {
                Some(name) => FieldLike::Ident(name),
                None => FieldLike::Indexed,
            }))
        }
    }
}

impl TryFrom<DataStruct> for FirstField {
    type Error = Error;

    fn try_from(value: DataStruct) -> Result<Self, Self::Error> {
        Ok(Self::Struct(match value.fields {
            Fields::Unit => None,
            Fields::Named(f) => Self::load_first_field(f.named)?,
            Fields::Unnamed(f) => Self::load_first_field(f.unnamed)?,
        }))
    }
}

impl TryFrom<DataEnum> for FirstField {
    type Error = Error;

    fn try_from(value: DataEnum) -> Result<Self, Self::Error> {
        let mut out = Vec::with_capacity(value.variants.len());

        for var in value.variants {
            let first_field = match var.fields {
                Fields::Unit => None,
                Fields::Unnamed(f) => Self::load_first_field(f.unnamed)?,
                Fields::Named(f) => Self::load_first_field(f.named)?,
            };
            out.push((var.ident, first_field));
        }

        Ok(Self::Enum(out))
    }
}
