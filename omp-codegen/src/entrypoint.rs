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


        }


        #[no_mangle]
        extern "C" fn onLoadCB() {}

        #[no_mangle]
        extern "C" fn onInitCB() {
            let _ = #function_name();
        }

        #[no_mangle]
        extern "C" fn onReadyCB() {}

        #[no_mangle]
        extern "C" fn onResetCB() {}

        #[no_mangle]
        extern "C" fn onFreeCB() {}

        #[no_mangle]
        unsafe extern "C" fn ComponentEntryPoint() -> *const std::ffi::c_void {
            omp::init_functions();

            let component = omp::Component_Create.unwrap()(
                0xA3CF477F384DEAFC,
                "omprs game mode".as_ptr().cast(),
                omp::ComponentVersion {
                    major: 0,
                    minor: 0,
                    patch: 1,
                    prerel: 0,
                },
                onLoadCB as *const std::ffi::c_void,
                onInitCB as *const std::ffi::c_void,
                onReadyCB as *const std::ffi::c_void,
                onResetCB as *const std::ffi::c_void,
                onFreeCB as *const std::ffi::c_void,
            );

            let _ = std::env::set_current_dir("../scriptfiles");

            component
        }
    };
    code.into()
}
