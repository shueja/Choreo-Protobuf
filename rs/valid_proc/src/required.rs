use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::PathSep;
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Generics, Ident, Path, PathArguments, PathSegment, QSelf, Token, Type, TypePath, Visibility
};

use crate::utils::{ForwardAttrsFilter, filter_forward_attrs};

#[derive(Debug, FromMeta)]
struct ValidArgs {
    ident: Option<Ident>,
    prefix: Option<Ident>,
    derive: Option<PathList>,
    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromField)]
#[darling(attributes(valid), forward_attrs)]
struct ValidField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
    #[darling(default)]
    optional: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(valid),
    forward_attrs,
    supports(struct_named, struct_tuple, enum_any)
)]
struct ValidInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, ValidField>,

    attrs: Vec<Attribute>,

    #[darling(flatten)]
    args: ValidArgs,
}


pub fn valid(input: TokenStream) -> TokenStream {
    // parse the struct or enum
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match ValidInput::from_derive_input(&derive_input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    // transfer the other derives besides this one
    let derive_attr = input.args.derive.as_ref().map(|derives| {
        let derives = derives.iter();
        quote! {
            #[derive(#(#derives),*)]
        }
    });

    //let forward_attrs = filter_forward_attrs(input.attrs.iter(), &input.args.forward_attrs);

    let vis = input.vis;
    let _ident = input.ident;

    // handling for enums
    // just type-alias as Valid
    if let syn::Data::Enum(_) = derive_input.data {
        let req_ident = format_ident!("Valid{}", _ident);
        return quote! {
            #vis type #req_ident = #_ident;
        }
        .into();
    }
    // get the ident for the new struct: either the "ident" argument or "prefix" on the existing name.
    let valid_ident = input
        .args
        .ident
        .or(input
            .args
            .prefix
            .map(|prefix| format_ident!("{prefix}{_ident}")))
        .expect("Must provide ident or prefix");
    let generics = input.generics;
    let fields = input.data.take_struct().unwrap();

    let mut field_idents = Vec::new();
    let mut field_declares = Vec::new();
    let mut field_conversion = Vec::new();
    fields.fields.iter().for_each(|field| {
        let vis = &field.vis;
        let ident = field.ident.as_ref().unwrap();
        let forward_attrs = filter_forward_attrs(
            field.attrs.iter(),
            &field.forward_attrs + &input.args.forward_attrs,
        );

        let ty: &Type = &field.ty;

        // Check if the field is optional

        field_idents.push(ident.clone());
        field_declares.push(quote! {
            // #(#forward_attrs)*
            #vis #ident: #ty
        });
        // if was_optional && !field.optional {
        //     field_conversion.push(quote! {#ident: Some(value.#ident.into())});
        // } else {
        //     field_conversion.push(quote! {#ident: value.#ident.into()});
        // }
    });

    // TODO: Implement From<ValidStruct> for OriginalStruct
    quote! {
        #derive_attr
        // #(#forward_attrs)*
        #vis struct #valid_ident #generics {
            #(#field_declares),*
        }

        impl From< #valid_ident > for #_ident {
            fn from(value: #valid_ident) -> Self {
                #_ident {
                #(#field_conversion),*
                }
            }
        }
    }
    .into()
}
