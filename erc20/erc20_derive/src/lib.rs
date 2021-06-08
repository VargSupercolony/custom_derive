
extern crate proc_macro;


use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ERC20)]
pub fn erc20_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_erc20(&ast)
}

fn impl_erc20(ast: &syn::DeriveInput) -> TokenStream {
    let contract_name = &ast.ident;
    let gen = quote! {
        impl ERC20 for #contract_name {
            #[named]
            fn new(name: String, symbol: String, initial_supply: u128) -> Self {
                Self {
                    name,
                    symbol,
                    total_supply: initial_supply
                }
            }

            #[named]
            fn name(&self) -> &String {
                &self.name
            }

            #[named]
            fn symbol(&self) -> &String {
                &self.symbol
            }

            #[named]
            fn total_supply(&self) -> u128 {
                self.total_supply
            }

        }
    };

    gen.into()

}