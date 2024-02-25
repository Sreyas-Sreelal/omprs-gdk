use proc_macro::TokenStream;

mod callbacks;
mod entrypoint;

#[proc_macro_attribute]
pub fn callback(args: TokenStream, input: TokenStream) -> TokenStream {
    callbacks::create_callback(args, input)
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    entrypoint::create_main(args, input)
}
