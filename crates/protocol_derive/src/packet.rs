use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Attribute, DeriveInput, Error, Expr, LitInt, LitStr, Result};

use crate::add_trait_bounds;

pub(super) fn derive_packet(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;

    let packet_attr = parse_packet_helper_attr(&input.attrs)?.unwrap_or_default();

    let name = input.ident.clone();

    let name_str = if let Some(attr_name) = packet_attr.name {
        attr_name.value()
    } else {
        name.to_string()
    };

    let id = if let Some(id_attr) = packet_attr.id {
        id_attr
    } else {
        return Err(Error::new(
            packet_attr.span,
            "missing `id = i32` value from `packet` attribute",
        ));
    };

    add_trait_bounds(&mut input.generics, quote!(::std::fmt::Debug));

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let side = if let Some(side_attr) = packet_attr.side {
        side_attr
    } else {
        return Err(Error::new(
            packet_attr.span,
            "missing `side = PacketSide::...` value from `packet` attribute",
        ));
    };

    let state = if let Some(state_attr) = packet_attr.state {
        state_attr
    } else {
        return Err(Error::new(
            packet_attr.span,
            "missing `state = PacketState::...` value from `packet` attribute",
        ));
    };

    Ok(quote! {
        impl #impl_generics crate::__private::Packet for #name #ty_generics
        #where_clause
        {
            const ID: i32 = #id;
            const NAME: &'static str = #name_str;
            const SIDE: crate::__private::PacketSide = #side;
            const STATE: crate::__private::PacketState = #state;
        }
    })
}

struct PacketAttr {
    span: Span,
    id: Option<Expr>,
    tag: Option<i32>,
    name: Option<LitStr>,
    side: Option<Expr>,
    state: Option<Expr>,
}

impl Default for PacketAttr {
    fn default() -> Self {
        Self {
            span: Span::call_site(),
            id: Default::default(),
            tag: Default::default(),
            name: Default::default(),
            side: Default::default(),
            state: Default::default(),
        }
    }
}

fn parse_packet_helper_attr(attrs: &[Attribute]) -> Result<Option<PacketAttr>> {
    for attr in attrs {
        if attr.path().is_ident("packet") {
            let mut res = PacketAttr {
                span: attr.span(),
                id: None,
                tag: None,
                name: None,
                side: None,
                state: None,
            };

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    res.id = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else if meta.path.is_ident("tag") {
                    res.tag = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<i32>()?);
                    Ok(())
                } else if meta.path.is_ident("name") {
                    res.name = Some(meta.value()?.parse::<LitStr>()?);
                    Ok(())
                } else if meta.path.is_ident("side") {
                    res.side = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else if meta.path.is_ident("state") {
                    res.state = Some(meta.value()?.parse::<Expr>()?);
                    Ok(())
                } else {
                    Err(meta.error("unrecognized packet argument"))
                }
            })?;

            return Ok(Some(res));
        }
    }

    Ok(None)
}
