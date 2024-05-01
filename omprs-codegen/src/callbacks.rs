use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, Token, Type,
};
fn convert_to_snake_case(name: Ident) -> Ident {
    let characters = name.to_string();
    let mut characters = characters.chars();
    let mut out = String::new();
    out.push(characters.next().unwrap().to_ascii_lowercase());
    for x in characters {
        if x.is_uppercase() {
            out.push('_');
            out.push(x.to_ascii_lowercase());
        } else {
            out.push(x);
        }
    }
    Ident::new(&out, name.span())
}

#[derive(Clone)]
struct CreateCallback {
    name: Ident,
    params: Vec<(Ident, Ident, bool)>,
    return_type: Option<Type>,
}

impl Parse for CreateCallback {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name: Ident = input.parse()?;
        let mut params = Vec::new();
        let mut return_type = None;

        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            if input.peek(token::RArrow) {
                let _: token::RArrow = input.parse()?;
                return_type = Some(input.parse()?);
            } else {
                let param_name: Ident = input.parse()?;
                let _: Token![:] = input.parse()?;
                let param_type: Ident = input.parse()?;
                if param_type == "Option" {
                    let _: Token![<] = input.parse()?;
                    let param_type: Ident = input.parse()?;
                    let _: Token![>] = input.parse()?;
                    params.push((param_name, param_type, true));
                } else {
                    params.push((param_name, param_type, false));
                }
            }
        }
        Ok(CreateCallback {
            name,
            params,
            return_type,
        })
    }
}
pub fn create_callback(input: TokenStream) -> TokenStream {
    let callback = parse_macro_input!(input as CreateCallback);
    let user_func_name = callback.name;
    let orig_callback_name = Ident::new(&format!("OMPRS_{user_func_name}"), user_func_name.span());
    let mut orig_callback_params = Vec::new();
    let user_func_name = convert_to_snake_case(user_func_name);
    let mut user_func_args = Vec::new();

    for (param_name, param_type, is_option) in callback.params {
        if param_type == "String" {
            orig_callback_params.push(quote!(#param_name:*const std::ffi::c_char,));
            user_func_args.push(quote!(unsafe { std::ffi::CStr::from_ptr(#param_name).to_string_lossy().to_string() },));
        } else if param_type == "Player"
            || param_type == "Actor"
            || param_type == "Vehicle"
            || param_type == "Object"
            || param_type == "GangZone"
            || param_type == "PlayerObject"
        {
            if is_option {
                orig_callback_params.push(quote!(#param_name:*const std::ffi::c_void,));
                user_func_args.push(quote!(
                    if #param_name.is_null(){
                        None
                    } else {
                        Some(#param_type::new(#param_name))
                    },
                ));
            } else {
                orig_callback_params.push(quote!(#param_name:*const std::ffi::c_void,));
                user_func_args.push(quote!(#param_type::new(#param_name),));
            }
        } else {
            orig_callback_params.push(quote!(#param_name:#param_type,));
            user_func_args.push(quote!(#param_name,));
        }
    }

    let orig_function = if let Some(ref return_type) = callback.return_type {
        quote!(
            #[no_mangle]
            pub unsafe extern "C" fn #orig_callback_name(#(#orig_callback_params)*) -> #return_type {
                crate::runtime::Runtime.as_mut().unwrap().#user_func_name(#(#user_func_args)*)
            }
        )
    } else {
        quote!(
            #[no_mangle]
            pub unsafe extern "C" fn #orig_callback_name(#(#orig_callback_params)*) {
                crate::runtime::Runtime.as_mut().unwrap().#user_func_name(#(#user_func_args)*);
            }
        )
    };
    let code = quote! {
        #orig_function
    };

    code.into()
}
