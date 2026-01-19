use core::error;
use std::vec;

use deluxe::ParseMetaItem;
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse::ParseBuffer;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    AngleBracketedGenericArguments, Attribute, FieldsNamed, GenericArgument, Generics, Ident, Path,
    PathArguments, PathSegment, QSelf, Token, Variant, Visibility,
};
use syn::{
    DataEnum, DataStruct, DeriveInput, Field, Type, TypePath, parse_macro_input, token::Token,
};
mod utils;

// #[proc_macro_derive(Valid, attributes(valid))]
// pub fn valid(input: TokenStream) -> TokenStream {
//     valid::valid(input)
// }
#[derive(ParseMetaItem)]
struct Options {
    prefix: Option<String>,
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

fn convert_type_path_to_valid(path: TypePath) -> TypePath {
    let mut path = path.clone();
    let ident = path.path.segments.last().unwrap().ident.clone();
    path.path.segments.last_mut().unwrap().ident = make_valid_ident(&ident);
    path
}

///
/// Option<Innertype> -> ValidInnerType
/// #[optional] Option<InnerType> -> Option<ValidInnerType>
/// Type -> Type
/// #[optional] Type -> Type
///
///
///
/// boolean: was originally option
fn convert_type(input: &Type, should_remain_optional: bool) -> (bool, Type) {
    let inner_type_opt = match input {
        Type::Path(TypePath { path, .. }) => extract_option_inner(path),
        _ => None,
    };
    if let Some(Type::Path(path)) = inner_type_opt {
        let valid_path = convert_type_path_to_valid(path);
        return (true, syn::Type::Path(valid_path));
    } else {
        return (false, input.clone());
    }
}
fn convert_field(field: &Field) -> (bool, Field) {
    let mut new_field = field.clone();
    new_field.attrs = filter_prost_attributes(new_field.attrs);
    // new_field.ident = new_field.ident.map(|i| format_ident!("req{i}"));
    let (was_option, new_ty) = convert_type(&new_field.ty, false);
    new_field.ty = new_ty;
    (was_option, new_field)
}
fn make_valid_ident(ident: &Ident) -> Ident {
    format_ident!("Valid{ident}")
}

use syn::Fields;

struct MakeValidFieldsOutput {
    is_different: bool,
    new_fields: Fields,
    tryfrom_conversions: Vec<proc_macro2::TokenStream>,
    from_conversions: Vec<proc_macro2::TokenStream>,
}
fn named_field_tryfrom_conversion(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {#ident: optional.#ident}
}
fn named_field_option_tryfrom(ident: &Ident) -> proc_macro2::TokenStream {
    let missing_message = format!("{} is missing", ident.to_string());
    quote! {
    #ident: match optional.#ident {
                Some(object)=> match object.try_into().map_err(|e| format!("{}", e).to_string()) {
                    Ok(valid_object)=>valid_object,
                    Err(e) => return Err(e)
                },

                None=> return Err(#missing_message.to_string())
            }}
}

fn unnamed_field_tryfrom_conversion(index: usize) -> proc_macro2::TokenStream {
    quote! {optional.#index.try_into()?}
}
fn unnamed_field_from_conversion(index: usize) -> proc_macro2::TokenStream {
    quote! {valid.#index}
}

fn named_field_from_conversion(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {#ident: valid.#ident}
}
fn named_field_option_from(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
    #ident: Some(valid.#ident.into())
}}

fn make_valid_fields(fields: Fields) -> MakeValidFieldsOutput {
    let mut is_different = false;
    let mut tryfrom_conversions: Vec<proc_macro2::TokenStream> = vec![];
    let mut from_conversions: Vec<proc_macro2::TokenStream> = vec![];
    let convert = |tup: (usize, &Field)| {
        let (index, field) = tup;
        let (was_option, new_field) = convert_field(field);
        is_different |= (new_field.ty != field.ty);

        tryfrom_conversions.push(match &field.ident {
            Some(ident) => {
                if was_option {
                    named_field_option_tryfrom(ident)
                } else {
                    named_field_tryfrom_conversion(ident)
                }
            }
            None => unnamed_field_tryfrom_conversion(index),
        });
        from_conversions.push(match &field.ident {
            Some(ident) => {
                if was_option {
                    named_field_option_from(ident)
                } else {
                    named_field_from_conversion(ident)
                }
            }
            None => unnamed_field_from_conversion(index),
        });
        new_field
    };
    let new_fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { brace_token, named }) => {
            syn::Fields::Named(syn::FieldsNamed {
                brace_token,
                named: named
                    .iter()
                    .enumerate()
                    .map(convert)
                    .collect::<Punctuated<Field, Comma>>(),
            })
        }
        syn::Fields::Unnamed(syn::FieldsUnnamed {
            paren_token,
            unnamed,
        }) => syn::Fields::Unnamed(syn::FieldsUnnamed {
            paren_token,
            unnamed: unnamed
                .iter()
                .enumerate()
                .map(convert)
                .collect::<Punctuated<Field, Comma>>(),
        }),
        syn::Fields::Unit => fields.clone(),
    };
    MakeValidFieldsOutput {
        is_different,
        new_fields,
        tryfrom_conversions,
        from_conversions,
    }
}
fn get_filtered_derive_macro(attrs: Vec<Attribute>) -> (proc_macro2::TokenStream, Vec<Attribute>) {
    let mut traits: Vec<Type> = vec![];

    attrs.iter().for_each(|a| {
        if !a.path().is_ident("derive") {
            return;
        }
        let _ = a.parse_nested_meta(|meta| {
            let args = meta.path.to_token_stream();
            let parse = |args| {
                syn::parse::Parser::parse2(
                    Punctuated::<Type, syn::token::Comma>::parse_terminated,
                    args,
                )
            };
            let res = parse(args);
            if let Err(e) = res {
                unimplemented!("unparseable");
            }
            let res = res.unwrap();

            for a in res.into_iter() {
                traits.push(a);
            }
            Ok(())
        });
    });
    let filtered_traits = traits.iter().filter(|t| match t {
        Type::Path(type_path) => type_path
            .path
            .segments
            .iter()
            .find(|s| s.ident.to_string() == "prost")
            .is_none(),
        _ => true,
    });

    // fully remove the derive attribute since all derive attributes are coalesced into trait_tokens
    let attrs = attrs
        .iter()
        .filter(|a| !a.path().is_ident("derive"))
        .map(Attribute::clone)
        .collect();
    (quote! {#[derive(#(#filtered_traits,)*)]}, attrs)
}

fn make_valid_struct(
    input: &DeriveInput,
) -> (
    proc_macro2::TokenStream,
    Option<DeriveInput>,
    proc_macro2::TokenStream,
) {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = input.clone();

    let (trait_tokens, attrs) = get_filtered_derive_macro(attrs);
    let valid_ident = make_valid_ident(&ident);
    let mut is_different = false;
    let gen_valid = |f: Fields, is_different: &mut bool| {
        let output = make_valid_fields(f);
        *is_different |= output.is_different;
        output
    };

    let (impls, data) = match data {
        syn::Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => {
            let MakeValidFieldsOutput {
                is_different,
                new_fields,
                tryfrom_conversions,
                from_conversions,
            } = gen_valid(fields, &mut is_different);
            let impls = quote!(
            impl TryFrom<#ident> for #valid_ident {
                type Error = String;
                fn try_from(optional: #ident) -> Result<#valid_ident, Self::Error> {
                    Ok(#valid_ident {
                        #(#tryfrom_conversions,)*
                    })
                }
            }
            impl From<#valid_ident> for #ident {
                fn from(valid: #valid_ident) -> #ident {
                    #ident {
                        #(#from_conversions,)*
                    }
                }
            }
            impl crate::validate::Valid for #valid_ident {
                type Optional = #ident;
                fn optionize(self) -> #ident {
                    self.into()
                }
            }
            impl crate::validate::Validate  for #ident {
                type Valid = #valid_ident;
                fn validate(self) -> Result<#valid_ident, String>{
                    self.try_into()
                }
            }
            );
            (
                impls,
                syn::Data::Struct(DataStruct {
                    struct_token,
                    semi_token,
                    fields: new_fields,
                }),
            )
        }
        syn::Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => (
            proc_macro2::TokenStream::new(),
            syn::Data::Enum(DataEnum {
                enum_token,
                brace_token,
                variants: variants
                    .iter()
                    .map(|v| {
                        let Variant {
                            attrs,
                            ident,
                            fields,
                            discriminant,
                        } = v.clone();

                        let MakeValidFieldsOutput {
                            is_different,
                            new_fields,
                            tryfrom_conversions,
                            from_conversions,
                        } = gen_valid(fields, &mut is_different);
                        Variant {
                            attrs: filter_prost_attributes(attrs),
                            ident,
                            fields: new_fields,
                            discriminant,
                        }
                    })
                    .collect(),
            }),
        ),
        _ => unimplemented!("unimplemented syn::Data"),
    };
    if !is_different {
        let original_ident = &input.ident;
        return (
            quote! {pub type #valid_ident = #original_ident;},
            None,
            proc_macro2::TokenStream::new(),
        );
    }
    (
        trait_tokens,
        Some(DeriveInput {
            attrs,
            vis,
            ident: valid_ident,
            generics,
            data,
        }),
        impls,
    )
}

fn filter_prost_attributes(attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|a| !a.path().is_ident("prost"))
        .map(Attribute::clone)
        .collect::<Vec<Attribute>>()
}

#[proc_macro_attribute]
pub fn make_valid(attr: TokenStream, item: TokenStream) -> proc_macro::TokenStream {
    let Options { prefix } = match deluxe::parse::<Options>(attr) {
        Ok(desc) => desc,
        Err(e) => return e.into_compile_error().into(),
    };
    let input = parse_macro_input!(item as DeriveInput);

    let (derive, valid_tokens, impls) = match &input.data {
        syn::Data::Struct(_data_struct) => make_valid_struct(&input),
        syn::Data::Enum(_data_enum) => make_valid_struct(&input),
        syn::Data::Union(_data_union) => unimplemented!("Only for structs and enums"),
    };

    let valid_tokens = valid_tokens
        .map(DeriveInput::into_token_stream)
        .unwrap_or(proc_macro2::TokenStream::new());
    quote! {
        #input
        #derive
        #valid_tokens
        #impls
    }
    .into()
}
