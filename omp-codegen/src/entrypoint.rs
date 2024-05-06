use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn create_main(_args: TokenStream, input: TokenStream) -> TokenStream {
    let sig = parse_macro_input!(input as ItemFn);
    let function_name = sig.clone().sig.ident;
    let code = quote! {
        #sig
        #[no_mangle]
        pub extern "C" fn OMPRS_Main() {
            let _ = std::env::set_current_dir("scriptfiles");
            omp::init_functions();
            let _ = #function_name();
        }
    };
    code.into()
}
