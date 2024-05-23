use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Error, Fields, LitInt, Result};

use crate::add_trait_bounds;

pub(super) fn derive_encode(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;

    let input_name = input.ident;

    add_trait_bounds(
        &mut input.generics,
        quote!(::valence_protocol::__private::Encode),
    );

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        Data::Struct(struct_) => {
            let encode_fields = match &struct_.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident.as_ref().unwrap();
                        let ctx = format!("failed to encode field `{name}` in `{input_name}`");
                        quote! {
                            self.#name.encode(_wtr).context(#ctx)?;
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        let ctx = format!("failed to encode field `{lit}` in `{input_name}`");
                        quote! {
                            self.#lit.encode(_wtr).context(#ctx)?;
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };

            Ok(quote! {
                impl #impl_generics crate::__private::Encode for #input_name #ty_generics
                #where_clause
                {
                    fn encode(&self, _wtr: &mut ::bytes::BytesMut) -> crate::__private::Result<()> {
                        use crate::__private::{Encode, Context};

                        #encode_fields

                        Ok(())
                    }
                }
            })
        }
        Data::Enum(enum_) => Err(Error::new(
            enum_.enum_token.span(),
            "cannot derive `Encode` on enums",
        )),
        Data::Union(union_) => Err(Error::new(
            union_.union_token.span(),
            "cannot derive `Encode` on unions",
        )),
    }
}
