use proc_macro::TokenStream;

mod callbacks;
mod entrypoint;
mod natives;

#[proc_macro]
pub fn callback(args: TokenStream) -> TokenStream {
    callbacks::create_callback(args)
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    entrypoint::create_main(args, input)
}

#[proc_macro]
pub fn native(args: TokenStream) -> TokenStream {
    natives::create_native(args)
}
