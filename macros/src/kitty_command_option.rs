use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemEnum, ItemStruct};

pub fn enum_impl(item: &ItemEnum) -> TokenStream {
    let name = item.ident.clone();

    let variant_case = item.variants.iter().map(|variant| {
        let variant_name = variant.ident.clone();

        let variant_str = syn::LitStr::new(
            variant_name.to_string().to_lowercase().as_str(),
            Span::call_site(),
        );

        match variant.fields.clone() {
            syn::Fields::Named(_) => {
                todo!()
                // let fields = named_fields
                //     .named
                //     .iter()
                //     .map(|named| named.ident.as_ref().unwrap());
                // let write_fields = fields.clone().map(|field| {
                //     quote! {
                //         #field.write(writer).await?;
                //     }
                // });

                // quote! {
                //     #name::#variant_name { #(#fields),* } => {
                //         #ordinal.write(writer).await?;
                //         #(#write_fields)*
                //         Ok(())
                //     }
                // }
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                assert!(
                    unnamed_fields.unnamed.len() == 1,
                    "Only a single unnamed field is supported"
                );

                quote! {
                    #name::#variant_name(value) => format!("{}:{}", #variant_str, value),
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #name::#variant_name => String::from(#variant_str),
                }
            }
        }
    });

    let gen = quote! {
        impl crate::ToArg for #name {
            fn to_arg(&self) -> String {
                match self {
                    #(#variant_case)*
                }
            }
        }
    };

    gen.into()
}

pub fn struct_impl(item: &ItemStruct) -> TokenStream {
    let name = item.ident.clone();
    match &item.fields {
        syn::Fields::Named(_) => todo!(),
        syn::Fields::Unnamed(unnamed_fields) => {
            assert!(
                unnamed_fields.unnamed.len() == 1,
                "Only new types with a single unnamed field are supported"
            );

            let gen = quote! {
                impl crate::ToArg for #name {
                    fn to_arg(&self) -> String {
                        crate::ToArg::to_arg(&self.0)
                    }
                }
            };

            gen.into()
        }
        syn::Fields::Unit => todo!(),
    }
}
