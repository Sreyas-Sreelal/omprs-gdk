use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, token, Ident, Token, Type,
};

#[derive(Clone)]
struct CreateNative {
    name: Ident,
    params: Vec<(Ident, Ident, bool, bool)>,
    return_type: Option<(Type, bool)>,
}

impl Parse for CreateNative {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let name: Ident = input.parse()?;
        let mut params = Vec::new();
        let mut return_type = None;

        while !input.is_empty() {
            let _: Token![,] = input.parse()?;
            if input.peek(token::RArrow) {
                let _: token::RArrow = input.parse()?;
                if input.peek(token::Struct) {
                    let _: token::Struct = input.parse()?;
                    return_type = Some((input.parse()?, true));
                } else {
                    return_type = Some((input.parse()?, false));
                }
            } else {
                let param_name: Ident = input.parse()?;
                let _: Token![:] = input.parse()?;

                if input.peek(token::Mut) {
                    let _: token::Mut = input.parse()?;
                    let param_type: Ident = input.parse()?;
                    params.push((param_name, param_type, true, false));
                } else if input.peek(token::Struct) {
                    let _: token::Struct = input.parse()?;
                    let param_type: Ident = input.parse()?;
                    params.push((param_name, param_type, false, true));
                } else {
                    let param_type: Ident = input.parse()?;
                    params.push((param_name, param_type, false, false));
                }
            }
        }
        Ok(CreateNative {
            name,
            params,
            return_type,
        })
    }
}

pub fn create_native(input: TokenStream) -> TokenStream {
    let native = parse_macro_input!(input as CreateNative);
    let name = native.name;
    let orig_name = Ident::new(&format!("OMPRS_{}", name), name.span());
    let return_type = native.return_type;

    let mut param_list = Vec::new();
    let mut body = Vec::new();
    let mut address_decl_stmts = Vec::new();
    let mut mutate_stmts = Vec::new();
    let mut orig_arg_list = Vec::new();
    let mut orig_param_list = Vec::new();

    for (param_name, param_type, is_mut, is_struct) in native.params {
        if param_name.to_string().contains("_len") {
            param_list.push(quote!(#param_name: #param_type,));
            continue;
        }
        if is_mut {
            if param_type == "str" {
                param_list.push(quote!(#param_name:&mut String,));
                let addr_var_name = Ident::new(&format!("addr_{}", param_name), param_name.span());
                address_decl_stmts.push(
                    quote!(let mut #addr_var_name = crate::types::stringview::StringView::new();),
                );
                orig_arg_list.push(quote!(&mut #addr_var_name,));
                mutate_stmts.push(quote!(
                    *#param_name = #addr_var_name.get_data();
                ));
                orig_param_list.push(quote!(#param_name:*mut crate::types::stringview::StringView,))
            } else {
                param_list.push(quote!(#param_name:&mut #param_type,));
                orig_arg_list.push(quote!(#param_name,));
                orig_param_list.push(quote!(#param_name:*mut #param_type,))
            }
        } else if is_struct {
            orig_arg_list.push(quote!(#param_name.get_handle(),));
            param_list.push(quote!(#param_name: &#param_type,));
            orig_param_list.push(quote!(#param_name:*const c_void,))
        } else if param_type == "str" {
            orig_arg_list.push(quote!(crate::types::stringview::StringView::from(#param_name),));
            param_list.push(quote!(#param_name: &#param_type,));
            orig_param_list.push(quote!(#param_name: crate::types::stringview::StringView,))
        } else {
            orig_arg_list.push(quote!(#param_name,));
            param_list.push(quote!(#param_name: #param_type,));
            orig_param_list.push(quote!(#param_name:#param_type,))
        }
    }

    let decl_address_var = if let Some((ref return_type, is_struct)) = return_type {
        if is_struct {
            quote!(
                #[doc(hidden)]
                pub static mut #orig_name: Option<unsafe extern "C" fn(#(#orig_param_list)*) -> *const c_void> =
                None;
            )
        } else {
            quote!(
                #[doc(hidden)]
                pub static mut #orig_name: Option<unsafe extern "C" fn(#(#orig_param_list)*) -> #return_type> =
                None;
            )
        }
    } else {
        quote!(
            #[doc(hidden)]
            pub static mut #orig_name: Option<unsafe extern "C" fn(#(#orig_param_list)*)> =
            None;
        )
    };
    if !address_decl_stmts.is_empty() {
        body.push(quote!(
            #(#address_decl_stmts)*
        ));
    }

    body.push(quote!(
        let ret_val = unsafe { #orig_name.unwrap()(#(#orig_arg_list)*)};
    ));

    if !mutate_stmts.is_empty() {
        body.push(quote!(
            #(#mutate_stmts)*
        ));
    }
    if let Some((ref return_type, is_struct)) = return_type {
        if is_struct {
            body.push(quote!(
                if ret_val.is_null() {
                    None
                } else {
                    Some(#return_type::new(ret_val))
                }
            ))
        } else {
            body.push(quote!(ret_val));
        }
    }

    let user_func = if let Some((return_type, is_struct)) = return_type {
        if is_struct {
            quote!(
                pub fn #name(#(#param_list)*) -> Option<#return_type> {
                    #(#body)*
                }
            )
        } else {
            quote!(
                pub fn #name(#(#param_list)*) -> #return_type {
                    #(#body)*
                }
            )
        }
    } else {
        quote!(
            pub fn #name(#(#param_list)*) {
                #(#body)*
            }
        )
    };

    quote!(
        #decl_address_var
        #user_func
    )
    .into()
}
