use super::dual_attr::DualAttr;
use macroific::prelude::*;
use std::ops::AddAssign;
use syn::punctuated::Punctuated;
use syn::{Attribute, Token, Type, WherePredicate};

#[derive(AttributeOptions, Default)]
pub(crate) struct ContainerOptions {
    pub bounds: Punctuated<WherePredicate, Token![,]>,
    pub delegate_to: Option<Type>,
}

impl ContainerOptions {
    pub fn resolve<I>(attrs: I, attr_name: &str) -> syn::Result<Self>
    where
        I: IntoIterator<Item = Attribute>,
    {
        let attrs = DualAttr::collect(attrs, attr_name);
        let mut out = Self::default();

        for dattr in attrs {
            let opts = ContainerOptions::from_attr(dattr.attr)?;
            out += opts;
        }

        Ok(out)
    }
}

impl AddAssign for ContainerOptions {
    fn add_assign(&mut self, rhs: Self) {
        let Self {
            bounds: bounds_l,
            delegate_to: delegate_to_l,
        } = self;

        let Self {
            bounds: bounds_r,
            delegate_to: delegate_to_r,
        } = rhs;

        bounds_l.extend(bounds_r);

        if let Some(delegate_to) = delegate_to_r {
            *delegate_to_l = Some(delegate_to);
        }
    }
}
