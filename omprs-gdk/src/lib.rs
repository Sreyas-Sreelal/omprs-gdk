#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
mod helper;

mod players;
mod utils;

use std::mem;

pub use crate::utils::*;
use helper::get_module_symbol_address;
pub use omprs_codegen::{callback, main};
use paste::paste;
pub use players::*;

pub fn init_functions() {
    load_function!(SendClientMessage);
    load_function!(Print);
}
