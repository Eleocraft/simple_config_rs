mod config_macro_derive;
mod config_enum_macro_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(Config)]
pub fn config_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    config_macro_derive::impl_config_macro(&ast)
}

#[proc_macro_derive(ConfigEnum)]
pub fn config_enum_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    config_enum_macro_derive::impl_config_macro(&ast)
}
