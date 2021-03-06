use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Attrs)]
pub fn attrs_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_attrs(&ast)
}

fn impl_attrs(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Attrs for #name {
            fn with_attrs(self, attrs: &[(&'static str, &'static str)]) -> Self {
                #name {
                    attrs: attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                    ..self
                }
            }

            fn get_attr(&self, name: &str) -> Option<&str> {
                self.attrs.get(name).map(|s| s.as_str())
            }
        }
    };
    gen.into()
}
