// TODO: a test that attemps to collide two types with the same name (with derive(Dashboard))
// in two different files

use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
// use proc_macro2::{Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod dash_enum;
mod dash_struct;
use crate::dash_enum::*;
use crate::dash_struct::*;

// Combine derive macros together
#[proc_macro_attribute]
pub fn dashboard(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream2 = input.into();
    let output = quote! {
        #[derive(Debug, Dashboard, Component, Clone)]
        #input
    };
    output.into()
}

#[proc_macro_derive(Dashboard)]
pub fn derive_dashboard(input: TokenStream) -> TokenStream {
    //
    ////
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let type_name = ident;

    if let Ok(tokens) = derive_dashboard_impl_enum(type_name.clone(), data.clone()) {
        return tokens;
    } else if let Ok(tokens) = derive_dashboard_impl_struct(type_name, data) {
        return tokens;
    } else {
        let tok = quote! {};
        let tokens = TokenStream::from(tok);
        return tokens;
    }
}

// fn derive_dashboard_impl(
//     type_name: syn::Ident,
//     data: syn::Data,
// ) -> Result<TokenStream, Box<dyn Error>> {
//     //

//     match data {
//         syn::Data::Enum(DataEnum { variants, .. }) => {
//             let variant_name_str = variants
//                 .iter()
//                 .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
//                 .map(|f| {
//                     let w = &f.ident;
//                     quote![stringify!(#type_name::#w)]
//                 })
//                 .collect::<Vec<TokenStream2>>();

//             // let variant_name_ident = variants
//             //     .iter()
//             //     .enumerate()
//             //     .filter(|(k, f)| f.attrs.is_empty() || !variant_is_ignored(f))
//             //     .map(|(k, f)| {
//             //         let w = &f.ident;
//             //         quote![#type_name::#w]
//             //     })
//             //     .collect::<Vec<TokenStream2>>();

//             let variant_name_ident_unit = variants
//                 .iter()
//                 .enumerate()
//                 .filter(|(_k, f)| {
//                     (f.attrs.is_empty() || !variant_is_ignored(f)) && variant_is_unit(f)
//                 })
//                 .map(|(_k, f)| {
//                     let w = &f.ident;
//                     quote![#type_name::#w]
//                 })
//                 .collect::<Vec<TokenStream2>>();

//             let from_enum_to_dash = variants
//                 .iter()
//                 .enumerate()
//                 .map(|(k, f)| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let k_f64 = k as f64;
//                     let digit = quote![#k_f64];
//                     let matches = make_enum_to_dash(variant_name_tokens, fields, digit);
//                     // fields
//                     matches
//                 })
//                 .collect::<Vec<TokenStream2>>();

//             let from_dash_to_enum2 = variants
//                 .iter()
//                 .enumerate()
//                 .map(|(k, f)| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let k_f64 = k as f64 - 0.000001 + 1.0;
//                     let digit = quote![#k_f64];
//                     let matches = make_dash_to_enum(variant_name_tokens, fields, digit);
//                     // fields
//                     matches
//                 })
//                 .collect::<Vec<TokenStream2>>();

//             let make_knob_named_and_unnamed = variants
//                 .iter()
//                 // we could add an additional filter for Unit variants to be ignored and treat them independently
//                 .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
//                 .map(|f| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let matches = make_enum_make_knob_match(variant_name_tokens, fields);
//                     // fields
//                     matches
//                 })
//                 // .collect::<Vec<&syn::Fields>>();
//                 .collect::<Vec<TokenStream2>>();

//             let get_structure_named_and_unnamed = variants
//                 .iter()
//                 // we could add an additional filter for Unit variants to be ignored and treat them independently
//                 .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
//                 .map(|f| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let matches = make_enum_get_structure(variant_name_tokens, fields);
//                     // fields
//                     matches
//                 })
//                 // .collect::<Vec<&syn::Fields>>();
//                 .collect::<Vec<TokenStream2>>();

//             let get_string_named_and_unnamed = variants
//                 .iter()
//                 // we could add an additional filter for Unit variants to be ignored and treat them independently
//                 .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
//                 .map(|f| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let matches = make_enum_get_string_match(variant_name_tokens, fields);
//                     // fields
//                     matches
//                 })
//                 // .collect::<Vec<&syn::Fields>>();
//                 .collect::<Vec<TokenStream2>>();

//             let update_value_named_and_unnamed = variants
//                 .iter()
//                 // we could add an additional filter for Unit variants to be ignored and treat them independently
//                 .filter(|f| f.attrs.is_empty() || !variant_is_ignored(f))
//                 .map(|f| {
//                     let variant_name2 = &f.ident;
//                     let variant_name_tokens = quote![#type_name :: #variant_name2];
//                     let fields = &f.fields;
//                     let matches = make_enum_update_value_match(variant_name_tokens, fields);
//                     // fields
//                     matches
//                 })
//                 // .collect::<Vec<&syn::Fields>>();
//                 .collect::<Vec<TokenStream2>>();

//             // eprintln!("{:?}", variants_fields_filtered);
//             let to_expand = quote! {
//                 impl From<#type_name> for DashF64 {
//                     fn from(my_enum: #type_name) -> Self {
//                         match my_enum {
//                             #( #from_enum_to_dash )*

//                             // MyEnum::A(_, _) => DashF64(0.0),
//                             // MyEnum::B(_) => DashF64(1.0),
//                             // MyEnum::C { .. } => DashF64(2.0),
//                             // MyEnum::D => DashF64(3.0),
//                         }
//                     }
//                 }

//                 impl From<DashF64> for #type_name {
//                     fn from(dash_value: DashF64) -> Self {
//                         let num = dash_value.get_value();

//                         // match true {
//                         //     true if num < 0.9999 => MyEnum::A(<f64>::default, DoubleInner::default()),
//                         //     true if num < 1.9999 => MyEnum::B(DoubleInner::default()),
//                         //     _ => MyEnum::D,
//                         // }

//                         match true {
//                             #( #from_dash_to_enum2 )*
//                             _ => panic!("invalid value for enum {}", stringify!(#type_name)),
//                         }

//                     }
//                 }

//                 impl Dashboard for MyEnum {
//                     fn get_string_value(&self, path: &str) -> String {
//                         if let Some((prefix, suffix)) = path.split_once(".") {
//                             let value = match prefix {
//                                 #(
//                                     #variant_name_str => {
//                                         match self {
//                                             #get_string_named_and_unnamed
//                                             _ => "".to_string(),
//                                         }
//                                     }
//                                 )*
//                                 _ => "".to_string(),
//                             };
//                             return value;
//                         } else {
//                             // leaf of structure
//                             match path {
//                                 #( #variant_name_str => #variant_name_str.to_string(), )*
//                                 _ => "".to_string(),
//                             }
//                         }

//                     }

//                     fn update_value(&mut self, path: &str, dash_value: &DashF64) {
//                         if let Some((prefix, suffix)) = path.split_once(".") {
//                             match prefix {
//                                 #(
//                                     #variant_name_str => {
//                                         match self {
//                                             #update_value_named_and_unnamed
//                                             _ => {}
//                                         }
//                                     }
//                                 )*
//                                 _ => {}
//                             }
//                         } else {
//                             // leaf of structure
//                             match path {
//                                 #( stringify!(#variant_name_ident_unit)  => {
//                                             *self = #type_name::from(*dash_value);
//                                         } )*
//                                 _ => {}
//                             }
//                         }
//                     }

//                     fn get_structure() -> Vec<String> {
//                         let mut v = vec![];
//                         #( #get_structure_named_and_unnamed )*
//                         return v;
//                     }

//                     fn get_type_name() -> String {
//                         stringify!(#type_name).to_string()
//                     }

//                     fn get_value<T: std::any::Any + std::fmt::Debug>(&self, path: &str) -> Option<&T> {
//                         unimplemented!()
//                     }

//                     fn make_knob(&self, path: &str) -> Option<KnobType> {
//                         if let Some((prefix, suffix)) = path.split_once(".") {
//                             let value = match prefix {
//                                 #(
//                                     #variant_name_str => {
//                                         match self {
//                                             #make_knob_named_and_unnamed
//                                             _ => None,
//                                         }
//                                     }
//                                 )*
//                                 _ => None,
//                             };
//                             return value;
//                         }
//                         // else { return None }

//                         else {
//                             // leaf of structure
//                             match path {
//                                 #( stringify!(#variant_name_ident_unit) => Some(KnobType::Discrete(LinearKnob::<i64>::new(
//                                     <DashF64>::from(#variant_name_ident_unit).get_value() as i64,
//                                     ))),
//                                 )*
//                                 _ => None,
//                             }
//                         }
//                     }

//                 }

//             };

//             eprintln!("{:#?}", to_expand.to_string());

//             return Ok(TokenStream::from(to_expand));

//             // make_enum_dashboard_impl(variant_name, variants_tokens);
//         }

//         syn::Data::Struct(s) => {
//             let getter_setter = match s.fields {
//                 syn::Fields::Named(FieldsNamed { named, .. }) => {
//                     // let mut ya = named.iter();
//                     // let yo = ya.filter(|f| f.attrs.is_empty() || !is_ignored(f));

//                     let field_name = named
//                         .iter()
//                         .filter(|f| f.attrs.is_empty() || !is_ignored(f))
//                         .map(|f| {
//                             let w = &f.ident;
//                             quote![#w]
//                         })
//                         .collect::<Vec<TokenStream2>>();

//                     let field_type = named
//                         .iter()
//                         .filter(|f| f.attrs.is_empty() || !is_ignored(f))
//                         .map(|f| {
//                             let t = &f.ty;
//                             quote![#t]
//                         })
//                         .collect::<Vec<TokenStream2>>();

//                     // eprintln!("{:#?}", field_name);

//                     make_struct_dashboard_impl(&type_name, field_name, field_type)
//                 }
//                 _ => {
//                     quote! {
//                         impl Dashboard for #type_name {
//                             fn get_string_value(& self, path: &str) -> String  { "".to_string() }
//                             fn update_value<T: std::any::Any + std::fmt::Debug >(&mut self, path: &str, value: &T) {}
//                         }
//                     }
//                 }
//             };

//             let tokens = TokenStream::from(quote! {  #getter_setter });
//             return Ok(tokens);
//         }

//         _ => {}
//     }

//     let tok = quote! {};
//     let tokens = TokenStream::from(tok);
//     return Ok(tokens);
// }

// fn make_enum_get_string_match(
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // good luck working this out...
//             let unnamed = field_unnamed.unnamed.clone();
//             // let unnamed_types = unnamed.iter().map(|x| x.ty.clone());
//             let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

//             let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
//                 let concatenated = format!("inner_{}", k);
//                 let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
//                 varname
//             });

//             let unnamed_idx2 = unnamed_idx.clone();
//             let inner_name2 = inner_name.clone();
//             let inner_name3 = inner_name.clone();

//             // path.segments[0].ident);
//             let unnamed_case = quote![
//                     #variant_name_tokens ( #(#inner_name2, )* ) => {
//                         if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                             match inner_pre {
//                                 #( #unnamed_idx => #inner_name.get_string_value(inner_suf), )*
//                                 _ => "".to_string(),
//                             }
//                         } else {
//                             // leaf
//                             match suffix {
//                                 #( #unnamed_idx2 => #inner_name3.get_string_value(""), )*
//                                 _ => "".to_string(),
//                             }
//                         }
//                     }
//                     _ => "".to_string(),

//             ];

//             return unnamed_case;
//         }
//         syn::Fields::Named(field_named) => {
//             let named = field_named.named.clone();
//             let named_name = named.iter().map(|x| x.ident.clone());
//             let named_name2 = named_name.clone();
//             let named_name4 = named_name.clone();

//             let named_case = quote![
//                 #variant_name_tokens  { #( #named_name, )* } => {
//                     if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                         match inner_pre {
//                             #( stringify!(#named_name2) => #named_name2.get_string_value(inner_suf), )*
//                             _ => "".to_string(),
//                         }
//                     } else {
//                         // leaf
//                         match suffix {
//                             #( stringify!(#named_name4) => #named_name4.get_string_value(""), )*
//                             _ => "".to_string(),
//                         }
//                     }
//                 }
//                 _ => "".to_string(),
//             ];

//             return named_case;
//         }
//         _ => quote![],
//         // syn::Fields::Unit => {
//         //     let unit_case = quote![
//         //         stringify!(#variant_name_tokens) => stringify!(#variant_name_tokens) .to_string(),
//         //     ];
//         //     return unit_case;
//         // }
//     };
//     quote![]
// }

// // TODO
// fn make_enum_make_knob_match(
//     // type_name: &syn::Ident,
//     // variant_name: Vec<TokenStream2>,
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // good luck working this out...
//             let unnamed = field_unnamed.unnamed.clone();
//             let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

//             let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
//                 let concatenated = format!("inner_{}", k);
//                 let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
//                 varname
//             });

//             let unnamed_idx2 = unnamed_idx.clone();
//             let inner_name2 = inner_name.clone();
//             let inner_name3 = inner_name.clone();

//             // path.segments[0].ident);
//             let unnamed_case = quote![
//                     #variant_name_tokens ( #(#inner_name2, )* ) => {
//                         if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                             match inner_pre {
//                                 #( #unnamed_idx => #inner_name.make_knob(inner_suf), )*
//                                 _ => None,
//                             }
//                         } else {
//                             // leaf
//                             match suffix {
//                                 #( #unnamed_idx2 => #inner_name3.make_knob(""), )*
//                                 _ => None,
//                             }
//                         }
//                     }
//                     _ => None,

//             ];

//             return unnamed_case;
//         }
//         syn::Fields::Named(field_named) => {
//             let named = field_named.named.clone();
//             let named_name = named.iter().map(|x| x.ident.clone());
//             let named_name2 = named_name.clone();
//             let named_name4 = named_name.clone();

//             let named_case = quote![
//                 #variant_name_tokens  { #( #named_name, )* } => {
//                     if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                         match inner_pre {
//                             #( stringify!(#named_name2) => #named_name2.make_knob(inner_suf), )*
//                             _ => None,
//                         }
//                     } else {
//                         // leaf
//                         match suffix {
//                             #( stringify!(#named_name4) => #named_name4.make_knob(""), )*
//                             _ => None,
//                         }
//                     }
//                 }
//                 _ => None,
//             ];

//             return named_case;
//         }

//         syn::Fields::Unit => {
//             let unit_case = quote![
//                 // #( stringify!(#variant_name_ident_unit) =>  Some(KnobType::Discrete(LinearKnob::<i64>::new(
//                 //                     <DashF64>::from(#variant_name_ident_unit).get_value() as i64,
//                 //                     ))), )*
//             ];
//             return unit_case;
//         }
//     };
// }

// fn make_enum_update_value_match(
//     // variant_name: Vec<TokenStream2>,
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // good luck working this out...
//             let unnamed = field_unnamed.unnamed.clone();

//             let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

//             let inner_name = unnamed.iter().enumerate().map(|(k, _x)| {
//                 let concatenated = format!("inner_{}", k);
//                 let varname = syn::Ident::new(&concatenated, proc_macro2::Span::call_site());
//                 varname
//             });

//             let unnamed_idx2 = unnamed_idx.clone();
//             let inner_name2 = inner_name.clone();
//             let inner_name3 = inner_name.clone();

//             // path.segments[0].ident);
//             let unnamed_case = quote![
//                     #variant_name_tokens ( #(#inner_name2, )* ) => {
//                         if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                             match inner_pre {
//                                 #( #unnamed_idx => #inner_name.update_value(inner_suf, dash_value), )*
//                                 _ => {}
//                             }
//                         } else {
//                             // leaf
//                             match suffix {
//                                 #( #unnamed_idx2 => #inner_name3.update_value("", dash_value), )*
//                                 _ => {}
//                             }
//                         }
//                     }
//                     _ => {}

//             ];

//             return unnamed_case;
//         }
//         syn::Fields::Named(field_named) => {
//             let named = field_named.named.clone();
//             let named_name = named.iter().map(|x| x.ident.clone());
//             let named_name2 = named_name.clone();
//             let named_name4 = named_name.clone();

//             let named_case = quote![
//                 #variant_name_tokens  { #( #named_name, )* } => {
//                     if let Some((inner_pre, inner_suf)) = suffix.split_once(".") {
//                         match inner_pre {
//                             #( stringify!(#named_name2) => #named_name2.update_value(inner_suf, dash_value), )*
//                             _ => {}
//                         }
//                     } else {
//                         // leaf
//                         match suffix {
//                             #( stringify!(#named_name4) => #named_name4.update_value("", dash_value), )*
//                             _ => {}
//                         }
//                     }
//                 }
//                 _ => {}
//             ];

//             return named_case;
//         }
//         _ => quote![],
//         // syn::Fields::Unit => {
//         //     let unit_case = quote![
//         //         stringify!(#variant_name_tokens) => stringify!(#variant_name_tokens) .to_string(),
//         //     ];
//         //     return unit_case;
//         // }
//     };
//     quote![]
// }

// fn make_enum_get_structure(
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // good luck working this out...
//             let unnamed = field_unnamed.unnamed.clone();
//             let unnamed_types = unnamed.iter().map(|x| x.ty.clone());
//             let unnamed_idx = unnamed.iter().enumerate().map(|(k, _x)| format!("{}", k));

//             let unnamed_case = quote![
//                 #({
//                     let mut re: Vec<String> = Vec::new();
//                     <#unnamed_types as Dashboard>::get_structure().iter().for_each(|x| {
//                         let mut s = stringify!(#variant_name_tokens).to_string();
//                         s.push('.');
//                         s.push_str(#unnamed_idx);

//                         if x != "" { // if the field doesn't have a sub-field, don't add a dot
//                             s.push('.');
//                             s.push_str(&x);
//                         }

//                         re.push(s.clone())
//                     });
//                     v.append(&mut re);
//                 })*

//             ];

//             return unnamed_case;
//         }
//         syn::Fields::Named(field_named) => {
//             let named = field_named.named.clone();
//             let named_types = named.iter().map(|x| x.ty.clone());
//             let named_name = named.iter().map(|x| x.ident.clone());

//             let named_case = quote![

//                 #({
//                     let mut re: Vec<String> = Vec::new();
//                     <#named_types as Dashboard>::get_structure().iter().for_each(|x| {
//                         let mut s = stringify!(#variant_name_tokens).to_string();
//                         s.push('.');
//                         s.push_str(stringify!(#named_name));
//                         if x != "" { // if the field doesn't have a sub-field, don't add a dot
//                             s.push('.');
//                             s.push_str(&x);
//                         }

//                         re.push(s.clone())
//                     });
//                     v.append(&mut re);
//                 })*

//             ];

//             return named_case;
//         }

//         syn::Fields::Unit => {
//             let unit_case = quote![
//                 let mut re = vec![stringify!(#variant_name_tokens).to_string()];
//                 v.append(&mut re);
//             ];
//             return unit_case;
//         }
//     };
// }

// fn make_enum_to_dash(
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
//     digit: TokenStream2,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // num_of_fields = field_unnamed.unnamed.len();
//             let underscore = (0..field_unnamed.unnamed.len())
//                 .map(|_x| syn::Ident::new("_", proc_macro2::Span::call_site()));

//             quote![ #variant_name_tokens ( #( #underscore, )* ) => DashF64(#digit), ]
//         }

//         syn::Fields::Named(_) => {
//             quote![ #variant_name_tokens { .. } => DashF64(#digit), ]
//         }

//         syn::Fields::Unit => {
//             quote![ #variant_name_tokens  => DashF64(#digit),  ]
//         }
//     }
// }

// fn make_dash_to_enum(
//     variant_name_tokens: TokenStream2,
//     fields: &syn::Fields,
//     digit: TokenStream2,
// ) -> TokenStream2 {
//     match fields {
//         syn::Fields::Unnamed(field_unnamed) => {
//             // num_of_fields = field_unnamed.unnamed.len();
//             let unnamed = field_unnamed.unnamed.clone();
//             let unnamed_types = unnamed.iter().map(|x| x.ty.clone());

//             quote![ true if num < #digit => #variant_name_tokens ( #( <#unnamed_types>::default() , )* ),  ]
//             // quote![]
//         }

//         syn::Fields::Named(field_named) => {
//             let named = field_named.named.clone();
//             let named_name = named.iter().map(|x| x.ident.clone());
//             let named_types = named.iter().map(|x| x.ty.clone());
//             quote![ true if num < #digit => #variant_name_tokens { #( #named_name: <#named_types>::default(), )* } , ]
//             // quote![]
//         }

//         syn::Fields::Unit => {
//             quote![ true if num < #digit => #variant_name_tokens ,  ]
//             // quote![]
//         }
//     }
// }

// fn make_struct_dashboard_impl(
//     type_name: &syn::Ident,
//     field_name: Vec<TokenStream2>,
//     field_type: Vec<TokenStream2>,
// ) -> TokenStream2 {
//     quote![
//         impl Dashboard for #type_name {
//             fn get_string_value(&self, path: &str) -> String {
//                 if let Some((prefix,suffix)) = path.split_once(".") {
//                     let value = match prefix {
//                         #(  stringify!(#field_name) => self.#field_name.get_string_value(&suffix), )*
//                         _ => "".to_string(),
//                     };
//                     return value;
//                 } else {
//                     // leaf of a structure
//                     match path {
//                         #( stringify!(#field_name) => self.#field_name.get_string_value(""), )*
//                         _ => {
//                             eprintln!("{} not found in {}", path, stringify!(#type_name));
//                             "".to_string()
//                         },
//                     }
//                 }

//             }

//             fn update_value(&mut self, path: &str, value: &DashF64) {
//                 if let Some((prefix,suffix)) = path.split_once(".") {
//                     match prefix {
//                         #(  stringify!(#field_name) => self.#field_name.update_value(&suffix, value), )*
//                         _ => {
//                             eprintln!("{} not found in {}", path, stringify!(#type_name));
//                         },
//                     }
//                 } else {
//                     match path {
//                         #( stringify!(#field_name) => self.#field_name.update_value("", &value ), )*
//                         _ => {
//                             eprintln!("{} not found in {}", path, stringify!(#type_name));
//                         },
//                     }
//                 }
//             }

//             fn get_structure() -> Vec<String> {
//                 let mut v = Vec::new();
//                 #({
//                     let mut re: Vec<String> = Vec::new();
//                     <#field_type as Dashboard>::get_structure().iter().for_each(|x| {
//                         let mut s = stringify!(#field_name).to_string();
//                         if x != "" { // if the field doesn't have a sub-field, don't add a dot
//                             s.push('.');
//                             s.push_str(&x);
//                         }

//                         re.push(s.clone())
//                     });
//                     v.append(&mut re);
//                 })*
//                 v

//             }

//             fn get_type_name() -> String {
//                 stringify!(#type_name).to_string()
//             }

//             fn get_value<T: std::any::Any + std::fmt::Debug>(&self, path: &str) -> Option<&T> {
//                 if let Some((prefix,suffix)) = path.split_once(".") {
//                     let value = match prefix {
//                         #(  stringify!(#field_name) => {
//                             self.#field_name.get_value::<T>(&suffix)
//                         } )*
//                         _ => None,
//                     };
//                     return value;
//                 } else {
//                     // leaf of a structure
//                     match path {
//                         #( stringify!(#field_name) => self.#field_name.get_value::<T>(""), )*
//                         _ => {
//                             eprintln!("{} not found in {}", path, stringify!(#type_name));
//                             None
//                         },
//                     }
//                 }
//              }

//             fn make_knob(&self, path: &str) -> Option<KnobType> {

//                 if let Some((prefix,suffix)) = path.split_once(".") {
//                     let value = match prefix {
//                         #(  stringify!(#field_name) => {
//                             self.#field_name.make_knob(&suffix)
//                         } )*
//                         _ => None,
//                     };
//                     return value;
//                 } else {
//                     // leaf of a structure
//                     match path {
//                         #( stringify!(#field_name) => self.#field_name.make_knob(""), )*
//                         _ => {
//                             eprintln!("{} not found in {}", path, stringify!(#type_name));
//                             None
//                         },
//                     }
//                 }
//              }

//         }
//     ]
// }

// fn is_ignored(f: &syn::Field) -> bool {
//     f.attrs.iter().any(|x| {
//         x.path
//             .segments
//             .iter()
//             .find(|x| x.ident == "ignore")
//             .is_some()
//     })
// }

// fn variant_is_ignored(f: &syn::Variant) -> bool {
//     f.attrs.iter().any(|x| {
//         x.path
//             .segments
//             .iter()
//             .find(|x| x.ident == "ignore")
//             .is_some()
//     })
// }

// fn variant_is_unit(f: &syn::Variant) -> bool {
//     match f.fields {
//         syn::Fields::Unit => true,
//         _ => false,
//     }
// }
