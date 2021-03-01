use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_planet(&ast)
}

fn impl_planet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / #name::ORBITAL_PERIOD
            }
        }
    };
    gen.into()
}
