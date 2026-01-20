/*
 *   Copyright (c) 2022 Nazmul Idris
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{Data::{Enum, Struct, Union}, DataEnum, DataStruct, DeriveInput, Field, Fields::{self, Named, Unit, Unnamed}, FieldsNamed, FieldsUnnamed, Ident, Path, Type, TypePath, Variant, ext, parse_macro_input, punctuated::Punctuated, token::Comma};

pub fn to_double_ident(ident: &Ident) -> Ident {
    syn::Ident::new(ident.to_string().replace("Expr", "Double").as_str(), Span::call_site())
}
pub fn derive_proc_macro_impl(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput); // Same as: syn::parse(input).unwrap();
let DeriveInput {
    ident: struct_name_ident,
    data,
    generics,
    ..
  } = input.clone();
  let where_clause = &generics.where_clause;

  let double_ident = to_double_ident(&struct_name_ident);
  let is_nullop = double_ident == struct_name_ident;
  if is_nullop {
    return   quote! {
    impl #generics crate::to_double::ToDouble for #struct_name_ident #generics #where_clause {
        type DoubleType = #double_ident;
    }
  }
  .into()
  }

  let into_f64 = make_f64_struct(&input);
  quote! {
    impl #generics crate::to_double::ToDouble for #struct_name_ident #generics #where_clause {
        type DoubleType = #double_ident;
    }
    impl #generics Into<#double_ident> for #struct_name_ident #generics #where_clause {
        fn into(self) -> #double_ident {
            #double_ident { // todo this should handle unnamed fields differently ()
            #(#into_f64,)*
            }
        }
    }
  }
  .into()
}
fn path_contains<T: IntoIterator<Item = &'static str>>(path: &Path, search: T) ->bool {
    let segments_str = &path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");
    let option_segment = search
        .into_iter()
        .find(|s| segments_str == *s);
    option_segment.is_some()
}

fn convert_named_field(ident: &Ident) ->proc_macro2::TokenStream {
    quote! {#ident: self.#ident.into()}
}
fn convert_unnamed_field(index: usize) ->proc_macro2::TokenStream {
    quote! {self.#index.into()}
}
fn convert_option_field(ident: &Ident) ->proc_macro2::TokenStream {
    quote! {#ident: self.#ident.map(Into::into).into()}
}
fn convert_option_expr_field(ident: &Ident) ->proc_macro2::TokenStream {
    quote! {#ident: self.#ident.expect("Converted optional Expr into double!!!").into()}
}
fn convert_vector_field(ident: &Ident) ->proc_macro2::TokenStream {
    quote! {#ident: self.#ident.into_iter().map(Into::into).collect()}
}

fn extract_option_inner(path: &Path) -> Option<Type> {
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
    option_segment
        .and_then(|path_seg| match &path_seg.arguments {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                args.first()
            }
            _ => None,
        })
        .and_then(|generic_arg| match generic_arg {
            GenericArgument::Type(typ) => Some(typ.clone()),
            _ => None,
        })
}

enum ConversionType {
    ExprOpt,
    Opt,
    Vec,
    Into
}
fn get_conversion_type(ty: &Type) -> ConversionType {
    match &field.ty {
            Type::Path(TypePath { path, .. }) => {
                if path_contains(path,["Vec", "prost::alloc::vec::Vec"]) {
                    return ConversionType::Vec;
                }
                match extract_option_inner(path) {

                    Type::Path(TypePath { path, .. }) => {
                        if path_contains(path, search)
                    },
                    _=
                }
            },
            _ => false
}
fn make_into_f64(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let mut conversions: Vec<proc_macro2::TokenStream> = vec![];
    let convert = |(index, field): (usize, &Field)| {
       ConversionType =  
        };
        let inner_type_vec = match &field.ty {
            Type::Path(TypePath { path, .. }) => path_contains(path,["Vec", "prost::alloc::vec::Vec"]),
            _ => false
        };
        conversions.push(match &field.ident {
            Some(ident) => {
                if (inner_type_opt) {
                    convert_option_field(ident)
                } else if (inner_type_vec) {
                    convert_vector_field(ident)
                }else {
                    convert_named_field(ident)
                }
            }
            None => convert_unnamed_field(index),
        });
    };
    match fields {
        syn::Fields::Named(syn::FieldsNamed { brace_token, named }) => {
            named
                    .iter()
                    .enumerate()
                    .for_each(convert);}
        syn::Fields::Unnamed(syn::FieldsUnnamed {
            paren_token,
            unnamed,
        }) => {unnamed
                .iter()
                .enumerate()
                .for_each(convert)
        },
        syn::Fields::Unit => {},
    };
    conversions
}

fn make_f64_struct(
    input: &DeriveInput,
) -> Vec<proc_macro2::TokenStream> {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = input.clone();

    match data {
        syn::Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => make_into_f64(&fields),
        syn::Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => 
            variants
                    .iter()
                    .map(|v| {
                        let Variant { attrs, ident, fields, discriminant } = v;
                        let conversions = make_into_f64(fields);
                        let field_tokens = match fields {
                            Named(fields_named) => quote!{{#(#conversions,)*}}
                            ,
                            Unnamed(fields_unnamed) =>  quote!{(#(#conversions,)*)},
                            Unit => quote!{},
                        };
                        let discriminant_tokens = discriminant.as_ref().map(|(eq, expr)|quote!{= #expr});
                        quote! {
                            #(#attrs)*
                            #ident #field_tokens #discriminant_tokens
                        }
                        
                    }).collect(),
        _ => unimplemented!("unimplemented syn::Data"),
    }

}