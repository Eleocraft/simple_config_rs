use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Type;
use quote::quote;

pub fn impl_config_macro(ast: &syn::DeriveInput) -> TokenStream {
    
    let name = &ast.ident;
    let syn::Data::Struct(data_struct) = &ast.data else { panic!("alarm, not a struct"); };
    let syn::Fields::Named(fields) = &data_struct.fields else { panic!("alarm, no named fields"); };
    let fields = fields.named.iter().map(|v| (
        v.ident.as_ref().unwrap(),
        if let Type::Path(ref path) = v.ty { (path.path.to_token_stream(), path.path.segments.last().unwrap().ident.to_string()) } else { panic!("invalid type") })
    );
    let field_parser: Vec<_> = fields.clone().map(|f| {
        let ident = f.0;
        let name = ident.to_string();
        let ty = f.1.1.as_str();
        let ty_path = f.1.0;
        match ty { // pass just the last item in the type path to match
            "String" => quote! {
                #name => {
                    self.#ident = String::from(values.next().ok_or(format!("Missing value for {}", #name))?);
                },
            },
            "bool" | "f64" | "f32" | "u64" | "u32" | "i64" | "i32" => quote! {
                #name => {
                    self.#ident = values.next().ok_or(format!("Missing {}", #name))?.parse().map_err(|_| format!("could not parse {}", #name))?;
                },
            },
            _ => quote! {
                #name => {
                    self.#ident = #ty_path::parse_config(&mut values)?;
                },
            },
        }
    }).collect();

    let field_help: Vec<_> = fields.map(|f| {
        let ident = f.0;
        let name = ident.to_string();
        let ty = f.1.1.as_str();
        let ty_path = f.1.0;
        match ty {
            "String" | "bool" | "f64" | "f32" | "u64" | "u32" | "i64" | "i32" => {
                let help_str = format!("{} <{}>", name, ty);
                quote!(#help_str.into())
            },
            _ => {
                quote!(format!("{} <{}>", #name, #ty_path::get_params()))
            }
        }
    }).collect();

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
