
use proc_macro::TokenStream;

use crate::todouble::derive_proc_macro_impl;
mod utils;
mod todouble;

#[proc_macro_derive(ToDouble)]
pub fn make_valid(input: TokenStream) -> proc_macro::TokenStream {
    derive_proc_macro_impl(input)
}
