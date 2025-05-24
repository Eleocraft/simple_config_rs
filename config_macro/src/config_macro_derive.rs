use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, PathSegment, Type};

fn get_parser(name: &Ident, path: &PathSegment) -> TokenStream {
    let ident = &path.ident;
    match ident.to_string().as_str() {
        // pass just the last item in the type path to match
        "String" => quote! {
            String::from(values.next().ok_or(format!("Missing value for {}", stringify!(#name)))?)
        },
        "bool" | "f64" | "f32" | "u64" | "u32" | "i64" | "i32" => quote! {
            values.next().ok_or(format!("Missing value for {}", stringify!(#name)))?.parse().map_err(|e| format!("could not parse value of type {} for field {}: {}", stringify!(#ident), stringify!(#name), e))?
        },
        "Vec" => {
            let args = if let PathArguments::AngleBracketed(a) = &path.arguments {
                a
            } else {
                panic!("not a vec type")
            };
            let inner_type = if let GenericArgument::Type(a) = &args.args[0] {
                a
            } else {
                panic!("not a vec type")
            };
            let inner_type_path = if let Type::Path(a) = inner_type {
                a
            } else {
                panic!("not a supported vec type")
            };
            let inner_type_path_seg = inner_type_path.path.segments.last().unwrap();
            let inner_type_parser = get_parser(name, inner_type_path_seg);
            quote! {{
                let starter = values.next().ok_or(format!("Vector {} must start with a [ followed by a space", stringify!(#name)))?;
                let mut vector = Vec::new();
                loop {
                    let value = values.next().ok_or(format!("Vector {} must end with a space followed by a ]", stringify!(#name)))?;

                    if value == "]" {
                        break vector;
                    }
                    let mut values = vec![ value ].into_iter();
                    vector.push(#inner_type_parser);
                }
            }}
        }
        _ => quote! {
            #path::parse_config(&mut values)?
        },
    }
}

fn get_help_params(path: &PathSegment) -> TokenStream {
    match path.ident.to_string().as_str() {
        "String" | "bool" | "f64" | "f32" | "u64" | "u32" | "i64" | "i32" => {
            let ident = &path.ident;
            quote!(format!("<{}>", stringify!(#ident)))
        }
        "Vec" => {
            let args = if let PathArguments::AngleBracketed(a) = &path.arguments {
                a
            } else {
                panic!("not a vec type")
            };
            let inner_type = if let GenericArgument::Type(a) = &args.args[0] {
                a
            } else {
                panic!("not a vec type")
            };
            let inner_type_path = if let Type::Path(a) = inner_type {
                a
            } else {
                panic!("not a supported vec type")
            };
            let inner_type_path_seg = inner_type_path.path.segments.last().unwrap();
            let inner_type_params = get_help_params(inner_type_path_seg);
            quote!(format!("[ {} ... ]", #inner_type_params))
        }
        _ => {
            quote!(format!("<{}>", #path::get_params()))
        }
    }
}

pub fn impl_config_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("alarm, not a struct");
    };
    let syn::Fields::Named(fields) = &data_struct.fields else {
        panic!("alarm, no named fields");
    };
    let fields = fields.named.iter().map(|v| {
        (
            v.ident.as_ref().unwrap(),
            if let Type::Path(ref path) = v.ty {
                path.path.segments.last()
            } else {
                panic!("invalid type")
            },
        )
    });
    let field_parser: Vec<_> = fields
        .clone()
        .map(|f| {
            let name = f.0;
            let parser = get_parser(name, f.1.unwrap());
            quote! {
                stringify!(#name) => {
                    self.#name = #parser;
                },
            }
        })
        .collect();

    let field_help: Vec<_> = fields
        .map(|f| {
            let name = f.0;
            let params = get_help_params(f.1.unwrap());
            quote!(format!("{} {}", stringify!(#name), #params))
        })
        .collect();

    quote! {
        impl Config for #name {
            fn add_source<'a>(&mut self, mut values: impl Iterator<Item = &'a str>) -> Result<(), String> {
                while let Some(value) = values.next() {
                    match value {
                        #( #field_parser )*
                        df => return Err(String::from(format!("Invalid setting: {}", df)))
                    }
                }
                Ok(())
            }
            fn get_help() -> String {
                format!("List of arguments with their respective parameters:\n{}", [#( #field_help ),*].join("\n"))
            }
        }
    }.into()
}
