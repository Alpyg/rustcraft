use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse2, Data, DeriveInput, Error, Fields, LitInt, Result};

use crate::{add_trait_bounds, pair_variants_with_discriminants};

pub(super) fn derive_encode(item: TokenStream) -> Result<TokenStream> {
    let mut input = parse2::<DeriveInput>(item)?;

    let input_name = input.ident;

    add_trait_bounds(&mut input.generics, quote!(crate::__private::Encode));

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
        Data::Enum(enum_) => {
            let variants = pair_variants_with_discriminants(enum_.variants)?;

            let encode_arms = variants
                .iter()
                .map(|(disc, variant)| {
                    let variant_name = &variant.ident;

                    let disc_ctx = format!(
                        "failed to encode enum discriminant {disc} for variant `{variant_name}` \
                         in `{input_name}`",
                    );

                    match &variant.fields {
                        Fields::Named(fields) => {
                            let field_names = fields
                                .named
                                .iter()
                                .map(|f| f.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    let ctx = format!(
                                        "failed to encode field `{name}` in variant \
                                         `{variant_name}` in `{input_name}`",
                                    );

                                    quote! {
                                        #name.encode(&mut _wtr).context(#ctx)?;
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name { #(#field_names,)* } => {
                                    VarInt(#disc).encode(&mut _wtr).context(#disc_ctx)?;

                                    #encode_fields
                                    Ok(())
                                }
                            }
                        }
                        Fields::Unnamed(fields) => {
                            let field_names = (0..fields.unnamed.len())
                                .map(|i| Ident::new(&format!("_{i}"), Span::call_site()))
                                .collect::<Vec<_>>();

                            let encode_fields = field_names
                                .iter()
                                .map(|name| {
                                    let ctx = format!(
                                        "failed to encode field `{name}` in variant \
                                         `{variant_name}` in `{input_name}`"
                                    );

                                    quote! {
                                        #name.encode(&mut _wtr).context(#ctx)?;
                                    }
                                })
                                .collect::<TokenStream>();

                            quote! {
                                Self::#variant_name(#(#field_names,)*) => {
                                    VarInt(#disc).encode(&mut _wtr).context(#disc_ctx)?;

                                    #encode_fields

                                    Ok(())
                                }
                            }
                        }
                        Fields::Unit => quote! {
                            Self::#variant_name => Ok(
                                VarInt(#disc)
                                    .encode(&mut _wtr)
                                    .context(#disc_ctx)?
                            ),
                        },
                    }
                })
                .collect::<TokenStream>();

            Ok(quote! {
                #[allow(unused_imports, unreachable_code)]
                impl #impl_generics crate::__private::Encode for #input_name #ty_generics
                #where_clause
                {
                    fn encode(&self, mut _wtr: &mut ::bytes::BytesMut) -> crate::__private::Result<()> {
                        use crate::__private::{Encode, VarInt, Context};

                        match self {
                            #encode_arms
                            _ => unreachable!(),
                        }
                    }
                }
            })
        }
        Data::Union(union_) => Err(Error::new(
            union_.union_token.span(),
            "cannot derive `Encode` on unions",
        )),
    }
}
