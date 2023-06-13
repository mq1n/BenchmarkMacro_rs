extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn benchmark(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let on_complete = syn::parse_macro_input!(attr as syn::Expr);

    let output = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_block
            let duration = start.elapsed().as_nanos();
            (#on_complete)(duration);
        }
    };

    output.into()
}