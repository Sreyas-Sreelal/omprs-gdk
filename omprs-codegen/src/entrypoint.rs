use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn create_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as ItemFn);
    let body = sig.block.stmts;
    let code = quote! {
        #[no_mangle]
        pub extern "C" fn OMPRS_Main() {
            omprs::init_functions();
            #(#body)*
        }
    };
    code.into()
}
