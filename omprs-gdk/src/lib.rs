#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
mod helper;

mod models;
mod players;
mod utils;

use std::mem;

pub use crate::utils::*;
use helper::get_module_symbol_address;
pub use omprs_codegen::{callback, main};
use paste::paste;

pub use models::*;
pub use players::*;

pub fn init_functions() {
    load_function!(SendClientMessage);
    load_function!(Print);
    load_function!(GetPlayerName);

    // models
    load_function!(AddCharModel);
    load_function!(AddSimpleModel);
    load_function!(AddSimpleModelTimed);
    load_function!(GetPlayerCustomSkin);
    load_function!(RedirectDownload);
    load_function!(FindModelFileNameFromCRC);
    load_function!(FindTextureFileNameFromCRC);
    load_function!(IsValidCustomModel);
    load_function!(GetCustomModelPath);
}
