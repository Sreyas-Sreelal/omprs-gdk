use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::RArrow,
    Block, Ident, Token,
};
struct CreateCallback {
    name: Ident,
    params: Vec<(Ident, Ident)>,
    return_type: Option<Ident>,
    body: Block,
}

impl Parse for CreateCallback {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _: Token![fn] = input.parse()?;
        let name: Ident = input.parse()?;
        let params_input;
        let mut params = Vec::new();
        parenthesized!(params_input in input);
        let mut return_type: Option<Ident> = None;

        while !params_input.is_empty() {
            let param_name: Ident = params_input.parse()?;
            let _: Token![:] = params_input.parse()?;
            let param_type: Ident = params_input.parse()?;
            params.push((param_name, param_type));

            if params_input.peek(Token![,]) {
                let _: Token![,] = params_input.parse()?;
            }
        }

        if input.peek(RArrow) {
            let _: RArrow = input.parse()?;
            return_type = Some(input.parse()?);
        }

        let body: Block = input.parse()?;

        Ok(CreateCallback {
            name,
            params,
            return_type,
            body,
        })
    }
}
pub fn create_callback(_args: TokenStream, input: TokenStream) -> TokenStream {
    let callback = parse_macro_input!(input as CreateCallback);

    let user_func_name = callback.name;
    let mut user_func_params = Vec::new();
    let mut user_func_args = Vec::new();
    let user_func_body = callback.body;

    let orig_callback_name = Ident::new(&format!("OMPRS_{user_func_name}"), user_func_name.span());
    let mut orig_callback_params = Vec::new();

    for (param_name, param_type) in callback.params {
        user_func_params.push(quote!(#param_name:#param_type,));
        if param_type == "String" {
            orig_callback_params.push(quote!(#param_name:*const std::ffi::c_char,));
            user_func_args.push(quote!(unsafe { std::ffi::CStr::from_ptr(#param_name).to_string_lossy().to_string() },));
        } else {
            orig_callback_params.push(quote!(#param_name:#param_type,));
            user_func_args.push(quote!(#param_name,));
        }
    }

    let user_function = if let Some(ref return_type) = callback.return_type {
        quote!(
            #[allow(non_snake_case)]
            fn #user_func_name(#(#user_func_params)*) -> #return_type {
                #user_func_body
            }
        )
    } else {
        quote!(
            #[allow(non_snake_case)]
            fn #user_func_name(#(#user_func_params)*) {
                #user_func_body
            }
        )
    };

    let orig_function = if let Some(ref return_type) = callback.return_type {
        quote!(
            #[no_mangle]
            pub unsafe extern "C" fn #orig_callback_name(#(#orig_callback_params)*) -> #return_type {
                #user_func_name(#(#user_func_args)*)
            }
        )
    } else {
        quote!(
            #[no_mangle]
            pub unsafe extern "C" fn #orig_callback_name(#(#orig_callback_params)*) {
                #user_func_name(#(#user_func_args)*);
            }
        )
    };
    let code = quote! {
        #user_function
        #orig_function
    };

    code.into()
}
