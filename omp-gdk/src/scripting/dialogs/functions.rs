use crate::players::Player;
use omp_codegen::native;
use std::ffi::c_void;

use super::DialogStyle;

native!(ShowPlayerDialog, player: struct Player, dialog:i16, style: DialogStyle,title:str,body:str, button1:str, button2:str);
native!(GetPlayerDialogID, player: struct Player, -> i16);
native!(HidePlayerDialog, player: struct Player, -> bool);

#[doc(hidden)]
pub fn load_functions() {
    load_function!(ShowPlayerDialog);
    load_function!(GetPlayerDialogID);
    load_function!(HidePlayerDialog);
}
