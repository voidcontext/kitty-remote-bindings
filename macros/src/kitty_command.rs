use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ExprLit, ItemStruct, Meta};

pub(crate) fn derive_impl(st: &ItemStruct) -> TokenStream {
    let struct_impl = derive_struct_impl(st);
    let from_kitty_command_for_command = drive_from_kitty_command_for_command(st);

    let gen = quote! {
        #struct_impl
        #from_kitty_command_for_command
    };

    gen.into()
}

fn derive_struct_impl(st: &ItemStruct) -> proc_macro2::TokenStream {
    let name = st.ident.clone();

    let required_fields = required_fields(st);

    let option_field_init = optional_fields(st).into_iter().map(|field| {
        let field_name = field.ident.clone();
        quote!(#field_name: None)
    });

    let required_field_capture = required_fields.iter().map(|field| {
        let field_name = field.ident.clone();
        let tpe = field.ty.clone();
        quote!(#field_name: #tpe)
    });

    let required_field_init = required_fields.iter().map(|field| {
        let field_name = field.ident.clone();
        quote!(#field_name)
    });

    let field_sep = if required_fields.is_empty() {
        quote!()
    } else {
        quote!(,)
    };

    let setters = st.fields.iter().map(|field| match &field.ty {
        syn::Type::Path(tpath) => {
            let option_segment = tpath.path.segments.last();
            let option_segment = option_segment
                .iter()
                .find(|p| p.ident == syn::Ident::new("Option", proc_macro2::Span::call_site()));

            if let Some(option) = option_segment {
                match &option.arguments {
                    syn::PathArguments::AngleBracketed(arg) => match arg.args.first() {
                        Some(syn::GenericArgument::Type(t)) => setter_for_optional_field(field, t),
                        _ => panic!("Not supported"),
                    },
                    _ => panic!("This shouldn't happen"),
                }
            } else {
                setter_for_field(field, &field.ty)
            }
        }
        _ => setter_for_field(field, &field.ty),
    });

    quote! {
        impl #name {
            pub fn new(#(#required_field_capture),*) -> Self {
                Self {
                    #(#required_field_init),*#field_sep
                    #(#option_field_init),*
                }
            }

            #(#setters)*
        }
    }
}

fn drive_from_kitty_command_for_command(st: &ItemStruct) -> proc_macro2::TokenStream {
    let name = st.ident.clone();
    let kitty_command = find_name_value_attr(&st.attrs, "kitty_command")
        .expect("kitty_command attribute must be set, and its value must be a string literal");

    let required_fields = required_fields(st);
    let args = if required_fields.is_empty() {
        quote!()
    } else {
        let args = required_fields.into_iter().map(|field| {
            let field_name = field.ident.clone().unwrap();
            quote! {
                cmd.args(&kitty_remote_bindings_core::ToArg::to_arg(&value.#field_name));
            }
        });
        quote!(#(#args)*)
    };

    let optional_fields = optional_fields(st);

    let top_level_options = to_cmd_options(
        optional_fields
            .iter()
            .copied()
            .filter(|field| is_top_level_option(field)),
    );

    let options = to_cmd_options(
        optional_fields
            .iter()
            .copied()
            .filter(|field| !is_top_level_option(field)),
    );

    quote! {
        impl<'a> From<&'a #name> for std::process::Command {
            fn from(value: &#name) -> Self {
                let mut cmd = std::process::Command::new("kitty");
                cmd.arg("@");

                #top_level_options

                cmd.arg(#kitty_command);

                #options
                #args

                cmd
            }
        }
    }
}

fn find_name_value_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a ExprLit> {
    attrs.iter().find_map(|attr| {
        if let Meta::NameValue(m) = &attr.meta {
            match m.path.get_ident() {
                Some(ident) if ident.to_string().as_str() == name => match &m.value {
                    syn::Expr::Lit(lit) => match &lit.lit {
                        syn::Lit::Str(_) => Some(lit),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    })
}

fn to_cmd_options<'a, I: Iterator<Item = &'a syn::Field>>(fields: I) -> proc_macro2::TokenStream {
    let options = fields.map(|field| {
        let field_name = field.ident.clone().unwrap();
        let option_name = find_name_value_attr(&field.attrs, "option")
            .and_then(|lit| match &lit.lit {
                syn::Lit::Str(str) => Some(str.value()),
                _ => todo!(),
            })
            .unwrap_or_else(|| field_name.to_string());
        let option_str = syn::LitStr::new(
            format!("--{}", option_name.replace('_', "-")).as_str(),
            proc_macro2::Span::call_site(),
        );
        quote! {
            if let Some(option_value) = &value.#field_name {
                cmd.arg(#option_str.to_string());
                cmd.args(&kitty_remote_bindings_core::ToArg::to_arg(option_value));
            }
        }
    });
    quote!(#(#options)*)
}

fn required_fields(st: &ItemStruct) -> Vec<&syn::Field> {
    st.fields
        .iter()
        .filter(|field| match &field.ty {
            syn::Type::Path(tpath) => tpath
                .path
                .segments
                .last()
                .iter()
                .any(|p| p.ident != syn::Ident::new("Option", proc_macro2::Span::call_site())),
            _ => false,
        })
        .collect()
}

fn optional_fields(st: &ItemStruct) -> Vec<&syn::Field> {
    st.fields
        .iter()
        .filter(|field| match &field.ty {
            syn::Type::Path(tpath) => tpath
                .path
                .segments
                .last()
                .iter()
                .any(|p| p.ident == syn::Ident::new("Option", proc_macro2::Span::call_site())),
            _ => false,
        })
        .collect()
}

fn is_top_level_option(field: &syn::Field) -> bool {
    field.attrs.iter().any(|attr| {
        if let Meta::Path(m) = &attr.meta {
            let segments = m.segments.iter().collect::<Vec<_>>();
            segments.len() == 1 && segments[0].ident.to_string().as_str() == "top_level"
        } else {
            false
        }
    })
}

fn setter_for_field(f: &syn::Field, t: &syn::Type) -> proc_macro2::TokenStream {
    let name = f.ident.clone();
    quote! {
        pub fn #name(mut self, v: #t) -> Self {
            self.#name = v;
            self
        }
    }
}

fn setter_for_optional_field(f: &syn::Field, t: &syn::Type) -> proc_macro2::TokenStream {
    let name = f.ident.clone();
    let doc_comments = find_name_value_attr(&f.attrs, "doc")
        .and_then(|lit| match &lit.lit {
            syn::Lit::Str(_) => {
                let lit = lit.lit.clone();
                Some(quote!(#[doc=#lit]))
            }
            _ => None,
        })
        .unwrap_or_else(|| quote!());

    quote! {
        #doc_comments
        pub fn #name(mut self, v: #t) -> Self {
            self.#name = Some(v);
            self
        }
    }
}
