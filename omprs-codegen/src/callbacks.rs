use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn create_callback(_args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as ItemFn);

    let code = quote! {
        #[no_mangle]
        pub extern "C" #sig
    };
    code.into()
}
