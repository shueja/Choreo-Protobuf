use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Generics, Ident, PathArguments,
    Type, TypePath, Visibility,
};

use crate::utils::{filter_forward_attrs, ForwardAttrsFilter};

#[derive(Debug, FromMeta)]
struct RequiredArgs {
    ident: Option<Ident>,
    prefix: Option<Ident>,
    derive: Option<PathList>,
    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromField)]
#[darling(attributes(required), forward_attrs)]
struct RequiredField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
    #[darling(default)]
    optional: bool
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(required),
    forward_attrs,
    supports(struct_named, struct_tuple, enum_any)
)]
struct RequiredInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, RequiredField>,

    attrs: Vec<Attribute>,

    #[darling(flatten)]
    args: RequiredArgs,
}

pub fn required(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match RequiredInput::from_derive_input(&derive_input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let derive_attr = input.args.derive.as_ref().map(|derives| {
        let derives = derives.iter();
        quote! {
            #[derive(#(#derives),*)]
        }
    });

    let forward_attrs = filter_forward_attrs(input.attrs.iter(), &input.args.forward_attrs);

    let vis = input.vis;
    let _ident = input.ident;

    if let syn::Data::Enum(_) = derive_input.data {
        let req_ident = format_ident!("Required{}", _ident);
        return quote! {
            #vis type #req_ident = #_ident;
        }.into();
    }
    let required_ident = input.args.ident.or(
        input.args.prefix.map(|prefix|format_ident!("{prefix}{_ident}"))).expect("Must provide ident or prefix");
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
        let (ty, was_optional) = match ty {
            Type::Path(TypePath { path, .. }) => {
                
                let segments_str = &path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                let option_segment = ["Option", "std::option::Option", "core::option::Option"]
                    .iter()
                    .find(|s| segments_str == *s)
                    .and_then(|_| path.segments.last());
                let was_optional = option_segment.is_some();
                let inner_type = option_segment
                    .and_then(|path_seg| match &path_seg.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) => 
                        args.first(),
                         _ => None,
                    })
                    .and_then(|generic_arg| match generic_arg {
                        GenericArgument::Type(typ) => if field.optional {Some(ty.clone())} else {
                            Some(match typ {
                            syn::Type::Path(path)=>{
                                let mut path = path.clone();
                                let ident = path.path.segments.last().unwrap().ident.clone();
                                path.path.segments.last_mut().unwrap().ident = format_ident!("Required{}", ident);
                                syn::Type::Path(path).into()},
                            _=>typ.clone()
                                })},
                        _ => None,
                    });

                (inner_type.unwrap_or(ty.clone()), was_optional)
            }
            // TODO case for 
            _ => (ty.clone(), false),
        };



        field_idents.push(ident.clone());
        field_declares.push(quote! {
            // #(#forward_attrs)*
            #vis #ident: #ty
        });
        if was_optional && !field.optional {
        field_conversion.push(quote!{#ident: Some(value.#ident.into())});
        } else {
            field_conversion.push(quote!{#ident: value.#ident.into()});
        }

    });

    // TODO: Implement From<RequiredStruct> for OriginalStruct
    quote! {
        #derive_attr
        // #(#forward_attrs)*
        #vis struct #required_ident #generics {
            #(#field_declares),*
        }

        impl From< #required_ident > for #_ident {
            fn from(value: #required_ident) -> Self {
                #_ident {
                #(#field_conversion),*
                }
            }
        }
    }
    .into()
}
