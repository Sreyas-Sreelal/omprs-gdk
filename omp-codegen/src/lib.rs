//! codgen crate that gives proc macros to generate exported functions and FFI related code automatically
use proc_macro::TokenStream;

mod callbacks;
mod entrypoint;
mod natives;

/// Creates a callback function for the component to call
#[proc_macro]
pub fn callback(args: TokenStream) -> TokenStream {
    callbacks::create_callback(args)
}

/// Creates an entry point function for the component to call when it loads
/// Entry point function is necessary, all the function address is calculated and initialised here
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    entrypoint::create_main(args, input)
}

/// Creates a userfunction that calls the native function in component
/// str - for string data
/// struct - for open.mp object pointers
#[proc_macro]
pub fn native(args: TokenStream) -> TokenStream {
    natives::create_native(args)
}
