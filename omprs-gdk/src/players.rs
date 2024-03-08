use std::ffi::{c_char, CStr};

use omprs_codegen::native;

native!(GetPlayerName,playerid: isize, name: mut str,name_len:usize, -> isize );
native!(SendClientMessage,playerid: isize, colour: isize, message: str, -> isize);
