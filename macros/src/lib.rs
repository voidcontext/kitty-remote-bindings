use proc_macro::TokenStream;
use syn::{DeriveInput, ItemEnum, ItemStruct};

mod kitty_command;
mod kitty_command_option;

/// # Panics
///
/// Will panic if item is not a struct
#[proc_macro_derive(KittyCommand, attributes(kitty_command, top_level, option))]
pub fn derive_command(item: TokenStream) -> TokenStream {
    let item_parsed = {
        let cloned = item.clone();
        syn::parse_macro_input!(cloned as DeriveInput)
    };

    match item_parsed.data {
        syn::Data::Struct(_) => {
            kitty_command::derive_impl(&syn::parse_macro_input!(item as ItemStruct))
        }
        _ => panic!("Only struct is supported by the KittyCommand macro"),
    }
}

/// # Panics
///
/// Panics if the item is not a struct or an enum
#[proc_macro_derive(KittyCommandOption, attributes(prefix))]
pub fn derive_command_option(item: TokenStream) -> TokenStream {
    let item_parsed = {
        let cloned = item.clone();
        syn::parse_macro_input!(cloned as DeriveInput)
    };

    match item_parsed.data {
        syn::Data::Struct(_) => {
            kitty_command_option::struct_impl(&syn::parse_macro_input!(item as ItemStruct))
        }
        syn::Data::Enum(_) => {
            kitty_command_option::enum_impl(&syn::parse_macro_input!(item as ItemEnum))
        }
        syn::Data::Union(_) => {
            panic!("Only enum and struct is supported by the KittyCommand macro")
        }
    }
}
