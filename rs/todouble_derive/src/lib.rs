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

use crate::todouble::derive_proc_macro_impl;
mod utils;
mod todouble;

#[proc_macro_derive(ToDouble)]
pub fn make_valid(input: TokenStream) -> proc_macro::TokenStream {
    derive_proc_macro_impl(input)
}
