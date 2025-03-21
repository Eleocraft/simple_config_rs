use proc_macro::TokenStream;
use syn::Fields;
use quote::quote;

pub fn impl_config_macro(ast: &syn::DeriveInput) -> TokenStream {
    
    let ident = &ast.ident;
    let name = ident.to_string();
    let syn::Data::Enum(data_enum) = &ast.data else { panic!("alarm, not an enum"); };
    let variant_parser: Vec<_> = data_enum.variants.iter().map(|v| if let Fields::Unit = v.fields {
        let ident = &v.ident;
        let name = ident.to_string();
        quote!{
            #name => Ok(Self::#ident),
        }
    } else { panic!("fields must be units") }).collect();
    let params: String = data_enum.variants.iter().map(|v| format!("{}|", v.ident.to_string())).collect();

    quote! {
        impl ConfigType for #ident {
            fn parse_config<'a>(mut values: impl Iterator<Item = &'a str>) -> Result<Self, String> {
                match values.next().ok_or(format!("Missing enum value for {}", #name))? {
                    #( #variant_parser )*
                    df => Err(format!("Invalid value for enum {}: {}", #name, df))
                }
            }
            fn get_params() -> String {
                #params
            }
        }
    }.into()
}
