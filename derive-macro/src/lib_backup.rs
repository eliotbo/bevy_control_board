// TODO: a test that attemps to collide two types with the same name (with derive(Dashboard))
// in two different files

use proc_macro::{self, TokenStream};
// use proc_macro2::{Span};
use quote::quote;
// use std::collections::HashMap;
// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;
use syn::{parse_macro_input, DataEnum, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(Dashboard)]
pub fn derive_dashboard(input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as DeriveInput);
    // let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let type_name = ident;

    match data {
        // if let syn::Data::Enum(DataEnum { variants, .. }) = data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            //
            // adding add_to_type_map for every local enum and structs
            let type_map_function = quote! {
                pub fn add_to_type_map(mut type_map: &mut TypeMap) {
                    //
                    let type_info = TypeInfo {
                        name: stringify!(#type_name).to_string(),
                        kind: TypeKind::Enum,
                    };
                    // type_map.insert(TypeId::of::<#type_name>(),  type_info.clone());
                    type_map.insert(stringify!(#type_name).to_string(),  type_info.clone());
                }

                pub fn add_to_named_struct_map(_named_struct_map: &mut NamedStructMap, _type_map: &TypeMap) {}

                pub fn get_type_id() -> TypeId {
                    TypeId::of::<#type_name>()
                }

            };

            // generating a default impl for enums
            let mut variants = variants.iter();
            if variants.clone().count() == 0 {
                panic!(
                    "{} is an empty enum: cannot derive Default with an empty enum",
                    stringify!(type_name)
                );
            }
            let first_variant = variants.next().unwrap(); // we can unwrap because enums are never empty
            let default_impl = quote! {
                impl Default for #type_name {
                    fn default() -> Self {
                        Self::#first_variant
                    }
                }
            };

            let tokens = TokenStream::from(quote! {
                #default_impl

                impl #type_name {
                    #type_map_function
                }
            });

            return tokens;
        }

        syn::Data::Struct(s) => {
            // eprintln!("{:#?}", s);

            let type_kind = match s.fields {
                syn::Fields::Named(FieldsNamed { .. }) => quote! {TypeKind::NamedStruct},
                syn::Fields::Unnamed(FieldsUnnamed { .. }) => quote! {TypeKind::UnnamedStruct},
                syn::Fields::Unit => quote! {TypeKind::UnitStruct},
            };

            // builds named_struct_map
            let type_fields_pair = match &s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    let field_names = named.iter().map(|f| {
                        let w = &f.ident;
                        let t = &f.ty;
                        // eprintln!("type Id!!!!!!!!: {:#?}", t);

                        // (name of field, type of coresponding field)
                        quote![(stringify!(#w), stringify!(#t))]
                    });
                    quote![{
                        let mut vvv: Vec<TypeInfo> = Vec::new();

                        #( {
                            let mut kind = TypeKind::Other;

                            let ty = #field_names.1.clone().split_whitespace().next().unwrap();

                            if '(' == ty.to_string().chars().next().unwrap() {
                                kind = TypeKind::Tuple;
                                //
                            } else if let Some(kind_local) = type_map.get(ty) {
                                kind = kind_local.kind.clone();
                            }
                            let what = TypeInfo {
                                name: (#field_names.0).to_string(),
                                // kind: type_map.get(stringify!(#field_names.1)).unwrap().kind.clone(),
                                kind,
                            };
                            vvv.push(what)
                        });*;

                        named_struct_map.insert(stringify!(#type_name).to_string(), vvv);
                    }]
                }
                syn::Fields::Unnamed(FieldsUnnamed { .. }) => quote! {},
                syn::Fields::Unit => quote! {},
            };

            // traversal of nested fields
            let string_path = match s.fields {
                syn::Fields::Named(FieldsNamed { .. }) => quote! {{
                    let dummy_instance = #type_name::default();
                    let mut v = Vec::new();

                    // scanning inner fields (should be a recursive call)
                    for (k, field) in dummy_instance.iter_fields().enumerate()  {
                        let field_name = dummy_instance.name_at(k).unwrap();
                        let mut long_name = stringify!(#type_name).to_string();
                        long_name.push('.');
                        long_name.push_str(field_name);

                        // map_field_name_types: map from field to its type
                        let type_string = #type_name::map_field_name_types(field_name);
                        //
                        if let Some(kind_of_inner_type) = type_info_map.get(&type_string) {
                            // println!("field {} for kind : {:#?}",  field_name, kind_of_inner_type);
                            match kind_of_inner_type.kind {
                                TypeKind::NamedStruct => {
                                    // Need a map from type to fields
                                    // named_structs_map, map from type of struct to its field infos
                                    // (name and kind of field)
                                    let struct_fields = named_structs_map.get(&type_string).unwrap();
                                    println!("");
                                    println!("explore deeper {:?}",  struct_fields);
                                    println!("");

                                    // scanning inner fields (should be a recursive call)
                                    for type_info in struct_fields {
                                        let mut inner_name = long_name.clone();
                                        inner_name.push('.');
                                        inner_name.push_str(type_info.name.as_str());
                                        v.push(inner_name.clone().to_string());
                                    }


                                }
                                _ => {  }
                            }
                        } else {
                            v.push(long_name.clone().to_string());
                        }


                        // println!("field {} for type : {:#?}",  field_name, type_string);
                    }
                    v
                }},
                syn::Fields::Unnamed(FieldsUnnamed { .. }) => {
                    quote! {vec![stringify!(#type_name).to_string()]}
                }
                syn::Fields::Unit => quote! {vec![stringify!(#type_name).to_string()]},
            };

            let mut named_only_quote = quote![];
            if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
                let name_and_types_iter = named.iter().map(|f| {
                    // quote![(&f.ident, &f.ty)];
                    let w = &f.ident;
                    let t = &f.ty;

                    //
                    quote![stringify!(#w) => stringify!(#t).to_string(), ]
                });
                // let types_iter = named.iter().map(|f| {
                //     quote![stringify!(#f.ty)];
                //     eprintln!("type Id!!!!!!!!: {:#?}", stringify!(&f.ty));
                //     // quote![stringify!(&f.ident)]
                // });
                named_only_quote = quote! {
                    pub fn map_field_name_types(field_name: &str) -> String {
                        //
                        let field_name_string = field_name.to_string();

                        match field_name {
                            #( #name_and_types_iter  )*
                            _ => "".to_string(),
                        }
                    }
                };
            };

            let type_map_function = quote! {
                //
                    pub fn add_to_type_map(mut type_map: &mut TypeMap) {
                        //
                        let type_info = TypeInfo {
                            name: stringify!(#type_name).to_string(),
                            kind: #type_kind,
                        };
                        // type_map.insert(TypeId::of::<#type_name>(),  type_info.clone());
                        type_map.insert(stringify!(#type_name).to_string(),  type_info.clone());
                    }

                    pub fn add_to_named_struct_map(mut named_struct_map: &mut NamedStructMap, type_map: &TypeMap) {
                        //
                        #type_fields_pair

                    }

                    // pub fn inspect(&self, mut type_map: &mut TypeMap) {

                    // }

                    pub fn make_paths( type_info_map: &TypeMap, named_structs_map: &NamedStructMap ) -> Vec<String> {
                        let mut paths_vec = vec![];
                        // {#( paths_vec.push( #paths )); *}
                        // {#( paths_vec.extend(  #string_path )); *}
                        paths_vec.extend(  #string_path );


                        paths_vec
                    }



            };

            // let inspection = match s.fields {
            //     syn::Fields::Named(FieldsNamed { named, .. }) => {
            //         let field_type = named.iter().map(|f| f.ty.clone());
            //         let field_name = named.iter().map(|f| f.ident.clone());
            //         // eprintln!("{:?}", field_type);
            //         quote! {
            //             //
            //             pub fn inspect(mut type_map: &mut TypeMap) {

            //             }
            //         }
            //     }
            //     syn::Fields::Unnamed(FieldsUnnamed { .. }) => quote! {},
            //     syn::Fields::Unit => quote! {},
            // };

            let tokens = TokenStream::from(quote! {
                impl #type_name {
                    #type_map_function
                    #named_only_quote
                }
            });
            return tokens;
        }

        _ => {}
    }

    let tok = quote! {};
    let tokens = TokenStream::from(tok);
    return tokens;
}

// fn is_registered(&self, type_registry: Res<TypeRegistry>) -> bool {
//     let type_registry = type_registry.read();
//     type_registry.get(TypeId::of::<Globals>()).is_some()
// }
