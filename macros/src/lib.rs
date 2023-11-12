use match_field::match_fields_impl;
use proc_macro::TokenStream;

mod match_field;

#[proc_macro_attribute]
pub fn match_fields(attr: TokenStream, item: TokenStream) -> TokenStream {
    match_fields_impl(attr, item)
}
