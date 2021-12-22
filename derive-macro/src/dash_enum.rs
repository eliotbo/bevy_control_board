// TODO: a test that attemps to collide two types with the same name (with derive(Dashboard))
// in two different files

use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
// use proc_macro2::{Span};
use quote::quote;
use syn::DataEnum;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DashMacroError {
    #[error("dummy")]
    MyError,
    // #[error("taht did not worked worked... bummer")]
    // QueryEntityError(#[from] bevy::ecs::query::QueryEntityError),
}

pub fn derive_dashboard_impl_enum(
    type_name: syn::Ident,
    data: syn::Data,
) -> Result<TokenStream, DashMacroError> {
    //
    match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let variant_name_str = variants
                .iter()
                .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
                .map(|f| {
                    let w = &f.ident;
                    quote![stringify!(#type_name::#w)]
                })
                .collect::<Vec<TokenStream2>>();

            // let variant_name_ident = variants
            //     .iter()
            //     .enumerate()
            //     .filter(|(k, f)| f.attrs.is_empty() || !variant_is_ignored(f))
            //     .map(|(k, f)| {
            //         let w = &f.ident;
            //         quote![#type_name::#w]
            //     })
            //     .collect::<Vec<TokenStream2>>();

            let variant_name_ident_unit = variants
                .iter()
                .enumerate()
                .filter(|(_k, f)| {
                    (f.attrs.is_empty() || !variant_is_ignored(f)) && variant_is_unit(f)
                })
                .map(|(_k, f)| {
                    let w = &f.ident;
                    quote![#type_name::#w]
                })
                .collect::<Vec<TokenStream2>>();

            let from_enum_to_dash = variants
                .iter()
                .enumerate()
                .map(|(k, f)| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let k_f64 = k as f64;
                    let digit = quote![#k_f64];
                    let matches = make_enum_to_dash(variant_name_tokens, fields, digit);
                    // fields
                    matches
                })
                .collect::<Vec<TokenStream2>>();

            let from_dash_to_enum2 = variants
                .iter()
                .enumerate()
                .map(|(k, f)| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let k_f64 = k as f64 - 0.000001 + 1.0;
                    let digit = quote![#k_f64];
                    let matches = make_dash_to_enum(variant_name_tokens, fields, digit);
                    // fields
                    matches
                })
                .collect::<Vec<TokenStream2>>();

            let make_knob_named_and_unnamed = variants
                .iter()
                // we could add an additional filter for Unit variants to be ignored and treat them independently
                .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
                .map(|f| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let matches = make_enum_make_knob_match(variant_name_tokens, fields);
                    // fields
                    matches
                })
                // .collect::<Vec<&syn::Fields>>();
                .collect::<Vec<TokenStream2>>();

            let get_structure_named_and_unnamed = variants
                .iter()
                // we could add an additional filter for Unit variants to be ignored and treat them independently
                .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
                .map(|f| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let matches = make_enum_get_structure(variant_name_tokens, fields);
                    // fields
                    matches
                })
                // .collect::<Vec<&syn::Fields>>();
                .collect::<Vec<TokenStream2>>();

            let get_string_named_and_unnamed = variants
                .iter()
                // we could add an additional filter for Unit variants to be ignored and treat them independently
                .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
                .map(|f| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let matches = make_enum_get_string_match(variant_name_tokens, fields);
                    // fields
                    matches
                })
                // .collect::<Vec<&syn::Fields>>();
                .collect::<Vec<TokenStream2>>();

            let update_value_named_and_unnamed = variants
                .iter()
                // we could add an additional filter for Unit variants to be ignored and treat them independently
                .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
                .map(|f| {
                    let variant_name2 = &f.ident;
                    let variant_name_tokens = quote![#type_name :: #variant_name2];
                    let fields = &f.fields;
                    let matches = make_enum_update_value_match(variant_name_tokens, fields);
                    // fields
                    matches
                })
                // .collect::<Vec<&syn::Fields>>();
                .collect::<Vec<TokenStream2>>();

            // eprintln!("{:?}", variants_fields_filtered);
            let to_expand = quote! {
                impl From<#type_name> for DashF64 {
                    fn from(my_enum: #type_name) -> Self {
                        match my_enum {
                            #( #from_enum_to_dash )*

                            // MyEnum::A(_, _) => DashF64(0.0),
                            // MyEnum::B(_) => DashF64(1.0),
                            // MyEnum::C { .. } => DashF64(2.0),
                            // MyEnum::D => DashF64(3.0),
                        }
                    }
                }

                impl From<DashF64> for #type_name {
                    fn from(dash_value: DashF64) -> Self {
                        let num = dash_value.get_value();

                        // match true {
                        //     true if num < 0.9999 => MyEnum::A(<f64>::default, DoubleInner::default()),
                        //     true if num < 1.9999 => MyEnum::B(DoubleInner::default()),
                        //     _ => MyEnum::D,
                        // }

                        match true {
                            #( #from_dash_to_enum2 )*
                            _ => panic!("invalid value for enum {}", stringify!(#type_name)),
                        }

                    }
                }

                impl Dashboard for MyEnum {
                    fn get_string_value(&self, path: &str) -> String {
                        if let Some((prefix, suffix)) = path.split_once(".") {
                            let value = match prefix {
                                #(
                                    #variant_name_str => {
                                        match self {
                                            #get_string_named_and_unnamed
                                            _ => "".to_string(),
                                        }
                                    }
                                )*
                                _ => "".to_string(),
                            };
                            return value;
                        } else {
                            // leaf of structure
                            match path {
                                #( #variant_name_str => #variant_name_str.to_string(), )*
                                _ => "".to_string(),
                            }
                        }

                    }

                    fn update_value(&mut self, path: &str, dash_value: &DashF64) {
                        if let Some((prefix, suffix)) = path.split_once(".") {
                            match prefix {
                                #(
                                    #variant_name_str => {
                                        match self {
                                            #update_value_named_and_unnamed
                                            _ => {}
                                        }
                                    }
                                )*
                                _ => {}
                            }
                        } else {
                            // leaf of structure
                            match path {
                                #( stringify!(#variant_name_ident_unit)  => {
                                            *self = #type_name::from(*dash_value);
                                        } )*
                                _ => {}
                            }
                        }
                    }

                    fn get_structure() -> Vec<String> {
                        let mut v = vec![];
                        #( #get_structure_named_and_unnamed )*
                        return v;
                    }

                    fn get_type_name() -> String {
                        stringify!(#type_name).to_string()
                    }

                    fn get_value<T: std::any::Any + std::fmt::Debug>(&self, path: &str) -> Option<&T> {
                        unimplemented!()
                    }

                    fn make_knob(&self, path: &str, enabled: bool) -> Option<KnobType> {
                        if let Some((prefix, suffix)) = path.split_once(".") {
                            let value = match prefix {
                                #(
                                    #variant_name_str => {
                                        match self {
                                            #make_knob_named_and_unnamed
                                            _ => None,
                                        }
                                    }
                                )*
                                _ => None,
                            };
                            return value;
                        }
                        // else { return None }

                        else {
                            // leaf of structure
                            match path {
                                #( stringify!(#variant_name_ident_unit) => Some(KnobType::Discrete(LinearKnob::<i64>::new(
                                    <DashF64>::from(#variant_name_ident_unit).get_value() as i64, enabled
                                    ))),
                                )*
                                _ => None,
                            }
                        }
                    }



                }

            };

            // eprintln!("{:#?}", to_expand.to_string());

            return Ok(TokenStream::from(to_expand));
        }

        _ => {}
    }
    return Err(DashMacroError::MyError);
}

fn make_enum_get_string_match(
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // good luck working this out...
            let unnamed = field_unnamed.unnamed.clone();
            // let unnamed_types = unnamed.iter().map(|x| x.ty.clone());
            let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

            let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
                let concatenated = format!("inner_{}", k);
                let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
                varname
            });

            let unnamed_idx2 = unnamed_idx.clone();
            let inner_name2 = inner_name.clone();
            let inner_name3 = inner_name.clone();

            // path.segments[0].ident);
            let unnamed_case = quote![
                    #variant_name_tokens ( #(#inner_name2, )* ) => {
                        if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                            match inner_pre {
                                #( #unnamed_idx => #inner_name.get_string_value(inner_suf), )*
                                _ => "".to_string(),
                            }
                        } else {
                            // leaf
                            match suffix {
                                #( #unnamed_idx2 => #inner_name3.get_string_value(""), )*
                                _ => "".to_string(),
                            }
                        }
                    }
                    _ => "".to_string(),

            ];

            return unnamed_case;
        }
        syn::Fields::Named(field_named) => {
            let named = field_named.named.clone();
            let named_name = named.iter().map(|x| x.ident.clone());
            let named_name2 = named_name.clone();
            let named_name4 = named_name.clone();

            let named_case = quote![
                #variant_name_tokens  { #( #named_name, )* } => {
                    if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                        match inner_pre {
                            #( stringify!(#named_name2) => #named_name2.get_string_value(inner_suf), )*
                            _ => "".to_string(),
                        }
                    } else {
                        // leaf
                        match suffix {
                            #( stringify!(#named_name4) => #named_name4.get_string_value(""), )*
                            _ => "".to_string(),
                        }
                    }
                }
                _ => "".to_string(),
            ];

            return named_case;
        }
        _ => quote![],
        // syn::Fields::Unit => {
        //     let unit_case = quote![
        //         stringify!(#variant_name_tokens) => stringify!(#variant_name_tokens) .to_string(),
        //     ];
        //     return unit_case;
        // }
    };
    quote![]
}

fn make_enum_make_knob_match(
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // good luck working this out...
            let unnamed = field_unnamed.unnamed.clone();
            let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

            let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
                let concatenated = format!("inner_{}", k);
                let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
                varname
            });

            let inner_type = unnamed.iter().map(|x| &x.ty);
            let inner_type2 = inner_type.clone();

            let unnamed_idx2 = unnamed_idx.clone();
            let unnamed_idx3 = unnamed_idx.clone();
            let unnamed_idx4 = unnamed_idx.clone();
            let inner_name2 = inner_name.clone();
            let inner_name3 = inner_name.clone();
            let inner_name4 = inner_name.clone();

            // path.segments[0].ident);
            let unnamed_case = quote![
                    #variant_name_tokens ( #(#inner_name2, )* ) => {
                        if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                            match inner_pre {
                                #( #unnamed_idx => #inner_name.make_knob(inner_suf, enabled), )*
                                _ => None,
                            }
                        } else {
                            // leaf
                            match suffix {
                                #( #unnamed_idx2 => #inner_name3.make_knob("", enabled), )*
                                _ => None,
                            }
                        }
                    }
                    _ => {
                        if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                            match inner_pre {
                                // TODO: propagate make_konb to inner
                                #( #unnamed_idx4 => <#inner_type2>::default().make_knob(inner_suf, false), )*
                                _ => None,
                            }
                        } else {
                            // leaf
                            match suffix {
                                // TODO: error message for when a type is required to have a default value
                                #( #unnamed_idx3 => <#inner_type>::default().make_knob("", false), )*
                                _ => None,
                            }
                        }
                    },
                    // _ => None,

            ];

            return unnamed_case;
        }
        syn::Fields::Named(field_named) => {
            let named = field_named.named.clone();
            let named_name = named.iter().map(|x| x.ident.clone());
            let named_name2 = named_name.clone();
            let named_name4 = named_name.clone();

            let named_case = quote![
                #variant_name_tokens  { #( #named_name, )* } => {
                    if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                        match inner_pre {
                            #( stringify!(#named_name2) => #named_name2.make_knob(inner_suf, enabled), )*
                            _ => None,
                        }
                    } else {
                        // leaf
                        match suffix {
                            #( stringify!(#named_name4) => #named_name4.make_knob("", enabled), )*
                            _ => None,
                        }
                    }
                }
                _ => None,
            ];

            return named_case;
        }

        syn::Fields::Unit => {
            let unit_case = quote![
                // #( stringify!(#variant_name_ident_unit) =>  Some(KnobType::Discrete(LinearKnob::<i64>::new(
                //                     <DashF64>::from(#variant_name_ident_unit).get_value() as i64,
                //                     ))), )*
            ];
            return unit_case;
        }
    };
}

fn make_enum_update_value_match(
    // variant_name: Vec<TokenStream2>,
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // good luck working this out...
            let unnamed = field_unnamed.unnamed.clone();

            let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

            let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
                let concatenated = format!("inner_{}", k);
                let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
                varname
            });

            let unnamed_idx2 = unnamed_idx.clone();
            let inner_name2 = inner_name.clone();
            let inner_name3 = inner_name.clone();

            // path.segments[0].ident);
            let unnamed_case = quote![
                    #variant_name_tokens ( #(#inner_name2, )* ) => {
                        if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                            match inner_pre {
                                #( #unnamed_idx => #inner_name.update_value(inner_suf, dash_value), )*
                                _ => {}
                            }
                        } else {
                            // leaf
                            match suffix {
                                #( #unnamed_idx2 => #inner_name3.update_value("", dash_value), )*
                                _ => {}
                            }
                        }
                    }
                    _ => {}

            ];

            return unnamed_case;
        }
        syn::Fields::Named(field_named) => {
            let named = field_named.named.clone();
            let named_name = named.iter().map(|x| x.ident.clone());
            let named_name2 = named_name.clone();
            let named_name4 = named_name.clone();

            let named_case = quote![
                #variant_name_tokens  { #( #named_name, )* } => {
                    if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
                        match inner_pre {
                            #( stringify!(#named_name2) => #named_name2.update_value(inner_suf, dash_value), )*
                            _ => {}
                        }
                    } else {
                        // leaf
                        match suffix {
                            #( stringify!(#named_name4) => #named_name4.update_value("", dash_value), )*
                            _ => {}
                        }
                    }
                }
                _ => {}
            ];

            return named_case;
        }
        _ => quote![],
        // syn::Fields::Unit => {
        //     let unit_case = quote![
        //         stringify!(#variant_name_tokens) => stringify!(#variant_name_tokens) .to_string(),
        //     ];
        //     return unit_case;
        // }
    };
    quote![]
}

fn make_enum_get_structure(
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // good luck working this out...
            let unnamed = field_unnamed.unnamed.clone();
            let unnamed_types = unnamed.iter().map(|x| x.ty.clone());
            let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

            let unnamed_case = quote![
                #({
                    let mut re: Vec<String> = Vec::new();
                    <#unnamed_types as Dashboard>::get_structure().iter().for_each(|x| {
                        let mut s = stringify!(#variant_name_tokens).to_string();
                        s.push('.');
                        s.push_str(#unnamed_idx);

                        if x != "" { // if the field doesn't have a sub-field, don't add a dot
                            s.push('.');
                            s.push_str(&x);
                        }

                        re.push(s.clone())
                    });
                    v.append(&mut re);
                })*


            ];

            return unnamed_case;
        }
        syn::Fields::Named(field_named) => {
            let named = field_named.named.clone();
            let named_types = named.iter().map(|x| x.ty.clone());
            let named_name = named.iter().map(|x| x.ident.clone());

            let named_case = quote![

                #({
                    let mut re: Vec<String> = Vec::new();
                    <#named_types as Dashboard>::get_structure().iter().for_each(|x| {
                        let mut s = stringify!(#variant_name_tokens).to_string();
                        s.push('.');
                        s.push_str(stringify!(#named_name));
                        if x != "" { // if the field doesn't have a sub-field, don't add a dot
                            s.push('.');
                            s.push_str(&x);
                        }

                        re.push(s.clone())
                    });
                    v.append(&mut re);
                })*

            ];

            return named_case;
        }

        syn::Fields::Unit => {
            let unit_case = quote![
                let mut re = vec![stringify!(#variant_name_tokens).to_string()];
                v.append(&mut re);
            ];
            return unit_case;
        }
    };
}

fn make_enum_to_dash(
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
    digit: TokenStream2,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // num_of_fields = field_unnamed.unnamed.len();
            let underscore = (0..field_unnamed.unnamed.len())
                .map(|_x| syn::Ident::new("_", proc_macro2::Span::call_site()));

            quote![ #variant_name_tokens ( #( #underscore, )* ) => DashF64(#digit), ]
        }

        syn::Fields::Named(_) => {
            quote![ #variant_name_tokens { .. } => DashF64(#digit), ]
        }

        syn::Fields::Unit => {
            quote![ #variant_name_tokens  => DashF64(#digit),  ]
        }
    }
}

fn make_dash_to_enum(
    variant_name_tokens: TokenStream2,
    fields: &syn::Fields,
    digit: TokenStream2,
) -> TokenStream2 {
    match fields {
        syn::Fields::Unnamed(field_unnamed) => {
            // num_of_fields = field_unnamed.unnamed.len();
            let unnamed = field_unnamed.unnamed.clone();
            let unnamed_types = unnamed.iter().map(|x| x.ty.clone());

            quote![ true if num < #digit => #variant_name_tokens ( #( <#unnamed_types>::default() , )* ),  ]
            // quote![]
        }

        syn::Fields::Named(field_named) => {
            let named = field_named.named.clone();
            let named_name = named.iter().map(|x| x.ident.clone());
            let named_types = named.iter().map(|x| x.ty.clone());
            quote![ true if num < #digit => #variant_name_tokens { #( #named_name: <#named_types>::default(), )* } , ]
            // quote![]
        }

        syn::Fields::Unit => {
            quote![ true if num < #digit => #variant_name_tokens ,  ]
            // quote![]
        }
    }
}

fn variant_is_ignored(f: &syn::Variant) -> bool {
    f.attrs.iter().any(|x| {
        x.path
            .segments
            .iter()
            .find(|x| x.ident == "ignore")
            .is_some()
    })
}

fn variant_is_unit(f: &syn::Variant) -> bool {
    match f.fields {
        syn::Fields::Unit => true,
        _ => false,
    }
}

// impl From<MyEnum> for DashF64 {
//     fn from(my_enum: MyEnum) -> Self {
//         match my_enum {
//             MyEnum::A(_, _) => DashF64(0.0),
//             MyEnum::B(_) => DashF64(1.0),
//             MyEnum::C { .. } => DashF64(2.0),
//             MyEnum::D => DashF64(3.0),
//         }
//     }
// }

// impl From<DashF64> for MyEnum {
//     fn from(dash_value: DashF64) -> Self {
//         let num = dash_value.get_value();
//         if num < 0.5 {
//             MyEnum::A(0.0, DoubleInner::default())
//         } else if num < 1.5 {
//             MyEnum::B(DoubleInner::default())
//         } else if num < 2.5 {
//             MyEnum::C {
//                 double: DoubleInner::default(),
//                 leaf: 0,
//             }
//         } else {
//             MyEnum::D
//         }
//     }
// }

// impl Dashboard for MyEnum {
// fn get_string_value(&self, path: &str) -> String {
//     #[macro_export]
//     macro_rules! print_err {
//         ( ) => {{
//             // println!("self ({:?}) does not match with path ({})", self, path);
//             "".to_string()
//         }};
//     }

//     if let Some((prefix, suffix)) = path.split_once(".") {
//         let value = match prefix {
//             "MyEnum::A" => {
//                 match self {
//                     MyEnum::A(a, b) => {
//                         if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                             match inner_pre {
//                                 "1" => b.get_string_value(inner_suf),
//                                 _ => print_err![],
//                             }
//                         } else {
//                             // leaf
//                             match suffix {
//                                 "0" => a.get_string_value(""),
//                                 _ => print_err![],
//                             }
//                         }
//                     }
//                     _ => print_err![],
//                 }
//             }

//             "MyEnum::B" => {
//                 println!("path {} is ignored in self ({:?}) ", path, self);
//                 "".to_string()
//             }
//             "MyEnum::C" => match self {
//                 MyEnum::C { double, leaf } => {
//                     if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                         match inner_pre {
//                             "double" => double.get_string_value(inner_suf),
//                             "leaf" => leaf.get_string_value(inner_suf),
//                             _ => print_err![],
//                         }
//                     } else {
//                         match suffix {
//                             "leaf" => leaf.get_string_value(""),
//                             _ => print_err![],
//                         }
//                     }
//                 }

//                 _ => print_err![],
//             },
//             _ => print_err![],
//         };
//         return value;
//     } else {
//         // leaf of a structure
//         match path {
//             "MyEnum::D" => "MyEnum::D".to_string(),
//             _ => print_err![],
//         }
//     }
// }

// fn update_value(&mut self, path: &str, dash_value: &DashF64) {
//     #[macro_export]
//     macro_rules! print_err2 {
//         ( ) => {{
//             // println!("self ({:?}) does not match with path ({})", self, path);
//         }};
//     }

//     if let Some((prefix, suffix)) = path.split_once(".") {
//         match prefix {
//             "MyEnum::A" => {
//                 match self {
//                     MyEnum::A(a, b) => {
//                         if let Some((pre, suf)) = suffix.split_once(".") {
//                             match pre {
//                                 "1" => b.update_value(suf, dash_value),
//                                 _ => print_err2![],
//                             }
//                         } else {
//                             // leaf
//                             match suffix {
//                                 "0" => a.update_value("", dash_value),
//                                 _ => print_err2![],
//                             }
//                         }
//                     }
//                     _ => print_err2![],
//                 }
//             }

//             "MyEnum::B" => {
//                 println!("path {} is ignored in self ({:?}) ", path, self);
//             }
//             "MyEnum::C" => match self {
//                 MyEnum::C { double, leaf } => {
//                     if let Some((pre, suf)) = suffix.split_once(".") {
//                         match pre {
//                             "double" => double.update_value(suf, dash_value),
//                             "leaf" => leaf.update_value(suf, dash_value),
//                             _ => print_err2![],
//                         }
//                     } else {
//                         match suffix {
//                             "leaf" => leaf.update_value("", dash_value),
//                             _ => print_err2![],
//                         }
//                     }
//                 }

//                 _ => print_err2![],
//             },
//             _ => print_err2![],
//         };
//     } else {
//         // leaf of a structure
//         match path {
//             "MyEnum::D" => {
//                 *self = MyEnum::from(*dash_value);
//             }
//             _ => print_err2![],
//         };
//     }
// }

//     fn get_structure() -> Vec<String> {
//         let mut v = vec![];

//         let mut re: Vec<String> = Vec::new();
//         <f32 as Dashboard>::get_structure().iter().for_each(|x| {
//             let mut s = "MyEnum::A.0".to_string();
//             if x != "" {
//                 // if the field doesn't have a sub-field, don't add a dot
//                 s.push('.');
//                 s.push_str(&x);
//             }

//             re.push(s.clone())
//         });
//         v.append(&mut re);

//         let mut re: Vec<String> = Vec::new();
//         <DoubleInner as Dashboard>::get_structure()
//             .iter()
//             .for_each(|x| {
//                 let mut s = "MyEnum::A.1".to_string();
//                 if x != "" {
//                     // if the field doesn't have a sub-field, don't add a dot
//                     s.push('.');
//                     s.push_str(&x);
//                 }

//                 re.push(s.clone())
//             });
//         v.append(&mut re);

//         // let mut re: Vec<String> = Vec::new();
//         // <DoubleInner as Dashboard>::get_structure()
//         //     .iter()
//         //     .for_each(|x| {
//         //         let mut s = "MyEnum::B.0".to_string();
//         //         if x != "" {
//         //             // if the field doesn't have a sub-field, don't add a dot
//         //             s.push('.');
//         //             s.push_str(&x);
//         //         }

//         //         re.push(s.clone())
//         //     });
//         // v.append(&mut re);

//         let mut re: Vec<String> = Vec::new();
//         <DoubleInner as Dashboard>::get_structure()
//             .iter()
//             .for_each(|x| {
//                 let mut s = "MyEnum::C.double".to_string();
//                 if x != "" {
//                     // if the field doesn't have a sub-field, don't add a dot
//                     s.push('.');
//                     s.push_str(&x);
//                 }

//                 re.push(s.clone())
//             });
//         v.append(&mut re);

//         let mut re: Vec<String> = Vec::new();
//         <i64 as Dashboard>::get_structure().iter().for_each(|x| {
//             let mut s = "MyEnum::C.leaf".to_string();
//             if x != "" {
//                 // if the field doesn't have a sub-field, don't add a dot
//                 s.push('.');
//                 s.push_str(&x);
//             }

//             re.push(s.clone())
//         });
//         v.append(&mut re);

//         // leaf
//         let mut re = vec!["MyEnum::D".to_string()];
//         v.append(&mut re);

//         return v;
//     }

//     fn get_type_name() -> String {
//         "MyEnum".to_string()
//     }

//     fn get_value<T: std::any::Any + std::fmt::Debug>(&self, path: &str) -> Option<&T> {
//         None
//     }

//     fn make_knob(&self, path: &str) -> Option<KnobType> {
//         #[macro_export]
//         macro_rules! print_err3 {
//             ( ) => {{
//                 println!("self ({:?}) does not match with path ({})", self, path);
//                 None
//             }};
//         }

//         if let Some((prefix, suffix)) = path.split_once(".") {
//             match prefix {
//                 "MyEnum::A" => {
//                     match self {
//                         MyEnum::A(a, b) => {
//                             if let Some((pre, suf)) = suffix.split_once(".") {
//                                 match pre {
//                                     "1" => b.make_knob(suf),
//                                     _ => print_err3![],
//                                 }
//                             } else {
//                                 // leaf
//                                 match suffix {
//                                     "0" => a.make_knob(""),
//                                     _ => print_err3![],
//                                 }
//                             }
//                         }
//                         _ => print_err3![],
//                     }
//                 }

//                 "MyEnum::B" => {
//                     println!("path {} is ignored in self ({:?}) ", path, self);
//                     None
//                 }
//                 "MyEnum::C" => match self {
//                     MyEnum::C { double, leaf } => {
//                         if let Some((pre, suf)) = suffix.split_once(".") {
//                             match pre {
//                                 "double" => double.make_knob(suf),
//                                 // "leaf" => leaf.make_knob(suf),
//                                 _ => print_err3![],
//                             }
//                         } else {
//                             match suffix {
//                                 "leaf" => leaf.make_knob(""),
//                                 _ => print_err3![],
//                             }
//                         }
//                     }

//                     _ => print_err3![],
//                 },
//                 _ => print_err3![],
//             }
//         } else {
//             // leaf of a structure
//             match path {
//                 "MyEnum::D" => Some(KnobType::Discrete(LinearKnob::<i64>::new(
//                     <DashF64>::from(MyEnum::D).get_value() as i64,
//                 ))),

//                 _ => print_err3![],
//             }
//         }
//     }
// }
