// TODO: a test that attemps to collide two types with the same name (with derive(Dashboard))
// in two different files

use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
// use proc_macro2::{Span};
use quote::quote;
use syn::FieldsNamed;

use crate::dash_enum::DashMacroError;

pub fn derive_dashboard_impl_struct(
    type_name: syn::Ident,
    data: syn::Data,
) -> Result<TokenStream, DashMacroError> {
    //

    match data {
        syn::Data::Struct(s) => {
            let getter_setter = match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    // let mut ya = named.iter();
                    // let yo = ya.filter(|f| f.attrs.is_empty() || !is_ignored(f));

                    let field_name = named
                        .iter()
                        .filter(|f| f.attrs.is_empty() || !is_ignored(f))
                        .map(|f| {
                            let w = &f.ident;
                            quote![#w]
                        })
                        .collect::<Vec<TokenStream2>>();

                    let field_type = named
                        .iter()
                        .filter(|f| f.attrs.is_empty() || !is_ignored(f))
                        .map(|f| {
                            let t = &f.ty;
                            quote![#t]
                        })
                        .collect::<Vec<TokenStream2>>();

                    // eprintln!("{:#?}", field_name);

                    make_struct_dashboard_impl(&type_name, field_name, field_type)
                }
                _ => {
                    quote! {
                        impl Dashboard for #type_name {
                            fn get_string_value(& self, path: &str) -> String  { "".to_string() }
                            fn update_value<T: std::any::Any + std::fmt::Debug >(&mut self, path: &str, value: &T) {}
                        }
                    }
                }
            };

            let tokens = TokenStream::from(quote! {  #getter_setter });
            return Ok(tokens);
        }

        _ => {}
    }

    return Err(DashMacroError::MyError);
}

fn make_struct_dashboard_impl(
    type_name: &syn::Ident,
    field_name: Vec<TokenStream2>,
    field_type: Vec<TokenStream2>,
) -> TokenStream2 {
    quote![
        impl Dashboard for #type_name {
            fn get_string_value(&self, path: &str) -> String {
                if let Some((prefix,suffix)) = path.split_once(".") {
                    let value = match prefix {
                        #(  stringify!(#field_name) => self.#field_name.get_string_value(&suffix), )*
                        _ => "".to_string(),
                    };
                    return value;
                } else {
                    // leaf of a structure
                    match path {
                        #( stringify!(#field_name) => self.#field_name.get_string_value(""), )*
                        _ => {
                            eprintln!("{} not found in {}", path, stringify!(#type_name));
                            "".to_string()
                        },
                    }
                }

            }

            fn update_value(&mut self, path: &str, value: &DashF64) {
                if let Some((prefix,suffix)) = path.split_once(".") {
                    match prefix {
                        #(  stringify!(#field_name) => self.#field_name.update_value(&suffix, value), )*
                        _ => {
                            eprintln!("{} not found in {}", path, stringify!(#type_name));
                        },
                    }
                } else {
                    match path {
                        #( stringify!(#field_name) => self.#field_name.update_value("", &value ), )*
                        _ => {
                            eprintln!("{} not found in {}", path, stringify!(#type_name));
                        },
                    }
                }
            }

            fn get_structure() -> Vec<String> {
                let mut v = Vec::new();
                #({
                    let mut re: Vec<String> = Vec::new();
                    <#field_type as Dashboard>::get_structure().iter().for_each(|x| {
                        let mut s = stringify!(#field_name).to_string();
                        if x != "" { // if the field doesn't have a sub-field, don't add a dot
                            s.push('.');
                            s.push_str(&x);
                        }

                        re.push(s.clone())
                    });
                    v.append(&mut re);
                })*
                v

            }

            fn get_type_name() -> String {
                stringify!(#type_name).to_string()
            }

            fn get_value<T: std::any::Any + std::fmt::Debug>(&self, path: &str) -> Option<&T> {
                if let Some((prefix,suffix)) = path.split_once(".") {
                    let value = match prefix {
                        #(  stringify!(#field_name) => {
                            self.#field_name.get_value::<T>(&suffix)
                        } )*
                        _ => None,
                    };
                    return value;
                } else {
                    // leaf of a structure
                    match path {
                        #( stringify!(#field_name) => self.#field_name.get_value::<T>(""), )*
                        _ => {
                            eprintln!("{} not found in {}", path, stringify!(#type_name));
                            None
                        },
                    }
                }
             }

            fn make_knob(&self, path: &str, enabled: bool) -> Option<KnobType> {

                if let Some((prefix,suffix)) = path.split_once(".") {
                    let value = match prefix {
                        #(  stringify!(#field_name) => {
                            self.#field_name.make_knob(&suffix, enabled)
                        } )*
                        _ => None,
                    };
                    return value;
                } else {
                    // leaf of a structure
                    match path {
                        #( stringify!(#field_name) => self.#field_name.make_knob("", enabled), )*
                        _ => {
                            eprintln!("{} not found in {}", path, stringify!(#type_name));
                            None
                        },
                    }
                }
             }



        }
    ]
}

fn is_ignored(f: &syn::Field) -> bool {
    f.attrs.iter().any(|x| {
        x.path
            .segments
            .iter()
            .find(|x| x.ident == "ignore")
            .is_some()
    })
}
