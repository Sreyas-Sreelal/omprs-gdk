use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ExprTuple, Ident, ItemFn, Lit, LitStr, Token,
};

struct EntryArgs {
    name: Option<Lit>,
    version: Option<ExprTuple>,
}

impl Parse for EntryArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let mut name: Option<Lit> = None;
        let mut version: Option<ExprTuple> = None;
        if input.is_empty() {
            return Ok(EntryArgs { name, version });
        }
        let key: Ident = input.parse()?;

        if key == "name" {
            let _: Token![=] = input.parse()?;
            name = Some(input.parse()?);

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
                let key: Ident = input.parse()?;

                if key == "version" {
                    let _: Token![=] = input.parse()?;
                    version = Some(input.parse()?);
                }
            }
        } else if key == "version" {
            let _: Token![=] = input.parse()?;
            version = Some(input.parse()?);
            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
                let key: Ident = input.parse()?;

                if key == "name" {
                    let _: Token![=] = input.parse()?;
                    name = Some(input.parse()?);
                }
            }
        }

        Ok(EntryArgs { name, version })
    }
}

pub fn create_main(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EntryArgs);

    let component_name = if args.name.is_some() {
        args.name.unwrap()
    } else {
        Lit::from(LitStr::new("OMPRS Gamemode", Span::call_site().into()))
    };

    let component_version = if args.version.is_some() {
        let version = args.version.unwrap();
        quote!(#version)
    } else {
        quote!((0, 0, 0, 0))
    };
    let sig = parse_macro_input!(input as ItemFn);
    let function_name = sig.clone().sig.ident;
    let code = quote! {
        #sig
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
                omp::gen_uid(),
                std::ffi::CString::new(#component_name).unwrap().into_raw(),
                omp::ComponentVersion {
                    major: #component_version.0,
                    minor: #component_version.1,
                    patch: #component_version.2,
                    prerel: #component_version.3,
                },
                onLoadCB as *const std::ffi::c_void,
                onInitCB as *const std::ffi::c_void,
                onReadyCB as *const std::ffi::c_void,
                onResetCB as *const std::ffi::c_void,
                onFreeCB as *const std::ffi::c_void,
            );

            component
        }
    };
    code.into()
}
