use crate::players::Player;
use omp_codegen::native;
use std::ffi::c_void;

native!(Dialog_Show, player: struct Player, dialog: i32, style: i32, title: str, body: str, button1: str, button2: str, -> bool);
native!(Dialog_Hide, player: struct Player, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(Dialog_Show);
    load_function!(Dialog_Hide);
}
