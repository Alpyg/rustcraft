use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
    token, GenericArgument, Ident, LitInt, PathArguments, Result, Token, Type,
};

pub(super) fn define_protocol(tokens: TokenStream) -> Result<TokenStream> {
    let input = parse2::<Protocol>(tokens)?;

    let mut generated = TokenStream::new();

    let protocol_version = input.version;
    generated.extend(quote! {
        pub const PROTOCOL_VERSION: i32 = #protocol_version;
    });

    for state in input.states {
        let state_ident = &state.state;
        for side in state.sides {
            let side_ident = &side.side;
            for packet in side.packets {
                let packet_ident = &packet.name;
                let packet_id = &packet.id;

                let lifetime = if packet
                    .fields
                    .iter()
                    .any(|field| contains_lifetime(&field.ty))
                {
                    Some(quote! {<'a>})
                } else {
                    None
                };

                let fields: Vec<_> = packet
                    .fields
                    .iter()
                    .map(|field| {
                        let field_name = &field.name;
                        let field_ty = &field.ty;
                        quote! {
                            pub #field_name: #field_ty
                        }
                    })
                    .collect();

                generated.extend(quote! {
                    #[derive(crate::Encode, crate::Decode, crate::Packet, Debug)]
                    #[packet(id = #packet_id, side = crate::PacketSide::#side_ident, state = crate::PacketState::#state_ident)]
                    pub struct #packet_ident #lifetime {
                        #(#fields),*
                    }
                });
            }
        }
    }

    Ok(generated)
}

struct Protocol {
    version: LitInt,
    _brace_token: token::Brace,
    states: Punctuated<State, Token![,]>,
}

impl Parse for Protocol {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Protocol {
            version: input.parse()?,
            _brace_token: braced!(content in input),
            states: content.parse_terminated(State::parse, Token![,])?,
        })
    }
}

struct State {
    state: Ident,
    _brace_token: token::Brace,
    sides: Punctuated<Side, Token![,]>,
}

impl Parse for State {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(State {
            state: input.parse()?,
            _brace_token: braced!(content in input),
            sides: content.parse_terminated(Side::parse, Token![,])?,
        })
    }
}

struct Side {
    side: Ident,
    _brace_token: token::Brace,
    packets: Punctuated<Packet, Token![,]>,
}

impl Parse for Side {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Side {
            side: input.parse()?,
            _brace_token: braced!(content in input),
            packets: content.parse_terminated(Packet::parse, Token![,])?,
        })
    }
}

struct Packet {
    id: LitInt,
    name: Ident,
    _brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Packet {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Packet {
            id: input.parse()?,
            name: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

struct Field {
    name: Ident,
    _colon_token: Token![:],
    ty: Type,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

fn contains_lifetime(ty: &Type) -> bool {
    match ty {
        Type::Reference(reference) => reference.lifetime.is_some(),
        Type::Path(type_path) => type_path.path.segments.iter().any(|segment| {
            if let PathArguments::AngleBracketed(ref args) = segment.arguments {
                args.args.iter().any(|arg| match arg {
                    GenericArgument::Lifetime(_) => true,
                    GenericArgument::Type(inner_ty) => contains_lifetime(inner_ty),
                    _ => false,
                })
            } else {
                false
            }
        }),
        _ => false,
    }
}
