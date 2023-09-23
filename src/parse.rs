use syn::punctuated::Punctuated;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields};

use crate::{BaseTokenStream, EnumData, FieldLike, FirstField, ParsedData};

impl ParsedData {
    pub fn parse(input: BaseTokenStream) -> syn::Result<Self> {
        let input = syn::parse::<DeriveInput>(input)?;
        Ok(Self {
            first_field: input.data.try_into()?,
            ident: input.ident,
            generics: input.generics,
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
            Err(Error::new_spanned(
                f,
                "The struct/enum can only have one member",
            ))
        } else {
            Ok(Some(match first.ident {
                Some(name) => FieldLike::Ident(name),
                None => FieldLike::Indexed,
            }))
        }
    }
}

impl TryFrom<Data> for FirstField {
    type Error = Error;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        match data {
            Data::Enum(data) => data.try_into(),
            Data::Struct(data) => data.try_into(),
            Data::Union(s) => Err(Error::new_spanned(s.union_token, "Unions not supported")),
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
        let it = value
            .variants
            .into_iter()
            .map(move |var| -> syn::Result<EnumData> {
                let first_field = match var.fields {
                    Fields::Unit => None,
                    Fields::Unnamed(f) => Self::load_first_field(f.unnamed)?,
                    Fields::Named(f) => Self::load_first_field(f.named)?,
                };
                Ok((var.ident, first_field))
            });

        Ok(Self::Enum(macroific::attr_parse::__private::try_collect(
            it,
        )?))
    }
}
