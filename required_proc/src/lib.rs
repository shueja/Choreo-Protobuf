use proc_macro::TokenStream;

mod required;
mod utils;

#[proc_macro_derive(Required, attributes(required))]
pub fn required(input: TokenStream) -> TokenStream {
    required::required(input)
}